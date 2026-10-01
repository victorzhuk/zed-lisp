# Tasks

Runs after the cutover change has landed its atomic registration and the removal of the previous entries in an explicitly unreleased development state. No gate other than this record's own G5 status is decided here; this list produces the evidence the gates and the release note are read against.

Preconditions, restated because they bound what any row below can claim: the cutover was landed under a separately recorded authorization that admits the atomic development cutover while the external host-aware gate is still open, and that authorization names the active pin, the passing G1 measurement, the verified dialect identifiers and release contract, and G4 met with its dependency approval; the server release and commit are the ones the baseline child pinned and re-verified, never the parent's `v0.2.0` documents; the observed registration is the landed one; and the analysis status is the baseline child's recorded finding on the active pin, carried here unchanged with its attribution.

If no such authorization is recorded, or it does not explicitly admit this boundary, this change does not start.

## 1. Preconditions and harness scaffolding

- [ ] 1.1 Record the cutover state the evidence will be bound to: the authorization reference admitting the unreleased cutover, the extension build under test, the server release and commit the harness resolves, the Zed version, and confirmation from the landed change that exactly one language server entry is registered, that the explicit identifier map is in place, and that the previous `lispico` and `sextant` entries and their code paths are gone. A run against anything else is not evidence and is recorded as unattributed. *(ob-4.1, ob-4.3)*
- [ ] 1.2 Add the stdio harness and its shared support module under `tests/`: a real server process, framed JSON-RPC, and request/response correlation by identifier. The positive entry point reads `LLSP_BINARY` and the active baseline; the historical entry point reads `LLSP_HISTORICAL_BINARY` and exact historical provenance. No mock, fixture, or recorded transcript stands in for the binary, and no permissive global version bypass is added.
- [ ] 1.3 Make the normal harness validate the active candidate before driving it, and fail loudly when the server is absent, is not the pinned build, or does not start, naming what was looked for and what was found. A missing or wrong server is never a skip and never a recorded pass. The historical mode validates only `v0.2.0` (`88e3e72`) under its own exact allowlist, binary path and attribution; it does not bypass the normal version guard, and a missing, unverifiable or wrong historical binary fails the historical invocation rather than skipping it. Provenance may use a verified release artifact digest mapped to the commit when `--version` omits the commit hash; a fabricated version string is never required. *(ob-4.1)*

## 2. Real-server stdio run: dialect identity across a settings change

- [ ] 2.1 Drive `initialize` with no language identifier, then `didOpen` each of `lisp`, `lispico-clojure` and `lispico-cl`, so the identifier reaches the server only where a real editor sends it and is never faked on `initialize`. *(ob-4.1)*
- [ ] 2.2 Apply an accepted `workspace/didChangeConfiguration` per buffer and assert both signals independently: the dialect of each buffer is unchanged, and `lispico-cl` reader-invalid diagnostics are still published after the change. Neither signal substitutes for the other. *(ob-4.1)*
- [ ] 2.3 Assert the control dialects do not flip through the same run: neither `lisp` nor `lispico-clojure` changes dialect as a result of the settings change. *(ob-4.1)*
- [ ] 2.4 Add the explicit `historical_language_id_regression_detected` test in `tests/llsp_stdio.rs`. Resolve its real historical binary only from `LLSP_HISTORICAL_BINARY` and verify `v0.2.0` (`88e3e72`) provenance. Drive the same retention assertion used by candidate acceptance and require an observed post-configuration dialect/reader-diagnostic retention failure, not a launch or version error. Record the failure details and historical attribution separately. The detector-proof invocation succeeds only because it observed the expected real assertion failure; it supplies no current gate or acceptance pass. Invoke it as `timeout 5m cargo test --test llsp_stdio historical_language_id_regression_detected -- --ignored --exact --test-threads=1`. The test is explicitly opt-in and ignored in the ordinary run, with that reason stated; ignoring it neither skips current acceptance nor waives the separately required negative invocation. *(ob-4.1)*

