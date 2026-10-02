//! Real-server acceptance harness for the landed llsp cutover.
//!
//! Every test drives a real `llsp` process over stdio with framed JSON-RPC —
//! no mock, no fixture, no recorded transcript. The positive entry point
//! reads `LLSP_BINARY` (falling back to the documented install path) and
//! validates the candidate against the active baseline pin before driving
//! it; a missing, wrong-version, or non-starting server is a loud failure,
//! never a skip and never a recorded pass. The historical entry point
//! (`historical_language_id_regression_detected`) reads
//! `LLSP_HISTORICAL_BINARY` and validates exact `v0.2.0` provenance under
//! its own allowlist.
//!
//! The language identifier is sent on `didOpen` and never on `initialize`,
//! exactly as a real editor sends it. Dialect identity is observed through
//! fresh hover responses (the fence identifier plus the builtin dialect
//! name), independently of the diagnostics.

use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::time::{Duration, Instant};

const PINNED_VERSION: &str = "0.2.1";
const DEFAULT_BINARY: &str = "/home/zhuk/.local/bin/llsp";
const REQUEST_TIMEOUT: Duration = Duration::from_secs(10);

const LISPICO_CL_TEXT: &str = "(f [x])\n(car '(1 2))\n(first [1 2])\n(def probe-me 1)\n";
const LISPICO_CLOJURE_TEXT: &str = "#(1 2)\n(car '(1 2))\n(first [1 2])\n(def probe-me 1)\n";

/// A validated llsp binary staged into its own private sandbox. The env-selected
/// source goes through an allowlist (it must name the `llsp` binary itself and
/// exist as a regular file) and is copied byte-for-byte into the sandbox, so
/// every spawn below uses the literal relative path `./llsp` inside that
/// sandbox and no external string ever reaches a spawn call.
struct Candidate {
    sandbox: PathBuf,
}

impl Candidate {
    fn install(raw: &str, purpose: &str) -> Candidate {
        let source = PathBuf::from(raw);
        let file_name = source
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_default();
        assert_eq!(
            file_name, "llsp",
            "{purpose}: {raw:?} must point at the llsp binary itself, not another program"
        );
        let canonical = source.canonicalize().unwrap_or_else(|err| {
            panic!("{purpose}: {raw:?} does not resolve to an existing file: {err}")
        });
        assert!(
            canonical.is_file(),
            "{purpose}: {raw:?} must be a regular llsp binary"
        );
        let sandbox = std::env::temp_dir().join(format!(
            "llsp-acceptance-binary-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&sandbox).expect("sandbox is creatable");
        std::fs::copy(&canonical, sandbox.join("llsp"))
            .expect("the validated llsp binary is copyable into the sandbox");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(
                sandbox.join("llsp"),
                std::fs::Permissions::from_mode(0o755),
            )
            .expect("the sandboxed llsp binary takes its executable bit");
        }
        Candidate { sandbox }
    }

    fn version(&self) -> String {
        let output = Command::new("./llsp")
            .arg("--version")
            .current_dir(&self.sandbox)
            .output()
            .expect("the sandboxed llsp binary starts for --version");
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    }
}

impl Drop for Candidate {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.sandbox);
    }
}

fn pinned_candidate() -> Candidate {
    let raw = match std::env::var("LLSP_BINARY") {
        Ok(path) if !path.is_empty() => path,
        _ => DEFAULT_BINARY.to_string(),
    };
    let candidate = Candidate::install(&raw, "LLSP_BINARY (the pinned llsp candidate)");
    let version = candidate.version();
    assert!(
        version.contains(PINNED_VERSION),
        "llsp candidate ({raw}) is not the pinned build: --version printed {version:?}, \
         expected a version containing {PINNED_VERSION:?} (active baseline: \
         migrate-to-llsp-upstream-gates/gates.md, v{PINNED_VERSION}, 436bc84)"
    );
    candidate
}

struct Message {
    seq: u64,
    id: Option<u64>,
    method: Option<String>,
    params: serde_json::Value,
    result: Option<serde_json::Value>,
}

/// Arrival sequence shared with the reader thread; a stage's `mark` is the
/// counter value before its trigger, and its waits match only messages that
/// arrived after it.
static ARRIVAL_SEQ: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// All inbound messages, in arrival order; waits scan the backlog before
/// blocking, so a publication that lands early is never lost.
type Inbox = std::sync::Arc<(
    std::sync::Mutex<std::collections::VecDeque<Message>>,
    std::sync::Condvar,
)>;

