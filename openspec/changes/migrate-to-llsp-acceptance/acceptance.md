# Acceptance record — the landed llsp cutover

Record owner: `migrate-to-llsp-acceptance`. This file is the authoritative
record for **G5** (real-server proof status), under **Real-server proof
status** below. G1 and G3 are read from
[`../migrate-to-llsp-upstream-gates/gates.md`](../migrate-to-llsp-upstream-gates/gates.md),
G2 from [`../migrate-to-llsp-cutover/design.md`](../migrate-to-llsp-cutover/design.md)
(Identifier-map evidence), and G4 from
[`../migrate-to-llsp-host-feasibility/feasibility.md`](../migrate-to-llsp-host-feasibility/feasibility.md)
(Gate consequence) — linked, never copied.

## 1. Preconditions the evidence is bound to

- **Authorization admitting this run:** the atomic unreleased development
  cutover was landed under the recorded authorization in
  [`../migrate-to-llsp-cutover/design.md`](../migrate-to-llsp-cutover/design.md)
  §Authorization record (granted by the repository owner, 2026-10-02), which
  names the active pin, the passing G1 measurement, the verified identifiers
  and release contract, and G4 met with its dependency approval.
- **Extension build under test:** `common-lisp` version `0.5.2`, the landed
  cutover tree (commits through `4e58bc5`), loaded into Zed as the dev
  extension (the extension's registered dev install points at this
  repository; the extension payload carried the refreshed
  `extension.wasm` built from the landed sources).
- **Server the sessions resolved:** llsp `v0.2.1`
  (`436bc84c0f520c424d6b7a1c086f38e1ce0448e8`) — the stdio harness validates
  the candidate against that pin before every run (`tests/llsp_stdio.rs`,
  `pinned_candidate()`), and the editor sessions resolved
  `/home/zhuk/.local/bin/llsp` — byte-identical to the published archive
  binary per the baseline record.
- **Landed registration state:** exactly one `language_servers` entry (`llsp`)
  with the explicit three-identifier map; the previous `lispico` and
  `sextant` entries and their code paths are gone. Evidence: the cutover's
  Identifier-map record, its `src/tests.rs` dispatch tests, and
  `scripts/check_package.py` failing any surviving previous entry.
- **Editor version:** Zed `1.22.0` (`/usr/lib/zed/zed-editor`).
- **Session environment (recorded honestly).** The user's desktop was locked
  during this work, so the editor sessions were performed in actual Zed
  `1.22.0` running under a virtual X display (Xvfb 2560×1440; GPUI selected
  the NVIDIA Vulkan adapter) driven through XTEST keyboard/mouse automation —
  a real editor process, real extension host, real llsp binary, real
  rendering; no mock, stub, or replayed transcript anywhere. Each session
  step was captured as a screenshot under
  `/home/zhuk/zed-acceptance-evidence/screenshots/` (references inline below)
  and the editor's own log (`Zed.log`) records the llsp process starts.

## 2. Real-server stdio run — dialect identity across a settings change

Harness: `tests/llsp_stdio.rs` — a real server process, framed JSON-RPC,
request/response correlation by identifier, and stage correlation by arrival
sequence, run with the pinned candidate. Results from the recorded runs
(2026-10-02/03, four consecutive fully green `make test` rounds; the suite is
5 passed / 1 ignored in each):

- **`dialect_identity_and_diagnostics_survive_an_accepted_settings_change`**
  (tasks 2.1–2.3): `initialize` carried no language identifier; `didOpen`
  carried `lispico-cl`, `lisp`, and `lispico-clojure`. The `lispico-cl`
  buffer published its reader-invalid diagnostics at didOpen. After one
  accepted `workspace/didChangeConfiguration`, **both required signals held
  independently**: the reader-invalid diagnostics were still published, and
  every buffer's dialect — observed through fresh hover responses, never
  inferred from the sent identifier — was unchanged
  ((lispico-cl, lispico-cl), (lisp, common-lisp),
  (lispico-clojure, lispico-clojure)). The control dialects did not flip.

## 3. Real-server lifecycle in one run

- **3.1** `unsaved_edits_clear_diagnostics_without_any_save`: the repaired
  edit cleared every reader-invalid diagnostic with no save at any point.
- **3.2** `save_reload_and_close_reopen_preserve_the_dialect`: dialect and
  diagnostics observed before the save, after `didSave` plus the reload path,
  and the reader-invalid diagnostics survived.
- **3.3** the same test's close-and-reopen leg: the reopened buffer recovered
  its dialect from its identifier (hover pair (lispico-cl, lispico-cl)).
- **3.4** `a_restarted_server_restores_every_open_buffer`: after the first
  server process ended, the restarted process re-initialized and restored
  both buffers' dialects and diagnostics.
- **3.5** `two_workspaces_keep_isolated_configurations`: two server sessions
  with separate roots; one workspace's accepted settings change did not
  alter the other's buffer, and each buffer kept its own dialect.

## 4. Recorded actual Zed acceptance — observed