## 3. Real-server lifecycle in one run

- [ ] 3.1 Unsaved edits clear diagnostics in an unsaved buffer, with no save at any point. *(ob-4.2)*
- [ ] 3.2 Save and reload: the buffer's dialect and diagnostics are observed before the save, after it, and after the reload. *(ob-4.2)*
- [ ] 3.3 Close and reopen: the reopened buffer is observed to recover its dialect from its identifier. *(ob-4.2)*
- [ ] 3.4 Server restart: the restarted server is observed to restore the dialect and diagnostics for every open buffer. *(ob-4.2)*
- [ ] 3.5 Two workspaces with different configurations in one session: each buffer keeps its own dialect and its own results, with one workspace's settings change not altering the other. *(ob-4.2)*

## 4. Recorded actual Zed acceptance, all three modes

Each entry records the extension build, the server version and commit, the Zed version, the workspace or worktrees involved, the actions performed, and the observed results. An entry missing any of those is marked incomplete and is not usable as evidence.

- [ ] 4.1 Unsaved-buffer behavior in `Common Lisp`, `Lispico Clojure` and `Lispico CL`, observed live with the server running. *(ob-4.3)*
- [ ] 4.2 An error landing on the correct range when the line contains multibyte and supplementary-Unicode characters, in each mode where the buffer can produce the error. *(ob-4.3)*
- [ ] 4.3 Save and reload in each mode, and server restart in each mode, with results recorded. *(ob-4.3)*
- [ ] 4.4 Two worktrees side by side with isolated results: a buffer in one worktree does not inherit the other's catalog, settings or diagnostics, and each is observed separately. *(ob-4.3)*
- [ ] 4.5 Plain Common Lisp unaffected by the migration, observed in the same sessions: recognition, highlighting, outline and text objects behave as before, now served by the shared entry. *(ob-4.3)*

## 5. Missing or failing server

- [ ] 5.1 With the server absent, record that structural editing remains usable in all three modes and that the failure is reported separately, not as a diagnostic result. *(ob-4.4)*
- [ ] 5.2 With a server that starts and then fails, record the same separation: editing stays usable in all three modes and the failure is reported on its own. *(ob-4.4)*

## 6. Acceptance record and current status

- [ ] 6.1 Assemble `acceptance.md` from the harness run, the historical detector proof, and the recorded sessions, every observation bound to the build and server release it came from, and mark unattributed or incomplete entries as such rather than omitting them. Keep the historical detector proof in its own section, marked as not current acceptance evidence. *(ob-4.1, ob-4.2, ob-4.3, ob-4.4)*
- [ ] 6.2 State the current host-aware analysis status of the active pin in the record as the baseline child measured it, citing the attributed baseline record: the missing library, catalog-identity and phase-violation analysis, `unresolved_call` defaulting off, and no session result closing that distance while the attributed record says it is open. The gap is not an approved reduction. If a superseding measurement has closed it, report that measurement with its provenance; passing ordinary sessions never imply host-aware parity or approve reduced semantics. *(ob-4.5)*
- [ ] 6.3 State in the record which evidence is missing, so the release note is written from what was observed: no capability may be described that no session here observed, and the analysis status from 6.2 is carried into the documentation and release close-out. *(ob-4.5, supports ob-5.4)*
- [ ] 6.4 Record the authoritative real-server proof status for G5 — candidate attribution, the required session checklist, evidence links, and one of `not observed | incomplete | failed | met` — met only when every required real-server and editor session, including host-aware isolation, is complete on the release candidate. Link G1/G3 to the baseline record, G2 to the cutover record, and G4 to the feasibility record rather than copying their states; where an observation contradicts an owning record, report the contradiction for remeasurement instead of editing it. *(ob-4.5)*
- [ ] 6.5 Expose what the parent-wide integration checks need: the observed registration state, the pinned server release and commit, the per-obligation evidence pointer, the separately attributed historical-negative result, and the open items left by this child. *(ob-4.1, ob-4.3, ob-4.5)*