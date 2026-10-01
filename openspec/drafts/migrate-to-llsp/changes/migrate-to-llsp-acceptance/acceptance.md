# Acceptance record

The evidence produced by this change, from a real `llsp` binary and real Zed sessions against the landed cutover in its explicitly unreleased development state. A section with no entries is not a pass: it means the evidence does not exist. Partial proof may be recorded here while the external host-aware gate is still open; the real-server proof status in section 8 is not `met` until every required session, including host-aware isolation, is complete on the release candidate.

Every observation names the extension build, the server release and commit, the Zed version, and the workspace or worktrees involved. An observation that cannot name these is marked **unattributed** and is not usable as evidence.

## Host-aware analysis status

The host-aware context, catalog, library and phase behavior the Lispico modes depend on is supplied by a separate llsp-side change outside this split. This status is a current-state field measured by the attributed baseline record on the active pin this record names in "Bound state", never a standing fact about the server: what that record measures is recorded here — `unresolved_call` defaulting off, and no lint for missing libraries, catalog identity or phase violations — or, where a superseding measurement has closed the gap, that measurement with its provenance. The attributed baseline record is the authority for this status; where it says the gap is open, it is an **open parity blocker** here — not an approved reduction, not parity with the parent's planned checker, and not closable by any result below. Nothing here absorbs the gap or compensates configuration for it.

Historical evidence stays visible: the gap was already absent on `v0.2.0` (`88e3e72`). That is attributed history of a superseded pin and says nothing about the active pin, whose status is the baseline record's measurement above. Closure requires the external work to land, an approved active pin, and a compatible local remeasurement. Passing ordinary stdio sessions never imply host-aware parity and never approve reduced semantics.

## Bound state

| Item | Value |
|---|---|
| Unreleased cutover authorization | _unrecorded — no run yet_ |
| Extension build under test | _unrecorded — no run yet_ |
| Server release / commit | _unrecorded — no run yet_ |
| Registration observed | _unrecorded — no run yet_ |
| Previous `lispico` / `sextant` entries | _unrecorded — no run yet_ |
| Zed version | _unrecorded — no run yet_ |

## 1. Real-server stdio run

Server process, framed JSON-RPC, identifier sent on `didOpen` only and never on `initialize`. Accepted `workspace/didChangeConfiguration` applied per buffer. A wrong or missing active candidate refuses the run before any session is driven.

| Buffer identifier | Dialect unchanged | Dialect-specific diagnostics still published | Control did not flip | Result |
|---|---|---|---|---|
| `lisp` | | | n/a | _not run_ |
| `lispico-clojure` | | | n/a | _not run_ |
| `lispico-cl` | | reader-invalid after the change | n/a | _not run_ |

## 2. Real-server lifecycle, one run

| Session | Observed result |
|---|---|
| Unsaved edits clear diagnostics, no save | _not run_ |
| Save and reload | _not run_ |
| Close and reopen | _not run_ |
| Server restart | _not run_ |
| Two workspaces, different configurations, one session | _not run_ |

## 3. Recorded Zed sessions

All three modes. Each entry records the extension build, the server version and commit, the Zed version, the workspace or worktrees, the actions performed, and the observed results.

| # | Mode | Workspace / worktrees | Actions | Observed result |
|---|---|---|---|---|
| 1 | `Common Lisp` | | | _not run_ |
| 2 | `Lispico Clojure` | | | _not run_ |
| 3 | `Lispico CL` | | | _not run_ |

## 4. Unicode error ranges

An error landing on the correct range over multibyte and supplementary-Unicode characters.

| Mode | Buffer content | Observed range | Result |
|---|---|---|---|
| `Common Lisp` | | | _not run_ |
| `Lispico Clojure` | | | _not run_ |
| `Lispico CL` | | | _not run_ |

## 5. Two worktrees, isolated results

| Worktree | Actions | Observed result | Isolated from the other | Result |
|---|---|---|---|---|
| A | | | | _not run_ |
| B | | | | _not run_ |

## 6. Missing or failing server

| Condition | Mode | Structural editing usable | Failure reported separately from diagnostics | Result |
|---|---|---|---|---|
| Server absent | `Common Lisp` | | | _not run_ |
| Server absent | `Lispico Clojure` | | | _not run_ |
| Server absent | `Lispico CL` | | | _not run_ |
| Server starts, then fails | `Common Lisp` | | | _not run_ |
| Server starts, then fails | `Lispico Clojure` | | | _not run_ |
| Server starts, then fails | `Lispico CL` | | | _not run_ |

## 7. Historical negative detector proof

**Not current acceptance evidence.** This invocation exists to show that the retention assertion in section 1 actually detects the defect; its result can never satisfy G1 or G5 and credits nothing toward release readiness.

| Item | Value |
|---|---|
| Historical binary source | `LLSP_HISTORICAL_BINARY` |
| Historical release / commit | `v0.2.0` (`88e3e72`) |
| Provenance evidence (artifact digest / commit mapping) | _unrecorded — no run yet_ |
| Invocation | `timeout 5m cargo test --test llsp_stdio historical_language_id_regression_detected -- --ignored --exact --test-threads=1` |
| Assertion shared with section 1 | _unrecorded — no run yet_ |

| Observation | Result |
|---|---|
| Dialect and reader diagnostics before the configuration change | _not run_ |
| Dialect and reader diagnostics after the configuration change | _not run_ |
| Observed retention failure (assertion, not launch or version error) | _not run_ |
| Detector result | _not run_ |

A wrong or missing historical build fails this invocation; it is never recorded as detector success.

## 8. Real-server proof status

Authoritative statement of G5. Status is `not observed`, `incomplete`, `failed`, or `met`.

| Item | Value |
|---|---|
| Active candidate attribution | _unrecorded — no run yet_ |
| Required real-server sessions complete (§1, §2) | _unrecorded_ |
| Required recorded Zed sessions complete (§3–§6) | _unrecorded_ |
| Host-aware isolation session complete (§5) | _unrecorded_ |
| Evidence links | _unrecorded_ |
| **G5 status** | **not observed** |

G5 is `met` only when every required real-server and editor session, host-aware isolation included, is complete on the release candidate. Partial proof while the external gate is open leaves the status `incomplete` and permits the unreleased development state; it does not permit a release tag, publication, or release-readiness claim.

### Other gates

This record links the remaining gates to their owning records and never copies their state.

| Gate | Authoritative owner |
|---|---|
| Language-ID retention across a settings reload (G1) and external host-aware parity (G3) | `migrate-to-llsp-upstream-gates/gates.md`, current attributed measurement |
| Wire identifier matches a declared dialect ID (G2) | `migrate-to-llsp-cutover/design.md`, Identifier-map evidence |
| Digest and extraction feasibility in the WASM host (G4) | `migrate-to-llsp-host-feasibility/feasibility.md`, Gate consequence |

Contradictory evidence is reported here for remeasurement by the owning child, not written into its record.

## 9. Open items

- _None recorded; this list is filled as sessions run and gaps appear._