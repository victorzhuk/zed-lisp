# Gate and version baseline

The recorded gate and version baseline for the migration onto `llsp`. The parent change [migrate-to-llsp](../migrate-to-llsp/design.md#gates-before-cutover) and the change that produces this record [migrate-to-llsp-upstream-gates](design.md) both point here.

This document is authoritative for **G1 and G3** — the current attributed measurement of the language-ID retention guarantee and the externally owned host-aware parity — plus the immutable history of both. The other three gates are owned elsewhere; this document indexes them and holds no copy of their state, so no two documents can report different states for the same gate.

| Gate | Authoritative record | How that record is written |
|---|---|---|
| G1 Language-ID retention across a settings reload | this document, section 1 | current attributed measurement; superseding measurements appended, history immutable |
| G2 Wire identifier matches a declared dialect ID | [migrate-to-llsp-cutover/design.md](../migrate-to-llsp-cutover/design.md#identifier-map-evidence) | written by the cutover child's own record |
| G3 Host-aware context, catalog, phase parity | this document, sections 3 and 5 | current attributed measurement; closure requires external completion plus compatible verification |
| G4 Digest and extraction feasibility in the WASM host | [migrate-to-llsp-host-feasibility/feasibility.md](../migrate-to-llsp-host-feasibility/feasibility.md) | written by the feasibility child's own record, bound to the active release facts and host route |
| G5 Real-server proof | [migrate-to-llsp-acceptance/acceptance.md](../migrate-to-llsp-acceptance/acceptance.md) | written by the acceptance child's own record, attributed to the candidate it ran |

A later reader looks for a gate's current state in the record named above and nowhere else. A snapshot taken from this document into another document names the release it was taken from and is not independent authority; a snapshot that disagrees with the owning record is stale and is rejected, not reconciled.

A cell that has not been observed reads **not observed**. It is never *met*, and no gate is cleared by this document — a gate is cleared by the behavior it names, observed on the release recorded in section 0.

## 0. Release under measurement

| Item | Value |
|---|---|
| Pinned release | `v0.2.1` |
| Commit | `436bc84` |
| Published | 2026-09-30 |
| Binary `--version` as run | _not recorded — no run yet_ |
| Build provenance confirmed | _not recorded — no run yet_ |

This pin is the initial measurement of the migration, not a perpetual constraint. When a later compatible release is adopted under approved authority, its measurement is appended below as a superseding section and the sections here are preserved unchanged.

A binary whose provenance cannot be established is not used. Provenance may come from the binary's own `--version` or, when it prints no commit hash, from the verified release artifact digest mapped to the commit; a published binary is not rejected for a field it does not print. A row below measured on a different binary than the one named here is **unattributed** and is not usable as evidence.

### Superseded measurement

The parent change's `v0.2.0` (`88e3e72`) findings are preserved as history, not as current state. They were real measurements of code that this release replaces.

| # | Finding | Measured on | Current state |
|---|---|---|---|
| F1 | `workspace/didChangeConfiguration` discards the client language ID | `v0.2.0` (`88e3e72`) | superseded — re-measured in section 1 |
| F2 | The derived wire identifier is not a guaranteed dialect identifier | `v0.2.0` (`88e3e72`) | re-read in section 2 |
| F3 | No host-aware contract upstream | `v0.2.0` (`88e3e72`) | re-read in section 3 |

F1's recorded history is the disappearance of the client-reported identifier after an accepted settings change and its non-recovery across every later input. It stays visible whatever the current measurement shows: a current run that retains the identifier does not rewrite it, and a current run that loses it does not make it current again.

## 1. Gates — ownership index

| Gate | Requirement | Owner | Release / commit | Established by | Current state |
|---|---|---|---|---|---|
| G1 Language-ID retention across a settings reload | `Dialect identity is stable for the life of a buffer` | this document, below | `v0.2.1` / `436bc84` | live protocol run | not observed |
| G2 Wire identifier matches a declared dialect ID | `Explicit language identifier map for the shared server` | `migrate-to-llsp-cutover` | — | — | See authoritative owner record |
| G3 Host-aware context, catalog, phase parity | `Host-aware release gates` | this document, sections 3 and 5 | `v0.2.1` / `436bc84` | source reading; work owned by the separate `llsp`-side change | unmet |
| G4 Digest and extraction feasibility in the WASM host | `llsp binary resolution precedence` | `migrate-to-llsp-host-feasibility` | — | — | See authoritative owner record |
| G5 Real-server proof | full proof tasks | `migrate-to-llsp-acceptance` | — | — | See authoritative owner record |

### G1 — what upstream must guarantee

Upstream `llsp` must guarantee that a settings-driven re-detect preserves each open document's **client-reported language ID**. Concretely: a buffer whose dialect came from its language ID keeps that dialect across an accepted `workspace/didChangeConfiguration`, and the dialect-specific evidence already established for it — reader-invalid diagnostics — remains published.

The guarantee is not obtained by configuration, by a settings shape, or by a compensating entry added from this repository. No `llsp` code is patched from this repository to obtain it: the fix for the previously recorded defect landed in `llsp` `v0.2.1`, and this record states what upstream owes, not a diff applied here.

### G1 — measurement

Live run against the installed binary. `initialize` carries no language identifier; the identifier is sent on `didOpen`, where a real editor sends it.

| Step | `lispico-cl` on `.lisp` | `lisp` (control) | `lispico-clojure` (control) |
|---|---|---|---|
| Binary `--version` / provenance | _not run_ | _not run_ | _not run_ |
| `didOpen` with identifier — dialect and diagnostics | _not run_ | _not run_ | _not run_ |
| After one accepted `workspace/didChangeConfiguration` | _not run_ | _not run_ | _not run_ |
| After a second accepted settings change | _not run_ | _not run_ | _not run_ |
| After an ordinary edit preserving the reader-invalid condition | _not run_ | _not run_ | _not run_ |
| After a `default_dialect` restore | _not run_ | _not run_ | _not run_ |
| Control verdict (pass or fail, as observed) | n/a | _not run_ | _not run_ |

The run is **outcome-neutral**: retention, loss, and recovery are all valid observations, and this table records whichever occurred. Initial and post-change buffer content stay comparable, so the ordinary edit leaves the reader-invalid condition in place and a genuine syntax repair cannot be recorded as diagnostic loss.

Both signals are required to apply the criterion: the buffer's dialect and its reader-invalid diagnostics. Neither substitutes for the other — diagnostics are empty for a clean file, and a dialect observation can be absent for unrelated reasons. The controls are part of the same run: a `lispico-cl` buffer that never received the dialect reads exactly like one that lost it, so a missing control makes the measurement incomplete. A control that flips is recorded in the table as observed; a recorded flip is a complete observation, and it leaves G1 unmet rather than making the measurement incomplete.

**Criterion, applied to whatever the run observes.** G1 is met only when the current measurement shows the client-reported identifier retained across the configuration reload, with the reader-invalid diagnostics still published for that buffer and both controls unflipped. An incomplete measurement does not meet G1, and neither does one that shows loss or recovery in place of retention. An observation that contradicts the pinned release's fix is recorded as observed and escalated, not reconciled by editing this table.

### G1 — what the pinned release changes

Read at `436bc84`; this is a reading, not the observation above.

- The document carries the client-reported `language_id` and retains it across reloads.
- The reload path passes that identifier back into dialect detection instead of passing no identifier.
- An upstream regression test covers `lispico-cl` and `lispico-clojure` retention across a configuration reload.
- Detection order is: `files.associations` → client language ID → `#lang` / modeline → file extension → configured default dialect. Retaining the identifier is sufficient for a `.lisp` buffer, which has no suffix of its own.

## 2. Identifier map — a zed-lisp fix, not an upstream gate

llsp matches only the exact strings each dialect declares. Zed's fallback for a server entry that declares no map derives an identifier from the display name. Whether the derived value is among the declared strings is read below, not assumed.

### 2.1 Declared identifiers, read at `436bc84`

| Dialect | Declared `language_ids` | Declares `extensions` |
|---|---|---|
| `common-lisp` | `lisp`, `commonlisp`, `common-lisp` | yes |
| `lispico-clojure` | `lispico-clojure` | no |
| `lispico-cl` | `lispico-cl` | no |

Source: `dialects/common-lisp.toml`, `dialects/lispico-clojure.toml`, `dialects/lispico-cl.toml`. The two Lispico dialects declare no extensions, so a `.lisp` buffer reaches them by language ID alone.

### 2.2 Derived identifier and per-mode verdict

| Mode | Declared identifier to send | Identifier Zed derives without a map | Matched without a map? |
|---|---|---|---|
| `Common Lisp` | `lisp` | _not read_ | _not read_ |
| `Lispico Clojure` | `lispico-clojure` | _not read_ | _not read_ |
| `Lispico CL` | `lispico-cl` | _not read_ | _not read_ |

Source of the derivation: the editor's `lsp_id()` on the display name. Citation to be recorded with the read.

**The explicit map is required regardless of what the verdict column shows.** A derived value that happens to coincide with a declared identifier is a coincidence of normalization, not a guarantee, and a future editor version may change it. The map is a fix in this repository, owned by the cutover change; no upstream change is required or requested for it. G2's state is recorded by that child in its own design under Identifier-map evidence.

## 3. Host-aware parity gap

G3 is unmet. `llsp` has no catalog, host-profile, layer or phase vocabulary, so the parent's host-aware context, catalog, library and phase requirements have no upstream equivalent. The work is owned by the separate `llsp`-side change in the parent's dependency chain, implemented under its own approval in its own repository. **Nothing in this split implements any of it**, and no row here marks a parent task complete.

| Parent requirement | `llsp` equivalent | Consequence |
|---|---|---|
| _enumeration not yet recorded_ | none | unmet |

The enumeration is written requirement by requirement, each with the parent's own requirement name, the open work that owes it, and the change that owns it. A task range on its own is not an enumeration: it cannot be checked against a requirement and does not say what a reader loses.

Three kinds of open work are named separately and not conflated:

- **Open migration tasks** in the parent change — editable and tracked.
- **Archived task evidence** in `2026-10-01-add-lispico-development-support` — immutable history. Its unchecked rows are not editable open tasks and are not ticked by this split; the amendment of the requirements they describe is the cutover child's audit, made as deltas in the open parent.
- **Currently unsatisfied canonical requirements** in `openspec/specs/lispico-*` — requirements, not tasks, satisfied only by shipped behavior or by a normative delta applied through the parent's archive.

The separate `llsp`-side change's identity is recorded as found at execution time. Where it is not yet known, that absence is recorded as such rather than filled with a placeholder slug.

Catalog, profile and snapshot data remain owned by go-lispico, zhk and Yagel. This record enumerates the parent requirements that depend on them and changes no ownership.

## 4. Release contract

Read from the pinned release's published assets and digest file. Copied from an earlier release's table, this section is wrong by construction; the values here are re-read for `v0.2.1`.

### 4.1 Platform matrix and assets

| Platform | Archive asset | Present |
|---|---|---|
| Linux x86_64 | `llsp-x86_64-unknown-linux-musl.tar.gz` | yes |
| Linux aarch64 | `llsp-aarch64-unknown-linux-musl.tar.gz` | yes |
| macOS x86_64 | `llsp-x86_64-apple-darwin.tar.gz` | yes |
| macOS aarch64 | `llsp-aarch64-apple-darwin.tar.gz` | yes |
| Windows x86_64 | `llsp-x86_64-pc-windows-msvc.zip` | yes |
| Windows arm64 | — | **no archive — unsupported** |

Asset list read from the release: _not recorded — no read yet_. Unsupported platforms are read from the asset list, not copied from the parent's table. A missing platform is reported as unsupported; substituting another platform's archive is prohibited.

### 4.2 Archive member layout

| Archive | Member path | Binary |
|---|---|---|
| `*.tar.gz` | `llsp-<target>/llsp` | `llsp` |
| `*.zip` | `llsp-<target>/llsp.exe` | `llsp.exe` |

One binary per archive, under a target-named directory. Read from the pinned release's archives: _not recorded — no read yet_.

### 4.3 Digest coverage

`SHA256SUMS` is published alongside the archives and covers every file in the distribution directory, archives included. Digest file read and coverage confirmed: _not recorded — no read yet_. An archive with no digest entry is a defect in the release contract and is recorded as such, never worked around by skipping verification.

## 5. Analysis gap — unmet on this measurement, blocking

`llsp` does not provide the analysis the parent's contract requires. This is the state of the gap between that contract and the available server.

| Expected analysis | Available in `v0.2.1` |
|---|---|
| Missing libraries | no lint exists |
| Catalog identity | no lint exists |
| Phase violations | no lint exists |
| Unresolved calls | lint exists but defaults to off |

**On this `v0.2.1` measurement the gap blocks release (G3).** It is not an approved reduction, and it is not parity with the parent's planned checker. Nothing here accepts a reduced outcome in its place, and no gate is satisfied by shipping compensating configuration for it.

A superseding measurement can change the current status only after external completion — the actual change identity, its completion evidence, the released tag and commit, artifact provenance, and the mapping from what shipped to every enumerated requirement, including the catalog, profile, provenance and source-evidence obligations — plus compatible local verification of those original requirements. An upstream code merge alone is not that evidence, and no shipped release is expected to remain permanently incapable of the required behavior: this row reports the measured state of the pinned release, and a later measured release reports its own.

The consequence for the release: no release tag, marketplace publication, release package publication or release-readiness claim while the gap stands, and any documented behavior reports the reduced capability rather than presenting ordinary Common Lisp results as host-aware results. Separately authorized unreleased implementation may still proceed under the recorded authorization boundary; that state is not a release and makes no readiness claim. The acceptance child reports this same baseline status with historical evidence in its own record, and the documentation child states currently open gate-backed limitations from the authoritative records.

## 6. Ownership

| Obligation | Owed by | Implemented here |
|---|---|---|
| Language-ID retention across a settings-driven reload | `llsp` (`v0.2.1` carries the fix) | no |
| Host-aware context, catalog, library, phase behavior | the separate `llsp`-side change in the parent's dependency chain | no |
| Catalog, profile and snapshot data | go-lispico, zhk, Yagel | no |
| Explicit language identifier map | this repository, cutover change | no |
| Digest and extraction in the WASM host (`migrate-to-llsp-host-feasibility`) | this repository, feasibility change | no |

## 7. Open items

- _None recorded; this list is filled as the runs and reads are performed._
