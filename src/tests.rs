use super::{
    binary_file_name, command_with, dispatch_language_server, install_verified_release,
    llsp_asset_name, member_dir, platform_key, resolve_llsp, verified_cache_binary, BinarySource,
    COMPLETION_STATE_FILE, LLSP_RELEASE_TAG,
};
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use zed_extension_api::{self as zed, GithubRelease, GithubReleaseAsset};
const LINUX: (zed::Os, zed::Architecture) = (zed::Os::Linux, zed::Architecture::X8664);
const LINUX_ASSET: &str = "llsp-x86_64-unknown-linux-musl.tar.gz";
const WINDOWS: (zed::Os, zed::Architecture) = (zed::Os::Windows, zed::Architecture::X8664);
const WINDOWS_ASSET: &str = "llsp-x86_64-pc-windows-msvc.zip";

const BINARY_CONTENTS: &[u8] = b"#!/bin/sh\nexit 0\n";

/// A named scratch directory that removes itself when the test ends; it
/// derefs to the directory path so call sites read as before.
struct Scratch(tempfile::TempDir);

impl std::ops::Deref for Scratch {
    type Target = Path;

    fn deref(&self) -> &Path {
        self.0.path()
    }
}

fn scratch_dir(name: &str) -> Scratch {
    Scratch(
        tempfile::Builder::new()
            .prefix(&format!("llsp-cutover-{name}-"))
            .tempdir()
            .unwrap(),
    )
}

fn tar_gz_fixture(member_dir: &str, binary: &str, contents: &[u8]) -> Vec<u8> {
    let encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    let mut builder = tar::Builder::new(encoder);
    let mut header = tar::Header::new_gnu();
    header.set_size(contents.len() as u64);
    header.set_mode(0o755);
    header.set_cksum();
    builder
        .append_data(&mut header, format!("{member_dir}/{binary}"), contents)
        .unwrap();
    builder.into_inner().unwrap().finish().unwrap()
}

fn zip_fixture(member_dir: &str, binary: &str, contents: &[u8]) -> Vec<u8> {
    let mut writer = zip::ZipWriter::new(std::io::Cursor::new(Vec::new()));
    writer
        .add_directory(member_dir, zip::write::SimpleFileOptions::default())
        .unwrap();
    writer
        .start_file(
            format!("{member_dir}/{binary}"),
            zip::write::SimpleFileOptions::default(),
        )
        .unwrap();
    writer.write_all(contents).unwrap();
    writer.finish().unwrap().into_inner()
}

fn sha256_hex(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let digest = Sha256::digest(bytes);
    digest.iter().map(|b| format!("{b:02x}")).collect()
}

/// A release source whose release and download behavior the test scripts.
/// Every call is recorded so a test can assert a step was never reached.
struct FakeSource {
    release: Result<Vec<(String, String)>, String>,
    files: HashMap<String, Result<Vec<u8>, String>>,
    calls: RefCell<Vec<String>>,
}

impl FakeSource {
    fn serving(fixture: Vec<u8>, asset_name: &str) -> Self {
        let sums = format!("{}  {asset_name}\n", sha256_hex(&fixture));
        Self {
            release: Ok(vec![
                (
                    asset_name.to_string(),
                    format!("https://fixture/{asset_name}"),
                ),
                (
                    "SHA256SUMS".to_string(),
                    "https://fixture/SHA256SUMS".to_string(),
                ),
            ]),
            files: HashMap::from([
                (format!("https://fixture/{asset_name}"), Ok(fixture)),
                (
                    "https://fixture/SHA256SUMS".to_string(),
                    Ok(sums.into_bytes()),
                ),
            ]),
            calls: RefCell::new(Vec::new()),
        }
    }

    fn offline() -> Self {
        Self {
            release: Err("unreachable".to_string()),
            files: HashMap::new(),
            calls: RefCell::new(Vec::new()),
        }
    }

