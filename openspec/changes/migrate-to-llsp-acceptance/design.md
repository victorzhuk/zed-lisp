# Design

## Context

The parent's verification mode is `existing-service-strict`: a migration onto an existing published server, proved against the real thing. Its design fixes the shape of the proof — a bounded feasibility probe, in-process behavior contract tests, a real-server stdio smoke, actual Zed acceptance, then the full checks — and requires that no stub server appear anywhere in the chain.

This child owns the last two steps. It depends on the landed atomic cutover: registration, the explicit identifier map, and the removal of the previous `lispico` and `sextant` entries land together in the cutover change, so the only registration state this child can observe is the final one. Anything else would make the evidence ambiguous about what was measured. That cutover is an explicitly unreleased development state: acceptance may record partial proof against it while the external host-aware gate is open, but G5 is not met until every required session, including host-aware isolation, is complete on the release candidate.

Two premises are inherited from siblings and are not re-derived here. The pinned server build is whatever the baseline child re-verified the gates against — not the parent's `v0.2.0` documents — and the host-aware analysis status is the baseline child's recorded finding on the active pin, not this child's conclusion. This child records the release and commit it actually ran, and refuses to present a run made against a build it cannot name.

The harness lives in the repository's `tests/` tree and speaks framed JSON-RPC to a real server process over stdio. It is a test, not a permanent capability: nothing in the extension depends on it, and it has no mock, fixture server, or recorded transcript standing in for the binary.

OVERSIZE: The stdio harness, the lifecycle sessions, the recorded Zed sessions, the historical-negative invocation and `acceptance.md` are one evidence set that the documentation child reads as the authoritative statement of what the landed candidate actually did. Delivering them apart leaves `acceptance.md` cited as authoritative for a capability whose session was never run, and splitting the historical invocation from the candidate run breaks the shared retention-assertion path it exists to prove — the two would then be free to resolve different binaries and the history row would stop being checkable against the current one.

## Decisions

### 1. Two independent signals for dialect identity

A buffer's dialect surviving a settings change is asserted twice: the dialect itself is observed to be unchanged, and `lispico-cl` reader-invalid diagnostics are observed to still be published after the accepted change. Both must hold.

Rejected: diagnostics alone. They are empty for a clean file, so a silently re-routed buffer and a healthy one look identical. Rejected: a dialect or hover signal alone. A null result is common for unrelated reasons and would let a real regression pass. Rejected: a source inspection of the server's re-detect path, which is what the baseline child's reproduction is for and which proves nothing about the shipped wire behavior.

### 2. The identifier goes on `didOpen`, and the harness must not fake it

`initialize` carries no language identifier. The identifier is sent only on `didOpen`, matching where a real editor sends it, so the run reproduces the editor's wire behavior instead of a convenient one.

Rejected: putting the dialect on `initialize` to make detection deterministic. It would make the run pass for a reason the editor does not have, and would mask exactly the class of defect the gate names.

### 3. The harness refuses a wrong or missing server before the run, with one named historical exception

Version refusal applies to the normal acceptance run: a missing, wrong-version, or non-starting active candidate is a failure of the run, reported with what was looked for and what was found, before any session is driven. It is never a skip, never an ignored exit, and never a recorded pass.

The sole exception is the separately named historical-negative invocation. It validates exactly `v0.2.0` (`88e3e72`) resolved from `LLSP_HISTORICAL_BINARY`, under its own exact historical allowlist, binary path and attribution; it does not widen or bypass the normal version guard. A missing, unverifiable, or wrong historical binary fails that invocation — it is never skipped, and a failure to resolve the historical build is not detector success. Provenance may rest on a verified release artifact digest mapped to the commit when `--version` prints no commit hash; a fabricated version string is never required.

Rejected: skipping the harness when the binary is absent so the suite stays green. A silently skipped acceptance harness is indistinguishable from a passing one at the point where release readiness is judged. Rejected equally: a permissive global version bypass, which would let any build satisfy either invocation.

### 4. Recorded Zed sessions, not a scripted editor test