struct Server {
    child: Child,
    stdin: ChildStdin,
    inbox: Inbox,
    next_id: u64,
}

/// Spawns the sandboxed candidate: the executable is the literal `./llsp`
/// inside the candidate's private sandbox, with HOME/XDG_CONFIG_HOME and the
/// working directory pointed at the isolated `root`.
fn spawn_server(root: &Path, candidate: &Candidate) -> Server {
    let mut child = Command::new("./llsp")
        .current_dir(&candidate.sandbox)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .env("HOME", root)
        .env("XDG_CONFIG_HOME", root)
        .spawn()
        .expect("real llsp server must start; resolution failure is never skipped");
    let stdout = child.stdout.take().expect("piped stdout");
    let stdin = child.stdin.take().expect("piped stdin");
    let inbox: Inbox = std::sync::Arc::new((
        std::sync::Mutex::new(std::collections::VecDeque::new()),
        std::sync::Condvar::new(),
    ));
    let reader_inbox = std::sync::Arc::clone(&inbox);
    std::thread::spawn(move || {
        let mut reader = BufReader::new(stdout);
        loop {
            let mut content_length: Option<usize> = None;
            let mut line = String::new();
            loop {
                line.clear();
                match reader.read_line(&mut line) {
                    Ok(0) => return,
                    Ok(_) => {
                        let trimmed = line.trim_end();
                        if trimmed.is_empty() {
                            break;
                        }
                        if let Some(value) = trimmed
                            .strip_prefix("Content-Length:")
                            .map(str::trim)
                            .and_then(|v| v.parse::<usize>().ok())
                        {
                            content_length = Some(value);
                        }
                    }
                    Err(_) => return,
                }
            }
            let Some(length) = content_length else { return };
            let mut body = vec![0u8; length];
            if reader.read_exact(&mut body).is_err() {
                return;
            }
            if let Ok(value) = serde_json::from_slice::<serde_json::Value>(&body) {
                let message = Message {
                    seq: ARRIVAL_SEQ.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
                    id: value.get("id").and_then(|v| v.as_u64()),
                    method: value
                        .get("method")
                        .and_then(|v| v.as_str())
                        .map(str::to_string),
                    params: value
                        .get("params")
                        .cloned()
                        .unwrap_or(serde_json::Value::Null),
                    result: value.get("result").cloned(),
                };
                let (queue, signal) = &*reader_inbox;
                queue.lock().expect("inbox lock").push_back(message);
                signal.notify_all();
            }
        }
    });
    Server {
        child,
        stdin,
        inbox,
        next_id: 1,
    }
}