    fn calls(&self) -> Vec<String> {
        self.calls.borrow().clone()
    }
}

impl super::ReleaseSource for FakeSource {
    fn release_by_tag(&self, _repo: &str, _tag: &str) -> Result<GithubRelease, String> {
        self.calls.borrow_mut().push("release_by_tag".to_string());
        match &self.release {
            Ok(assets) => Ok(GithubRelease {
                version: LLSP_RELEASE_TAG.to_string(),
                assets: assets
                    .iter()
                    .map(|(name, url)| GithubReleaseAsset {
                        name: name.clone(),
                        download_url: url.clone(),
                    })
                    .collect(),
            }),
            Err(reason) => Err(reason.clone()),
        }
    }

    fn download(&self, url: &str, path: &Path) -> Result<(), String> {
        self.calls.borrow_mut().push(format!("download {url}"));
        match self.files.get(url) {
            Some(Ok(bytes)) => {
                std::fs::write(path, bytes).unwrap();
                Ok(())
            }
            Some(Err(reason)) => Err(reason.clone()),
            None => Err(format!("no fixture for {url}")),
        }
    }

    fn make_executable(&self, path: &Path) -> Result<(), String> {
        self.calls
            .borrow_mut()
            .push(format!("make_executable {}", path.display()));
        Ok(())
    }
}

fn nothing_on_path(name: &str) -> Option<String> {
    let _ = name;
    None
}

fn make_cache_entry(root: &Path, asset_name: &str, executable: bool) -> PathBuf {
    let entry = root.join(format!("llsp-{LLSP_RELEASE_TAG}"));
    std::fs::create_dir_all(&entry).unwrap();
    let binary = entry.join(binary_file_name(asset_name));
    std::fs::write(&binary, BINARY_CONTENTS).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = if executable { 0o755 } else { 0o644 };
        std::fs::set_permissions(&binary, std::fs::Permissions::from_mode(mode)).unwrap();
    }
    let _ = executable;
    std::fs::write(
        entry.join(COMPLETION_STATE_FILE),
        zed::serde_json::json!({
            "version": LLSP_RELEASE_TAG,
            "platform": platform_key(LINUX),
            "asset": asset_name,
            "digest": "0".repeat(64),
            "binary_sha256": sha256_hex(BINARY_CONTENTS),
        })
        .to_string(),
    )
    .unwrap();
    entry
}

fn assert_no_usable_entry(root: &Path) {
    let entry = root.join(format!("llsp-{LLSP_RELEASE_TAG}"));
    assert!(
        !entry.join(COMPLETION_STATE_FILE).exists(),
        "a failed attempt must not leave a completion state: {}",
        entry.display()
    );
    assert!(
        verified_cache_binary(root, LINUX).is_none(),
        "a failed attempt must not leave a startable entry"
    );
}

#[test]
fn dispatch_accepts_only_llsp() {
    assert_eq!(dispatch_language_server("llsp"), Ok(()));
}

#[test]
fn dispatch_rejects_both_previous_server_ids() {
    for id in ["sextant", "lispico", "lispico-lsp"] {
        let error = dispatch_language_server(id).unwrap_err();
        assert!(
            error.contains(id),
            "error must name the rejected id: {error}"
        );
    }
}

#[test]
fn unknown_server_ids_name_the_one_registered_server() {
    for id in ["", "Lispico", "llsp2", "sextant "] {
        let error = dispatch_language_server(id).unwrap_err();
        assert!(
            error.contains("'llsp'"),
            "unknown-id error must name the registered server: {error}"
        );
    }
}

#[test]
fn a_configured_binary_wins_without_a_path_lookup_or_download() {
    let root = scratch_dir("configured-wins");
    let source = FakeSource::offline();
    let (binary, source_kind) = resolve_llsp(
        &root,
        Some("/opt/llsp/bin/llsp".to_string()),
        &|name| {
            let _ = name;
            panic!("PATH lookup must not happen when a binary is configured");
        },
        &source,
        LINUX,
    )
    .unwrap();

    assert_eq!(binary, PathBuf::from("/opt/llsp/bin/llsp"));
    assert_eq!(source_kind, BinarySource::Configured);
    assert!(source.calls().is_empty(), "no release lookup may occur");
}