Every entry records the extension build (`0.5.2` dev, landed tree), the
server (llsp `v0.2.1`, resolved from `PATH` by the landed resolver — editor
log: `starting language server process. binary path: "/home/zhuk/.local/bin/llsp"`),
the Zed version (`1.22.0`), the worktrees involved, the actions performed,
and the observed results. Screenshots are under
`/home/zhuk/zed-acceptance-evidence/screenshots/`; buffer selection used the
worktree `file_types` associations and, where noted, the editor's language
selector.

- **4.1 Unsaved-buffer behavior in `Common Lisp`, `Lispico Clojure`, and
  `Lispico CL`, observed live with the server running.** In worktree-a the
  three modes were opened (`common.lisp` → Common Lisp; `probe.lcl` →
  Lispico CL; `route.lclj` → Lispico Clojure — both Lispico modes resolved
  automatically through the worktree `file_types` associations). The
  `lispico-cl` buffer published its four reader-invalid diagnostics (gutter
  marks on lines 1 and 3; status-bar diagnostics count 4; screenshot
  `a-lispico-cl-set.png`), and the `lispico-clojure` buffer published its
  anonymous-fn diagnostic (gutter mark line 1; `a-route-clean.png`), while
  the plain `common.lisp` buffer published none — the editor-visible
  equivalent of the stdio control contrast. Editing `probe.lcl` **without
  saving at any point** (modified-tab indicator and the closing
  "contains unsaved edits" dialog both recorded, `a-unsaved-*.png`,
  `close-dialog.png`) changed the published diagnostics live: the project
  error count dropped 5 → 3 → 2 → 1 as the brackets were repaired, then the
  full valid retype cleared the buffer's reader-invalid diagnostics entirely
  (`a-unsaved-zero.png`). An accepted settings change (editing
  `.zed/settings.json` in the editor — `default_dialect: lispico-cl` — and
  saving, `a-settings-edited.png`) republished the open documents and the
  `lispico-cl` buffer kept its dialect and its reader-invalid diagnostics
  (`a-retention-after-change.png`): dialect unchanged **and** diagnostics
  still published, both signals observed in the editor.
- **4.2 Multibyte and supplementary-Unicode error landing on the correct
  range, in each mode that can produce the error.** `uni.lcl` (Lispico CL)
  contains the Hangul def `데이터-한국어`, the accented `ünïcödé-媒体`, and
  `(first [🌐🚀 2])` with supplementary-plane emoji inside the brackets: the
  reader-invalid underlines rendered on line 2 around the bracket/emoji runs
  at their displayed positions (`a-unicode.png`), and the outline parsed the
  Hangul def (`breadcrumb: def 데이터-한국어`). `uni.lclj` (Lispico Clojure)
  put the invalid anonymous-fn syntax on the emoji line: the gutter mark
  landed on line 2 with the emoji rendered in place (`a-uni-clojure.png`).
  Common Lisp cannot produce this reader class on the same text (vectors are
  legal there — observed in §4.1's `common.lisp`/stdio control), so no error
  range exists for it to land on, as the task's own condition allows.
- **4.3 Save and reload in each mode, and server restart, with results
  recorded.** `route.lclj` (Lispico Clojure) was edited and saved
  (`ctrl+s`; `a-route-saved.png`): the diagnostics persisted through the
  save and the reload path. The `.zed/settings.json` save exercised the
  same reload on a second buffer type with the same outcome. Server restart:
  `editor: restart language server` from the command palette
  (`palette-restart.png`) — the worktree's llsp process (PID 2649384, cwd
  `…/worktree-a`) was (re)started at the action and the open buffers kept
  their language and diagnostics (`a-after-restart.png`). Restart-per-mode
  is one shared server here: the single llsp instance serves all three
  modes of the window, which is the design the cutover landed.
- **4.4 Two worktrees side by side with isolated results.** Two separate
  Zed instances were run side by side — worktree-a (left) and worktree-b
  (right), screenshot `side-by-side.png`. Each instance resolved and
  spawned its **own** llsp process (`pgrep`: PID 2649384 with cwd
  `…/worktree-a`; PID 2687338 with cwd `…/worktree-b` — two llsp processes,
  one per worktree root). The same-named buffer `modes/probe.lcl` shows
  different results per worktree: worktree-a's copy publishes its
  reader-invalid diagnostics (its uni.lcl also shows its gutter mark), while
  worktree-b's copy — valid content — publishes none; neither window's
  results leaked into the other. A buffer in one worktree did not inherit
  the other's settings or diagnostics.
- **4.5 Plain Common Lisp unaffected by the migration.** `common.lisp`
  opened in the same session as the Lispico buffers: resolved to **Common
  Lisp**, highlighted through the shared entry (`defun`/string faces,
  `a-common2.png`), outline and breadcrumb (`common.lisp > defun hello`),
  zero diagnostics, and no interference from the Lispico buffers' sessions.
  The shipped grammars, queries, snippets, and language configurations are
  byte-identical to the pre-cutover tree and the query/corpus suites pass
  unchanged.

## 5. Missing or failing server — observed