impl Server {
    fn notify(&mut self, method: &str, params: serde_json::Value) {
        let message = serde_json::json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
        });
        self.frame(&message);
    }

    fn frame(&mut self, message: &serde_json::Value) {
        let body = serde_json::to_vec(message).expect("message serializes");
        write!(self.stdin, "Content-Length: {}\r\n\r\n", body.len())
            .and_then(|_| self.stdin.write_all(&body))
            .and_then(|_| self.stdin.flush())
            .expect("server stdin stays writable");
    }

    fn request(&mut self, method: &str, params: serde_json::Value) -> serde_json::Value {
        let mut raw = serde_json::json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
        });
        raw["id"] = serde_json::json!(self.next_id);
        self.frame(&raw);
        self.next_id += 1;
        let id = raw["id"].as_u64().expect("issued id");
        let deadline = Instant::now() + REQUEST_TIMEOUT;
        let (queue, signal) = &*self.inbox;
        loop {
            let remaining = deadline.saturating_duration_since(Instant::now());
            {
                let mut pending = queue.lock().expect("inbox lock");
                if let Some(position) = pending.iter().position(|m| {
                    m.method.is_none() && m.id == Some(id)
                }) {
                    let message = pending.remove(position).expect("position is valid");
                    return message.result.expect("successful responses carry a result");
                }
                if remaining.is_zero() {
                    panic!("real llsp server did not answer {method} within {REQUEST_TIMEOUT:?}");
                }
                let (guard, timeout) = signal
                    .wait_timeout(pending, remaining)
                    .expect("inbox lock");
                drop(guard);
                let _ = timeout;
            }
        }
    }

    /// The arrival counter before a trigger; waits pass it as `since`.
    fn mark(&self) -> u64 {
        ARRIVAL_SEQ.load(std::sync::atomic::Ordering::SeqCst)
    }

    /// Waits for the first publishDiagnostics for `uri` that arrived after
    /// `since` was captured, with a fresh bounded wait; early arrivals wait
    /// in the backlog and are matched here.
    fn wait_publish(&self, uri: &str, since: u64) -> serde_json::Value {
        let deadline = Instant::now() + REQUEST_TIMEOUT;
        let (queue, signal) = &*self.inbox;
        loop {
            let remaining = deadline.saturating_duration_since(Instant::now());
            {
                let mut pending = queue.lock().expect("inbox lock");
                if let Some(position) = pending.iter().position(|m| {
                    m.seq >= since
                        && m.method.as_deref() == Some("textDocument/publishDiagnostics")
                        && m.params.get("uri").and_then(|v| v.as_str()) == Some(uri)
                }) {
                    let message = pending.remove(position).expect("position is valid");
                    return message.params;
                }
                if remaining.is_zero() {
                    panic!("real llsp server published no diagnostics for {uri} within {REQUEST_TIMEOUT:?}");
                }
                let (guard, timeout) = signal
                    .wait_timeout(pending, remaining)
                    .expect("inbox lock");
                drop(guard);
                let _ = timeout;
            }
        }
    }

    fn hover(&mut self, uri: &str, line: u32, character: u32) -> (Option<String>, Option<String>) {
        let result = self.request(
            "textDocument/hover",
            serde_json::json!({
                "textDocument": {"uri": uri},
                "position": {"line": line, "character": character},
            }),
        );
        let value = result
            .get("contents")
            .and_then(|c| c.get("value"))
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();
        let fence = value
            .split_once("```")
            .and_then(|(_, rest)| rest.lines().next())
            .map(str::to_string);
        let builtin = value
            .split("builtin (")
            .nth(1)
            .and_then(|rest| rest.split(')').next())
            .map(str::to_string);
        (fence, builtin)
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        let _ = self.send_raw_shutdown();
        let _ = self.child.wait();
    }
}