#[test]
fn a_path_hit_wins_without_a_release_lookup() {
    let root = scratch_dir("path-hit");
    let source = FakeSource::offline();
    let (binary, source_kind) = resolve_llsp(
        &root,
        None,
        &|name| {
            assert_eq!(name, "llsp");
            Some("/usr/local/bin/llsp".to_string())
        },
        &source,
        LINUX,
    )
    .unwrap();

    assert_eq!(binary, PathBuf::from("/usr/local/bin/llsp"));
    assert_eq!(source_kind, BinarySource::PathHit);
    assert!(source.calls().is_empty(), "no release lookup may occur");
}

#[test]
fn a_verified_cache_entry_is_reused_without_any_release_lookup() {
    let root = scratch_dir("offline-reuse");
    make_cache_entry(&root, LINUX_ASSET, true);
    let source = FakeSource::offline();

    let (binary, source_kind) =
        resolve_llsp(&root, None, &nothing_on_path, &source, LINUX).unwrap();

    assert_eq!(source_kind, BinarySource::VerifiedCache);
    assert_eq!(
        binary,
        root.join(format!("llsp-{LLSP_RELEASE_TAG}")).join("llsp")
    );
    assert!(
        source.calls().is_empty(),
        "offline reuse must make zero network calls: {:?}",
        source.calls()
    );
}

#[test]
fn a_first_installation_downloads_verifies_and_installs() {
    let root = scratch_dir("first-install");
    let fixture = tar_gz_fixture(member_dir(LINUX_ASSET), "llsp", BINARY_CONTENTS);
    let source = FakeSource::serving(fixture, LINUX_ASSET);

    let (binary, source_kind) =
        resolve_llsp(&root, None, &nothing_on_path, &source, LINUX).unwrap();

    assert_eq!(source_kind, BinarySource::VerifiedDownload);
    assert_eq!(std::fs::read(&binary).unwrap(), BINARY_CONTENTS);
    let entry = root.join(format!("llsp-{LLSP_RELEASE_TAG}"));
    let state = std::fs::read_to_string(entry.join(COMPLETION_STATE_FILE)).unwrap();
    let state: zed::serde_json::Value = zed::serde_json::from_str(&state).unwrap();
    assert_eq!(
        state.get("version").and_then(|v| v.as_str()),
        Some(LLSP_RELEASE_TAG),
        "completion state must record the installed version: {state}"
    );
    assert_eq!(
        state.get("digest").and_then(|v| v.as_str()).map(str::len),
        Some(64),
        "completion state must record the verified archive digest: {state}"
    );
    assert_eq!(
        state.get("binary_sha256").and_then(|v| v.as_str()),
        Some(sha256_hex(BINARY_CONTENTS).as_str()),
        "completion state must record the installed binary's digest: {state}"
    );
    assert!(
        source
            .calls()
            .iter()
            .any(|c| c.starts_with("make_executable")),
        "the installed binary must be marked executable"
    );
    assert!(!entry.join("payload").exists(), "staging must be removed");
}

#[test]
fn a_digest_mismatch_installs_nothing() {
    let root = scratch_dir("digest-mismatch");
    let fixture = tar_gz_fixture(member_dir(LINUX_ASSET), "llsp", BINARY_CONTENTS);
    let mut source = FakeSource::serving(fixture, LINUX_ASSET);
    // Publish a digest that cannot match the archive actually served.
    source.files.insert(
        "https://fixture/SHA256SUMS".to_string(),
        Ok(format!("{}  {LINUX_ASSET}\n", "a".repeat(64)).into_bytes()),
    );

    let error = resolve_llsp(&root, None, &nothing_on_path, &source, LINUX).unwrap_err();

    assert!(error.contains("digest mismatch"), "error: {error}");
    assert_no_usable_entry(&root);
}