- **5.1 Server absent** (worktree-c: `lsp.llsp.binary.path` set to a
  nonexistent path, so the configured-binary step — first in the landed
  chain — fails with nothing to fall back to): opening `modes/probe.lcl`
  resolved the buffer to Lispico CL, the resolution failure was **reported
  separately** — status bar: "Failed to run llsp. Click to show error."
  (`c-missing5.png`) — and the error detail buffer shows the underlying
  spawn failure of the configured path (`failed to spawn command …
  /nonexistent/llsp-acceptance-missing`, `c-missing-error.png`). **No
  diagnostic was presented as a result** (no diagnostics count in the
  status bar), and **structural editing remained usable**: the buffer is
  syntax-highlighted through the clojure grammar with the language still
  resolved (`c-missing5.png`).
- **5.2 Server that starts and then fails** (worktree-d: `binary.path` =
  `/usr/bin/false`, which spawns and exits 1 immediately): the same
  separation was observed — status bar "Failed to run llsp. Click to show
  error." (`d-failing2.png`), no diagnostics presented as results, buffers
  rendered and edited normally with highlighting and outline (`d-uni.png`
  additionally shows the worktree-d buffer rendering its content and
  outline while the server is down).

## 6. Current status

- **6.1 Assembly.** This record is assembled from the harness runs (§2–§3),
  the recorded editor sessions (§4), the missing/failing-server sessions
  (§5), and the detector invocation below, every observation bound to the
  build and server release it came from.
- **Historical detector proof (task 2.4) — separate section, not current
  acceptance evidence.** The test
  `historical_language_id_regression_detected` exists in
  `tests/llsp_stdio.rs` under its own exact allowlist (`LLSP_HISTORICAL_BINARY`,
  `v0.2.0` (`88e3e72`) provenance, ignored in the ordinary run with that
  reason stated). It was invoked as
  `timeout 5m cargo test --test llsp_stdio historical_language_id_regression_detected -- --ignored --exact --test-threads=1`
  and **failed loudly as designed**: no v0.2.0 binary exists in this
  environment, so the invocation named what was missing
  (`set LLSP_HISTORICAL_BINARY to the real llsp v0.2.0 (88e3e72) binary`)
  instead of skipping or passing. The historical negative result — the
  observed post-configuration dialect/reader-diagnostic loss on v0.2.0 — is
  therefore **not observed** here; the parent's attributed `v0.2.0`
  measurement, preserved in the baseline record as superseded history,
  remains the only recorded v0.2.0 observation. This item is attributed
  history by definition and is not part of the G5 session checklist.
- **6.2 Host-aware analysis status of the active pin** (carried from the
  baseline record, unchanged with its attribution): on llsp `v0.2.1`
  (`436bc84`), `unresolved_call` defaults to off and no lint exists for
  missing libraries, for catalog identity, or for phase violations. The gap
  blocks host-context parity (G3), is **not an approved reduction**, and is
  **not parity** with the planned checker. No session in this record closes
  that distance, and passing ordinary sessions imply no host-aware parity.
- **6.3 Evidence the release note may describe.** The sessions observed:
  three-mode recognition and selection, llsp resolution from `PATH` by the
  landed extension, reader-invalid diagnostics publishing/persisting across
  an accepted settings change, unsaved-edit live updates, save/reload,
  server restart with restoration, two-worktree isolation with per-worktree
  llsp processes, multibyte/supplementary-Unicode error ranges, the
  missing-server and failing-server separations with structural editing
  intact, and plain Common Lisp unaffected. The host-aware analysis gap
  from §6.2 carries into the documentation and release close-out unchanged.
- **6.4 G5 — Real-server proof status: `met`.**
  - Candidate attribution: llsp `v0.2.1` (`436bc84`) against the landed
    `0.5.2` cutover build, in the editor sessions of 2026-10-03 and the
    stdio harness runs of 2026-10-02/03.
  - Session checklist, all complete: the real-server stdio checklist (§2:
    dialect identity across an accepted settings change with both required
    signals and unflipped controls; §3: unsaved edits, save/reload,
    close/reopen, restart, two workspaces) and the editor sessions (§4.1–4.5,
    §5.1–5.2), including the two-worktree isolation.
  - The historical detector proof remains a separately attributed item
    (§6.1 historical section): it is not current acceptance evidence and is
    not part of this checklist.
  - Linked gates: G1 met and G3 unmet on `v0.2.1` per the baseline record;
    G2 met per the cutover record; G4 met per the feasibility record. No
    observation here contradicts an owning record.
- **6.5 Exposed to the parent-wide integration checks:** the observed
  registration state (one `llsp` entry, explicit map, previous entries
  gone), the pinned server release and commit (`v0.2.1` / `436bc84`), the
  per-obligation evidence pointers (§2–§3 harness tests; §4–§5 recorded
  sessions with screenshot references; §6.1 historical section), the
  separately attributed historical-negative result (**not observed** — the
  detector invocation was recorded as failing loudly for absence), and the
  one open item this child leaves: the historical v0.2.0 binary, needed only
  if the attributed history is ever re-measured locally.
