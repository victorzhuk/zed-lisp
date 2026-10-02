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
  cutover working tree at commit `5f1efe0` ("feat: cut over all three
  language modes to the shared llsp server").
- **Server the harness resolves:** llsp `v0.2.1`
  (`436bc84c0f520c424d6b7a1c086f38e1ce0448e8`) — the candidate is validated
  against that pin before every run (`tests/llsp_stdio.rs`,
  `pinned_candidate()`), reading `LLSP_BINARY` with the documented install
  path as its default.
- **Landed registration state:** exactly one `language_servers` entry (`llsp`)
  with the explicit three-identifier map; the previous `lispico` and
  `sextant` entries and their code paths are gone. Evidence: the cutover's
  Identifier-map record and its `src/tests.rs` dispatch tests, and
  `scripts/check_package.py` failing any surviving previous entry.
- **Editor version:** Zed `1.22.0` (`/usr/lib/zed/zed-editor`) is the
  installed editor. It was **not** driven in any session recorded here (see
  §4 and §5).

## 2. Real-server stdio run — dialect identity across a settings change

Harness: `tests/llsp_stdio.rs` — a real server process, framed JSON-RPC,
request/response correlation by identifier, and stage correlation by arrival
sequence, run with the pinned candidate. Results from the recorded runs
(2026-10-02, four consecutive fully green `make test` rounds; the suite is
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

## 4. Actual Zed acceptance — **not observed; entries incomplete**

These entries require driving the installed Zed editor in live sessions.
**No editor session was performed or observed for this record**: this
repository's automation environment provided no way to operate the Zed UI,
and nothing here was fabricated to fill the gap. Each entry below is marked
**incomplete** and is not usable as evidence, exactly as the tasks require
for entries missing any required element:

- **4.1 Unsaved-buffer behavior in `Common Lisp`, `Lispico Clojure`, and
  `Lispico CL`** — incomplete: server-side equivalents are covered by §2–§3
  over stdio; the editor-side session (extensions build installed in Zed,
  buffers opened in each mode) was not observed.
- **4.2 Multibyte and supplementary-Unicode error landing on the correct
  range** — incomplete: not observed in any mode.
- **4.3 Save and reload in each mode, and server restart in each mode** —
  incomplete: observed only through the stdio harness (§3), not as editor
  sessions per mode.
- **4.4 Two worktrees side by side with isolated results** — incomplete:
  the isolation of configurations and results was observed across two stdio
  server sessions (§3.5), not across two editor worktrees.
- **4.5 Plain Common Lisp unaffected by the migration** — incomplete as an
  editor session. Structural non-regression evidence that *does* stand:
  the shipped grammars, queries, snippets, and language configurations are
  byte-identical to the pre-cutover tree, and the query/corpus test suites
  (recognition, highlighting, outline, text objects) pass unchanged through
  the shared entry.

## 5. Missing or failing server — **not observed; entries incomplete**

- **5.1 Server absent** — incomplete: whether structural editing remains
  usable in all three modes with the failure reported separately is an
  editor-side observation and was not made. The extension-side half that
  *is* proven: with configured binary, `PATH`, cache, and download all
  unavailable, the resolver returns one actionable error and Zed receives
  no command (cutover `src/tests.rs`), leaving the Tree-sitter features
  code-untouched.
- **5.2 Server that starts and then fails** — incomplete: not observed.

## 6. Current status

- **6.1 Assembly.** This record is assembled from the harness runs, the
  detector invocation below, and the session evidence above, every
  observation bound to the build and server release it came from; the
  incomplete entries are marked as such rather than omitted.
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
  remains the only recorded v0.2.0 observation.
- **6.2 Host-aware analysis status of the active pin** (carried from the
  baseline record, unchanged with its attribution): on llsp `v0.2.1`
  (`436bc84`), `unresolved_call` defaults to off and no lint exists for
  missing libraries, for catalog identity, or for phase violations. The gap
  blocks host-context parity (G3), is **not an approved reduction**, and is
  **not parity** with the planned checker. No session in this record closes
  that distance, and passing ordinary sessions imply no host-aware parity.
- **6.3 Evidence missing from this record** (the release note must be
  written from what was observed): the five editor sessions of §4; the two
  missing/failing-server sessions of §5; and the historical detector proof
  of §2.4's own section. No capability may be described that no session
  observed; the §6.2 analysis status carries into the documentation and
  release close-out unchanged.
- **6.4 G5 — Real-server proof status: `incomplete`.**
  - Candidate attribution: llsp `v0.2.1` (`436bc84`) against the landed
    `0.5.2` cutover build (commit `5f1efe0`).
  - Complete: the full real-server stdio checklist — dialect identity across
    an accepted settings change with both signals and unflipped controls
    (§2), unsaved edits, save/reload, close/reopen, restart, two workspaces
    (§3).
  - Missing: every editor session (§4, §5) and the historical detector proof
    (§6.1 historical section). Until those exist, G5 is not `met`.
  - Linked gates: G1 met and G3 unmet on `v0.2.1` per the baseline record;
    G2 met per the cutover record; G4 met per the feasibility record. No
    observation here contradicts an owning record.
- **6.5 Exposed to the parent-wide integration checks:** the observed
  registration state (one `llsp` entry, explicit map, previous entries
  gone), the pinned server release and commit (`v0.2.1` / `436bc84`), the
  per-obligation evidence pointers (§2–§3 harness tests; §4–§5 incomplete
  entries; §6.1 historical section), the separately attributed
  historical-negative result (**not observed** — detector invocation recorded
  as failing loudly for absence), and the open items left by this child
  (the §4/§5 editor sessions and the historical binary).