#[test]
fn an_extraction_failure_installs_nothing() {
    let root = scratch_dir("extraction-failure");
    // Valid bytes for the digest step that are not an archive at all.
    let not_an_archive = b"definitely not a tar.gz".to_vec();
    let source = FakeSource::serving(not_an_archive, LINUX_ASSET);

    let error = resolve_llsp(&root, None, &nothing_on_path, &source, LINUX).unwrap_err();

    assert!(
        error.to_lowercase().contains("archive") || error.to_lowercase().contains("extract"),
        "error must name the extraction failure: {error}"
    );
    assert_no_usable_entry(&root);
}

/// A raw ustar header block with an arbitrary member name, for hostile
/// fixtures the tar builder itself refuses to produce.
fn raw_tar_header(name: &str, size: usize) -> Vec<u8> {
    let mut block = [0u8; 512];
    block[..name.len()].copy_from_slice(name.as_bytes());
    block[100..108].copy_from_slice(b"0000644\0");
    block[108..116].copy_from_slice(b"0000000\0");
    block[116..124].copy_from_slice(b"0000000\0");
    let size_octal = format!("{size:011o}\0");
    block[124..124 + size_octal.len()].copy_from_slice(size_octal.as_bytes());
    block[148..156].copy_from_slice(b"        ");
    block[156] = b'0';
    block[257..262].copy_from_slice(b"ustar");
    block[263..265].copy_from_slice(b"00");
    let checksum = block.iter().map(|byte| u64::from(*byte)).sum::<u64>();
    let checksum_octal = format!("{checksum:06o}\0 ");
    block[148..156].copy_from_slice(checksum_octal.as_bytes());
    block.to_vec()
}

fn hostile_tar_fixture(member: &str, contents: &[u8]) -> Vec<u8> {
    let encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    let mut tar_bytes = raw_tar_header(member, contents.len());
    tar_bytes.extend_from_slice(contents);
    let padding = (512 - contents.len() % 512) % 512;
    tar_bytes.extend(std::iter::repeat_n(0u8, padding));
    tar_bytes.extend_from_slice(&[0u8; 1024]);
    let mut encoder = encoder;
    encoder.write_all(&tar_bytes).unwrap();
    encoder.finish().unwrap()
}

/// The one location outside the cache that a traversal attempt would have to
/// create for this process; nothing may ever appear there.
fn escape_probe() -> PathBuf {
    std::env::temp_dir().join(format!("zed-lisp-escape-probe-{}", std::process::id()))
}

fn assert_rejected_member(
    root: &Path,
    fixture: Vec<u8>,
    platform: (zed::Os, zed::Architecture),
    asset: &str,
) {
    let outside = escape_probe();
    let _ = std::fs::remove_dir_all(&outside);
    let source = FakeSource::serving(fixture, asset);

    let error = resolve_llsp(root, None, &nothing_on_path, &source, platform).unwrap_err();

    assert!(
        error.contains("unsafe archive member"),
        "a traversal member must be rejected by name: {error}"
    );
    assert_no_usable_entry(root);
    assert!(
        !outside.exists(),
        "no member may extract outside the payload directory"
    );
}

#[test]
fn a_rooted_tar_member_installs_nothing() {
    let root = scratch_dir("rooted-tar-member");
    // A rooted member would make `payload_dir.join(member)` replace the
    // payload root entirely and write to the absolute path.
    assert_rejected_member(
        &root,
        hostile_tar_fixture("/tmp/escape/llsp", BINARY_CONTENTS),
        LINUX,
        LINUX_ASSET,
    );
}