impl Server {
    fn send_raw_shutdown(&mut self) -> std::io::Result<()> {
        let message = serde_json::json!({"jsonrpc": "2.0", "id": 0, "method": "shutdown", "params": null});
        let body = serde_json::to_vec(&message)?;
        self.stdin.write_all(format!("Content-Length: {}\r\n\r\n", body.len()).as_bytes())?;
        self.stdin.write_all(&body)?;
        self.stdin.write_all(
            format!(
                "Content-Length: {}\r\n\r\n",
                br#"{"jsonrpc":"2.0","method":"exit"}"#.len()
            )
            .as_bytes(),
        )?;
        self.stdin.write_all(br#"{"jsonrpc":"2.0","method":"exit"}"#)?;
        self.stdin.flush()
    }
}

struct Buffer {
    uri: String,
    identifier: &'static str,
}

fn make_buffer(root: &Path, name: &str, text: &str, identifier: &'static str) -> Buffer {
    let path = root.join(format!("{name}.lisp"));
    std::fs::write(&path, text).expect("buffer file is writable");
    Buffer {
        uri: format!("file://{}", path.display()),
        identifier,
    }
}

fn initialize(root: &Path, candidate: &Candidate) -> Server {
    let mut server = spawn_server(root, candidate);
    server.request(
        "initialize",
        serde_json::json!({
            "processId": null,
            "capabilities": {
                "textDocument": {"hover": {"contentFormat": ["markdown"]}},
                "workspace": {"didChangeConfiguration": {"dynamicRegistration": false}},
            },
            "initializationOptions": {
                "files": {"associations": {}, "default_dialect": "common-lisp"},
                "workspace": {"index": false},
            },
        }),
    );
    server.notify("initialized", serde_json::json!({}));
    server
}

fn did_open(server: &mut Server, buffer: &Buffer, text: &str) {
    server.notify(
        "textDocument/didOpen",
        serde_json::json!({
            "textDocument": {
                "uri": buffer.uri,
                "languageId": buffer.identifier,
                "version": 1,
                "text": text,
            }
        }),
    );
}

fn change_default_dialect(server: &mut Server, dialect: &str) {
    server.notify(
        "workspace/didChangeConfiguration",
        serde_json::json!({
            "settings": {
                "llsp": {
                    "files": {"associations": {}, "default_dialect": dialect},
                    "workspace": {"index": false},
                }
            }
        }),
    );
}

fn invalid_syntax_count(publish: &serde_json::Value) -> usize {
    publish["diagnostics"]
        .as_array()
        .map(|items| {
            items
                .iter()
                .filter(|d| d.get("code").and_then(|c| c.as_str()) == Some("invalid-syntax"))
                .count()
        })
        .unwrap_or(0)
}

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "llsp-acceptance-{}-{}",
        name,
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn dialect_identity_and_diagnostics_survive_an_accepted_settings_change() {
    let root = scratch("retention");
    let candidate = pinned_candidate();
    let mut server = initialize(&root, &candidate);

    let lispico_cl = make_buffer(&root, "retention-cl", LISPICO_CL_TEXT, "lispico-cl");
    let lisp = make_buffer(&root, "retention-lisp", LISPICO_CL_TEXT, "lisp");
    let clojure = make_buffer(&root, "retention-clojure", LISPICO_CLOJURE_TEXT, "lispico-clojure");

    let mut opened = Vec::new();
    for (buffer, text) in [
        (&lispico_cl, LISPICO_CL_TEXT),
        (&lisp, LISPICO_CL_TEXT),
        (&clojure, LISPICO_CLOJURE_TEXT),
    ] {
        let since = server.mark();
        did_open(&mut server, buffer, text);
        let publish = server.wait_publish(&buffer.uri, since);
        opened.push((buffer.uri.clone(), publish));
    }

    // didOpen published reader-invalid diagnostics for lispico-cl, and the
    // identifier — never sent on initialize — resolved each dialect.
    let cl_open = &opened[0].1;
    assert!(
        invalid_syntax_count(cl_open) >= 2,
        "lispico-cl buffer must publish reader-invalid diagnostics at didOpen: {cl_open}"
    );

    let changed_at = server.mark();
    change_default_dialect(&mut server, "lispico-clojure");

    // Both required signals, independently: the lispico-cl diagnostics keep
    // publishing after the accepted settings change, and every buffer keeps
    // its dialect per a fresh hover.
    let mut kept = Vec::new();
    for (uri, _) in &opened {
        let publish = server.wait_publish(uri, changed_at);
        kept.push((uri.clone(), publish));
    }
    assert!(
        invalid_syntax_count(&kept[0].1) >= 2,
        "reader-invalid diagnostics must survive the accepted settings change: {kept:?}"
    );

    let expected = [
        (&lispico_cl, ("lispico-cl", "lispico-cl")),
        (&lisp, ("lisp", "common-lisp")),
        (&clojure, ("lispico-clojure", "lispico-clojure")),
    ];
    for (buffer, expected_pair) in expected {
        let (fence, builtin) = server.hover(&buffer.uri, 2, 1);
        assert_eq!(
            Some(expected_pair.0),
            fence.as_deref(),
            "{}: dialect fence identifier changed across the settings change",
            buffer.identifier
        );
        assert_eq!(
            Some(expected_pair.1),
            builtin.as_deref(),
            "{}: dialect name changed across the settings change",
            buffer.identifier
        );
    }
}

#[test]
fn unsaved_edits_clear_diagnostics_without_any_save() {
    let root = scratch("unsaved");
    let candidate = pinned_candidate();
    let mut server = initialize(&root, &candidate);
    let buffer = make_buffer(&root, "unsaved-cl", LISPICO_CL_TEXT, "lispico-cl");

    let since = server.mark();
    did_open(&mut server, &buffer, LISPICO_CL_TEXT);
    let broken = server.wait_publish(&buffer.uri, since);
    assert!(
        invalid_syntax_count(&broken) >= 2,
        "the buffer must start with reader-invalid diagnostics: {broken}"
    );

    // Repair the buffer with an edit that is valid for lispico-cl; nothing
    // is saved at any point.
    let repaired = "(def fixed 1)\n(car '(1 2))\n(first (list 1 2))\n(def probe-me 1)\n";
    let edited_at = server.mark();
    server.notify(
        "textDocument/didChange",
        serde_json::json!({
            "textDocument": {"uri": buffer.uri, "version": 2},
            "contentChanges": [{"text": repaired}],
        }),
    );
    let cleared = server.wait_publish(&buffer.uri, edited_at);
    assert_eq!(
        invalid_syntax_count(&cleared),
        0,
        "unsaved repairs must clear the diagnostics without a save: {cleared}"
    );
}

#[test]
fn save_reload_and_close_reopen_preserve_the_dialect() {
    let root = scratch("lifecycle");
    let candidate = pinned_candidate();
    let mut server = initialize(&root, &candidate);
    let buffer = make_buffer(&root, "lifecycle-cl", LISPICO_CL_TEXT, "lispico-cl");

    let since = server.mark();
    did_open(&mut server, &buffer, LISPICO_CL_TEXT);
    let _ = server.wait_publish(&buffer.uri, since);

    // Save, then force the reload path llsp runs on any settings change.
    server.notify(
        "textDocument/didSave",
        serde_json::json!({"textDocument": {"uri": buffer.uri}}),
    );
    let reloaded_at = server.mark();
    change_default_dialect(&mut server, "lispico-cl");
    let after_reload = server.wait_publish(&buffer.uri, reloaded_at);
    assert!(
        invalid_syntax_count(&after_reload) >= 2,
        "reader-invalid diagnostics must survive save and reload: {after_reload}"
    );

    // Close and reopen: the dialect is recovered from the identifier.
    server.notify(
        "textDocument/didClose",
        serde_json::json!({"textDocument": {"uri": buffer.uri}}),
    );
    let reopened_at = server.mark();
    did_open(&mut server, &buffer, LISPICO_CL_TEXT);
    let _ = server.wait_publish(&buffer.uri, reopened_at);
    let (fence, builtin) = server.hover(&buffer.uri, 2, 1);
    assert_eq!(Some("lispico-cl"), fence.as_deref());
    assert_eq!(Some("lispico-cl"), builtin.as_deref());
}

#[test]
fn a_restarted_server_restores_every_open_buffer() {
    let root = scratch("restart");
    let candidate = pinned_candidate();
    let lispico_cl = make_buffer(&root, "restart-cl", LISPICO_CL_TEXT, "lispico-cl");
    let clojure = make_buffer(&root, "restart-clojure", LISPICO_CLOJURE_TEXT, "lispico-clojure");

    let mut server = initialize(&root, &candidate);
    for (buffer, text) in [(&lispico_cl, LISPICO_CL_TEXT), (&clojure, LISPICO_CLOJURE_TEXT)] {
        let since = server.mark();
        did_open(&mut server, buffer, text);
        let _ = server.wait_publish(&buffer.uri, since);
    }
    // The first server process ends here; a restarted editor session
    // re-initializes and re-opens every buffer.
    let _ = server.send_raw_shutdown();

    let mut restarted = initialize(&root, &candidate);
    for (buffer, text) in [(&lispico_cl, LISPICO_CL_TEXT), (&clojure, LISPICO_CLOJURE_TEXT)] {
        let since = restarted.mark();
        did_open(&mut restarted, buffer, text);
        let publish = restarted.wait_publish(&buffer.uri, since);
        if buffer.identifier == "lispico-cl" {
            assert!(
                invalid_syntax_count(&publish) >= 2,
                "restarted server must republish reader-invalid diagnostics: {publish}"
            );
        }
        let (fence, builtin) = restarted.hover(&buffer.uri, 2, 1);
        let expected = if buffer.identifier == "lispico-cl" {
            ("lispico-cl", "lispico-cl")
        } else {
            ("lispico-clojure", "lispico-clojure")
        };
        assert_eq!(Some(expected.0), fence.as_deref(), "restarted dialect fence");
        assert_eq!(Some(expected.1), builtin.as_deref(), "restarted dialect name");
    }
}

#[test]
fn two_workspaces_keep_isolated_configurations() {
    let root_a = scratch("workspace-a");
    let root_b = scratch("workspace-b");
    let candidate = pinned_candidate();
    let mut server_a = initialize(&root_a, &candidate);
    let mut server_b = initialize(&root_b, &candidate);

    let buffer_a = make_buffer(&root_a, "isolated-a", LISPICO_CL_TEXT, "lispico-cl");
    let buffer_b = make_buffer(&root_b, "isolated-b", LISPICO_CL_TEXT, "lispico-cl");
    for (server, buffer) in [(&mut server_a, &buffer_a), (&mut server_b, &buffer_b)] {
        let since = server.mark();
        did_open(server, buffer, LISPICO_CL_TEXT);
        let _ = server.wait_publish(&buffer.uri, since);
    }

    // One workspace switches its default dialect; the other does not.
    let changed_at = server_a.mark();
    change_default_dialect(&mut server_a, "lispico-clojure");
    let _ = server_a.wait_publish(&buffer_a.uri, changed_at);

    // Workspace A's change must not reach workspace B, and each buffer keeps
    // its own dialect from its own identifier.
    let (fence_a, builtin_a) = server_a.hover(&buffer_a.uri, 2, 1);
    let (fence_b, builtin_b) = server_b.hover(&buffer_b.uri, 2, 1);
    for (fence, builtin, workspace) in [(fence_a, builtin_a, "A"), (fence_b, builtin_b, "B")] {
        assert_eq!(Some("lispico-cl"), fence.as_deref(), "workspace {workspace} dialect fence");
        assert_eq!(Some("lispico-cl"), builtin.as_deref(), "workspace {workspace} dialect name");
    }
}

/// Detector proof for the historical defect: the same retention assertion
/// that passes on the pinned candidate must FAIL on llsp v0.2.0 (88e3e72).
/// Explicitly ignored in the ordinary run — it needs the real historical
/// binary in LLSP_HISTORICAL_BINARY and supplies no current acceptance pass;
/// a missing or wrong historical binary fails this invocation, it never
/// skips.
#[test]
#[ignore = "opt-in detector proof: requires LLSP_HISTORICAL_BINARY pointing at the real llsp v0.2.0 (88e3e72) build; supplies no current acceptance pass"]
fn historical_language_id_regression_detected() {
    let historical = std::env::var("LLSP_HISTORICAL_BINARY").unwrap_or_default();
    assert!(
        !historical.is_empty(),
        "set LLSP_HISTORICAL_BINARY to the real llsp v0.2.0 (88e3e72) binary; \
         the detector proof never runs against the current pin and never skips"
    );
    let candidate = Candidate::install(
        &historical,
        "LLSP_HISTORICAL_BINARY (the historical detector build)",
    );
    let version = candidate.version();
    assert!(
        version.contains("0.2.0") && !version.contains("0.2.1"),
        "LLSP_HISTORICAL_BINARY {historical} is not the v0.2.0 build: --version printed {version:?}"
    );

    let root = scratch("historical");
    let buffer = make_buffer(&root, "historical-cl", LISPICO_CL_TEXT, "lispico-cl");
    let mut server = spawn_server(&root, &candidate);
    server.request(
        "initialize",
        serde_json::json!({
            "processId": null,
            "capabilities": {},
            "initializationOptions": {
                "files": {"associations": {}, "default_dialect": "common-lisp"},
                "workspace": {"index": false},
            },
        }),
    );
    server.notify("initialized", serde_json::json!({}));
    let opened_at = server.mark();
    did_open(&mut server, &buffer, LISPICO_CL_TEXT);
    let broken = server.wait_publish(&buffer.uri, opened_at);
    assert!(
        invalid_syntax_count(&broken) >= 2,
        "v0.2.0 must publish the reader-invalid diagnostics at didOpen for the detector to mean anything"
    );
    let changed_at = server.mark();
    change_default_dialect(&mut server, "lispico-clojure");
    let after = server.wait_publish(&buffer.uri, changed_at);

    // The regression: v0.2.0 loses the dialect and its reader-invalid
    // diagnostics on the accepted settings change. The detector proof
    // succeeds only when it observes exactly that failure.
    let lost = invalid_syntax_count(&after) == 0;
    let (fence, builtin) = server.hover(&buffer.uri, 2, 1);
    let dialect_lost =
        fence.as_deref() != Some("lispico-cl") || builtin.as_deref() != Some("lispico-cl");
    assert!(
        lost || dialect_lost,
        "the historical binary unexpectedly retained the dialect; \
         the v0.2.0 regression was not reproduced and this detector proof fails \
         (diagnostics after change: {after}, hover: {fence:?}/{builtin:?})"
    );
}