The Zed-side sessions are performed and recorded by a person: each entry carries the extension build, the server version and commit, the Zed version, the workspace or worktrees involved, the actions performed, and the observed results. Two worktrees side by side, a failing server, and correct error ranges under multibyte and supplementary Unicode are observed outcomes, not assertions a harness can make about an editor it does not drive.

Rejected: driving Zed from a test suite to synthesize the same record. A generated transcript asserts that the harness sent the right keystrokes, not that the editor behaved correctly, and it cannot be reviewed the way a recorded session can.

### 5. Evidence is bound to one registration and one build

Every observation — harness run, historical detector proof, and recorded session alike — names the extension build and the server release and commit it came from. A run made against a partial registration, a surviving old entry, or an unnamed build is not evidence and is recorded as such.

Rejected: aggregating sessions run at different times into one record without versions. The parent cutover is atomic, so an unattributed observation cannot be shown to describe the shipped state.

### 6. The analysis gap is reported from the attributed baseline, not asserted permanently

The record states the current host-aware analysis status of the active pin as the baseline child measured it, with the attributed baseline evidence and the historical absence kept visible, and reports it as an open parity blocker wherever that record still says it is open: the analysis the parent's contract requires is not provided by the server, and no session result may be read as closing that distance. Closure requires the external llsp-side work plus compatible local verification; passing ordinary stdio sessions never imply host-aware parity and never approve reduced semantics.

Rejected: a standing assertion that the gap must remain open forever, which would contradict a superseding measurement after external completion. Rejected: keeping the status only in the gate record and referencing it from acceptance — a reader arriving at the evidence would see passing sessions with nothing marking missing semantics. Rejected equally: recording it as an accepted reduction. The parent forbids that reading, and a record that permitted it would convert a blocker into a release.

### 7. This record is G5's authority; the other gates are links

The real-server proof status recorded here is the authoritative statement of G5: candidate attribution, the required session checklist, evidence links, and one of `not observed | incomplete | failed | met`. G5 is met only when every required real-server and editor session is complete on the release candidate. G1/G3, G2 and G4 are owned by the baseline, cutover and feasibility records respectively; the gate rows here name those owners and point at them, and this child cites current authority and reports contradictory evidence for remeasurement rather than editing another's state.

Rejected: letting a green harness mark a gate satisfied, or copying other gates' states into this record. A gate is about what the server guarantees; a harness run on one build is evidence about that build, and a copied state rots out of date silently.

## Verification

1. **Harness, active candidate** — the stdio run over the real active binary, with both signals asserted and controls checked, and both a settings-change pass and a settings-change failure case observable from the same code path.
2. **Harness, historical detector proof** — a separate invocation over the real historical `v0.2.0` (`88e3e72`) binary. Both invocations call the same retention assertion function and the same protocol driver; the historical mode changes only which binary is resolved and which attribution is recorded, never the assertion itself, and no inline replacement or mock stands in for the server.
3. **Harness, lifecycle** — the remaining sessions in one run: unsaved edits, save and reload, close and reopen, restart, two configurations in one session.
4. **Recorded Zed sessions** — all three modes across unsaved buffers, Unicode error ranges, save/reload, restart, worktree isolation, and the missing or failing server; each entry complete or explicitly marked incomplete.
5. **Parent-wide** — the bounded full check suite and `openspec validate --strict` across the parent and every child, proving the harness and the record are consistent with the landed cutover and the pinned server.

## Risks and rollback

- **A harness failure on a gate the baseline recorded as met** — the failure stands and is escalated; the recorded gate state is re-derived by the baseline child, not edited to fit one run.
- **Historical evidence read as current proof** — the historical invocation carries its own attribution, its own record section, and an explicit exclusion from current acceptance credit; any reader confusing the two is a defect in the record.
- **Sessions recorded against a build that is not the landed one** — the record is marked unattributed and the affected entries are re-run; unattributed evidence is not usable for release readiness.
- **A session result read as parity with the parent's planned checker** — the analysis-status line referencing the attributed baseline is the correction, and any entry that contradicts it is a defect in the entry.
- **Rollback** — none needed. This child adds tests and one record; reverting removes evidence and restores no behavior.