#[test]
fn a_parent_traversing_tar_member_installs_nothing() {
    let root = scratch_dir("parent-tar-member");
    assert_rejected_member(
        &root,
        hostile_tar_fixture("../escape/llsp", BINARY_CONTENTS),
        LINUX,
        LINUX_ASSET,
    );
}

#[test]
fn a_rooted_zip_member_installs_nothing() {
    let root = scratch_dir("rooted-zip-member");
    assert_rejected_member(
        &root,
        zip_fixture("/tmp/escape", "llsp.exe", BINARY_CONTENTS),
        WINDOWS,
        WINDOWS_ASSET,
    );
}

#[test]
fn a_parent_traversing_zip_member_installs_nothing() {
    let root = scratch_dir("parent-zip-member");
    assert_rejected_member(
        &root,
        zip_fixture("../escape", "llsp.exe", BINARY_CONTENTS),
        WINDOWS,
        WINDOWS_ASSET,
    );
}

#[test]
fn an_interrupted_download_leaves_nothing_a_later_attempt_could_start() {
    let root = scratch_dir("interrupted-download");
    let mut source = FakeSource::offline();
    source.release = Ok(vec![
        (
            LINUX_ASSET.to_string(),
            "https://fixture/archive".to_string(),
        ),
        (
            "SHA256SUMS".to_string(),
            "https://fixture/SHA256SUMS".to_string(),
        ),
    ]);
    source.files.insert(
        "https://fixture/SHA256SUMS".to_string(),
        Ok(format!("{}  {LINUX_ASSET}\n", "0".repeat(64)).into_bytes()),
    );
    source.files.insert(
        "https://fixture/archive".to_string(),
        Err("connection reset mid-download".to_string()),
    );

    let error = resolve_llsp(&root, None, &nothing_on_path, &source, LINUX).unwrap_err();

    assert!(error.contains("connection reset"), "error: {error}");
    assert_no_usable_entry(&root);
}

#[cfg(unix)]
#[test]
fn a_non_executable_cache_entry_is_reinstalled_online() {
    let root = scratch_dir("non-exec-online");
    make_cache_entry(&root, LINUX_ASSET, false);
    let fixture = tar_gz_fixture(member_dir(LINUX_ASSET), "llsp", BINARY_CONTENTS);
    let source = FakeSource::serving(fixture, LINUX_ASSET);

    let (_binary, source_kind) =
        resolve_llsp(&root, None, &nothing_on_path, &source, LINUX).unwrap();

    assert_eq!(
        source_kind,
        BinarySource::VerifiedDownload,
        "an unusable entry must re-resolve to a fresh verified download"
    );
    assert!(
        source
            .calls()
            .iter()
            .any(|call| call.starts_with("release_by_tag")),
        "re-resolution must reach the release"
    );
}

#[cfg(unix)]
#[test]
fn a_non_executable_cache_entry_is_an_error_offline() {
    let root = scratch_dir("non-exec-offline");
    make_cache_entry(&root, LINUX_ASSET, false);
    let source = FakeSource::offline();

    let error = resolve_llsp(&root, None, &nothing_on_path, &source, LINUX).unwrap_err();

    assert!(
        error.contains("Remedies"),
        "offline with an unusable entry must return the documented error: {error}"
    );
    assert!(
        source
            .calls()
            .iter()
            .any(|call| call.starts_with("release_by_tag")),
        "the unreachable release must have been attempted"
    );
}

#[test]
fn a_cache_binary_that_no_longer_matches_its_recorded_digest_is_reinstalled() {
    let root = scratch_dir("tampered-cache");
    let entry = make_cache_entry(&root, LINUX_ASSET, true);
    std::fs::write(entry.join(binary_file_name(LINUX_ASSET)), b"tampered").unwrap();
    let fixture = tar_gz_fixture(member_dir(LINUX_ASSET), "llsp", BINARY_CONTENTS);
    let source = FakeSource::serving(fixture, LINUX_ASSET);

    let (_binary, source_kind) =
        resolve_llsp(&root, None, &nothing_on_path, &source, LINUX).unwrap();

    assert_eq!(
        source_kind,
        BinarySource::VerifiedDownload,
        "a cache binary that fails its recorded digest must never be started"
    );
}

#[test]
fn a_cache_binary_that_fails_its_digest_is_an_error_offline() {
    let root = scratch_dir("tampered-offline");
    let entry = make_cache_entry(&root, LINUX_ASSET, true);
    std::fs::write(entry.join(binary_file_name(LINUX_ASSET)), b"tampered").unwrap();
    let source = FakeSource::offline();

    let error = resolve_llsp(&root, None, &nothing_on_path, &source, LINUX).unwrap_err();

    assert!(
        error.contains("Remedies"),
        "offline with a digested-mismatch entry must return the documented error: {error}"
    );
    assert!(
        verified_cache_binary(&root, LINUX).is_none(),
        "a digested-mismatch entry must not read as verified"
    );
}

#[test]
fn a_cache_entry_recorded_for_a_foreign_asset_is_not_reused() {
    let root = scratch_dir("foreign-asset");
    // The state names an asset this platform never published; the binary
    // beside it matches nothing the resolver may start.
    let entry = make_cache_entry(&root, LINUX_ASSET, true);
    let state_path = entry.join(COMPLETION_STATE_FILE);
    let state = zed::serde_json::json!({
        "version": LLSP_RELEASE_TAG,
        "platform": platform_key(LINUX),
        "asset": WINDOWS_ASSET,
        "digest": "0".repeat(64),
        "binary_sha256": sha256_hex(BINARY_CONTENTS),
    });
    std::fs::write(state_path, state.to_string()).unwrap();
    let source = FakeSource::serving(
        tar_gz_fixture(member_dir(LINUX_ASSET), "llsp", BINARY_CONTENTS),
        LINUX_ASSET,
    );

    let (_binary, source_kind) =
        resolve_llsp(&root, None, &nothing_on_path, &source, LINUX).unwrap();

    assert_eq!(
        source_kind,
        BinarySource::VerifiedDownload,
        "a cache entry recorded for another platform's asset must not be reused"
    );
}

#[test]
fn installing_a_version_prunes_older_version_directories() {
    let root = scratch_dir("pruning");
    let stale = root.join("llsp-v0.1.0");
    std::fs::create_dir_all(&stale).unwrap();
    std::fs::write(stale.join("llsp"), b"old").unwrap();
    let unrelated = root.join("unrelated");
    std::fs::create_dir_all(&unrelated).unwrap();

    let fixture = tar_gz_fixture(member_dir(LINUX_ASSET), "llsp", BINARY_CONTENTS);
    let source = FakeSource::serving(fixture, LINUX_ASSET);
    install_verified_release(&root, &source, LINUX).unwrap();

    assert!(!stale.exists(), "stale version directory must be pruned");
    assert!(unrelated.exists(), "unrelated entries must be untouched");
    assert!(
        root.join(format!("llsp-{LLSP_RELEASE_TAG}"))
            .join("llsp")
            .exists(),
        "the installed version must remain"
    );
}

#[test]
fn an_unsupported_platform_names_the_supported_list() {
    let root = scratch_dir("unsupported-platform");
    let source = FakeSource::serving(
        tar_gz_fixture(member_dir(LINUX_ASSET), "llsp", BINARY_CONTENTS),
        LINUX_ASSET,
    );

    let error =
        resolve_llsp(&root, None, &nothing_on_path, &source, WINDOWS_UNSUPPORTED).unwrap_err();

    for platform in [
        "Linux x86_64",
        "Linux aarch64",
        "macOS x86_64",
        "macOS aarch64",
        "Windows x86_64",
    ] {
        assert!(
            error.contains(platform),
            "error must name {platform}: {error}"
        );
    }
    assert!(
        !error.contains(WINDOWS_UNSUPPORTED_ASSET),
        "another platform's archive must not be substituted: {error}"
    );
    assert_no_usable_entry(&root);
}

const WINDOWS_UNSUPPORTED: (zed::Os, zed::Architecture) =
    (zed::Os::Windows, zed::Architecture::Aarch64);
const WINDOWS_UNSUPPORTED_ASSET: &str = "llsp-aarch64-pc-windows-msvc.zip";

#[test]
fn the_windows_x86_64_archive_installs_from_a_zip() {
    let root = scratch_dir("windows-zip");
    let fixture = zip_fixture(member_dir(WINDOWS_ASSET), "llsp.exe", BINARY_CONTENTS);
    let source = FakeSource::serving(fixture, WINDOWS_ASSET);

    let binary = install_verified_release(&root, &source, WINDOWS)
        .map_err(|e| panic!("{e}"))
        .unwrap();

    assert_eq!(std::fs::read(&binary).unwrap(), BINARY_CONTENTS);
    assert_eq!(binary.file_name().unwrap(), "llsp.exe");
}

#[test]
fn a_cache_entry_for_a_different_platform_is_not_reused() {
    let root = scratch_dir("platform-mismatch");
    let entry = make_cache_entry(&root, LINUX_ASSET, true);
    // Record the entry for a platform the resolver is not asked for.
    let state_path = entry.join(COMPLETION_STATE_FILE);
    let state = zed::serde_json::json!({
        "version": LLSP_RELEASE_TAG,
        "platform": platform_key(WINDOWS),
        "asset": WINDOWS_ASSET,
        "digest": "0".repeat(64),
    });
    std::fs::write(state_path, state.to_string()).unwrap();
    let source = FakeSource::serving(
        tar_gz_fixture(member_dir(LINUX_ASSET), "llsp", BINARY_CONTENTS),
        LINUX_ASSET,
    );

    let (_binary, source_kind) =
        resolve_llsp(&root, None, &nothing_on_path, &source, LINUX).unwrap();

    assert_eq!(
        source_kind,
        BinarySource::VerifiedDownload,
        "a cache entry for another platform must not be reused"
    );
}

#[test]
fn the_selected_asset_matches_the_published_platform_matrix() {
    assert_eq!(
        llsp_asset_name(LINUX),
        Some("llsp-x86_64-unknown-linux-musl.tar.gz")
    );
    assert_eq!(
        llsp_asset_name((zed::Os::Linux, zed::Architecture::Aarch64)),
        Some("llsp-aarch64-unknown-linux-musl.tar.gz")
    );
    assert_eq!(
        llsp_asset_name((zed::Os::Mac, zed::Architecture::X8664)),
        Some("llsp-x86_64-apple-darwin.tar.gz")
    );
    assert_eq!(
        llsp_asset_name((zed::Os::Mac, zed::Architecture::Aarch64)),
        Some("llsp-aarch64-apple-darwin.tar.gz")
    );
    assert_eq!(llsp_asset_name(WINDOWS), Some(WINDOWS_ASSET));
    assert_eq!(llsp_asset_name(WINDOWS_UNSUPPORTED), None);
}

#[test]
fn forwarding_reaches_the_command_on_every_resolution_path() {
    let args = vec!["--verbose".to_string()];
    let env = vec![("LLSP_LOG".to_string(), "debug".to_string())];

    let from_configured = command_with(Path::new("/configured/llsp"), &args, &env);
    assert_eq!(from_configured.command, "/configured/llsp");
    assert_eq!(from_configured.args, args);
    assert_eq!(from_configured.env, env);

    for path in [
        "/usr/bin/llsp",
        "/cache/llsp-v0.2.1/llsp",
        "/downloaded/llsp",
    ] {
        let command = command_with(Path::new(path), &args, &env);
        assert_eq!(command.command, path);
        assert_eq!(command.args, args, "args must reach the {path} command");
        assert_eq!(command.env, env, "env must reach the {path} command");
    }
}
