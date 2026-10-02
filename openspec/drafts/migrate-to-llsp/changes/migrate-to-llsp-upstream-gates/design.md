# Design

## Context

The parent change measures everything against **llsp `v0.2.0` (`88e3e72`)** and records three findings: F1, `workspace/didChangeConfiguration` discarding the client language ID; F2, the derived wire identifier not being a guaranteed dialect identifier; F3, no host-aware contract upstream. F1 is the one item the parent cannot close from this repository, and it is the gate that holds the cutover.

The pinned release has moved. **`v0.2.1` (`436bc84`, published 2026-09-30)`** is the installed binary and the release this split pins. Its change is exactly F1's mechanism: `Document` gains `language_id: Option<String>` retained across reloads, `reload_documents` passes `doc.language_id.as_deref()` back into `Settings::detect` instead of `None`, and `tests/sync.rs` gains `language_id_survives_configuration_reload` covering `lispico-cl` on `(f [x])` and `lispico-clojure` on `#(1 2)`. `Settings::detect` resolves dialects in the order associations → client language ID → modeline → extension → `default_dialect` → first dialect, so retaining the identifier is sufficient for a `.lisp` buffer that has no suffix of its own and no `files.associations` entry.

That is the fix's shape, read from source at the pinned commit. It is not a measurement. The record this child produces carries two different marks and never conflates them: **read at `436bc84`** for what the source and the published assets say, and **observed on the installed binary** for what a live run shows. A row without the second mark is a reading, and the gate table says so. The parent's `v0.2.0` result is neither: it is a measurement of code that no longer ships, and it is preserved as history rather than carried into any current row.

Two of the parent's contracts are load-bearing here and are not re-argued: `Dialect identity is stable for the life of a buffer` forbids clearing the gate with compensating configuration or degrading to suffix and default-dialect selection, and `Host-aware release gates` forbids releasing while host-aware context, catalog and phase behavior is absent, calling that absence an open gap rather than a reduced scope.

The pinned release is the initial measurement, not a perpetual constraint on the migration. When the separately owned llsp-side change completes and a compatible release is adopted, this record appends a superseding measurement against that pin and preserves the `v0.2.1` findings as history.

One constraint on this child's own reach: the parent's own Authorization section requires separate authorization for every change to the extension, and the split therefore keeps the record and the measurement apart from the cutover. Nothing here patches llsp, go-lispico, zhk or Yagel; the record states what each owes and where that work lives.

OVERSIZE: Every row here feeds one record, and four sibling children link to that record as the authority for G1 and G3, so a partial delivery would leave `gates.md` cited as authoritative while its retention measurement, the release contract, the identifier reads and the analysis-gap rows are still missing. The rows cannot be split because the pin, the asset and digest readings, the identifier reads and the G1 observation are the same measurement on the same release: cutting between them lets one release or measurement drift across pins and leaves the G1 criterion applied to an observation made against a different binary than the contract it is recorded beside.

## Decisions

### 1. One authoritative record per gate

G1 and G3 current measurements are authoritative in `upstream-gates/gates.md`; later measurements append superseding attributed sections rather than rewriting historical facts. G2 is authoritative in `cutover/design.md` under Identifier-map evidence. G4 is authoritative in `host-feasibility/feasibility.md` under Gate consequence. G5 is authoritative in `acceptance/acceptance.md` under Real-server proof status. Other files link to these records; snapshots name their source revision and are not independent authority.

Rejected: a single all-gates file holding every current state. One writer per record means the other three owners would have to edit a file they do not own, which is a shared-write contract rather than a record — and either that contract is honoured, in which case four children edit one document, or it is ignored, in which case the copies disagree and nothing says which is authoritative. Rejected: each child keeping its own copy of the whole gate set. Two tables drift, and the drift is invisible because each looks complete — the exact failure this child exists to prevent. Rejected: keeping the rows in the parent's `design.md` only and pointing here. The parent is amended by another child and archived last; a record that moves when the document it describes is amended cannot be cited by release date.

### 2. A gate row is only a gate row with a live observation behind it

Each row carries the release, the commit, and how the state was established: a live run, a source reading at the pinned commit, or a published-asset reading. A row whose state came from the parent's `v0.2.0` result is recorded as superseded, with the parent's finding preserved as history rather than deleted — the parent's measurement was real, it was of different code, and erasing it would leave a reader unable to tell whether the fix was ever exercised.

Rejected: carrying the `v0.2.0` status forward with a note that the fix landed. A note is not a measurement; the release notes say a fix shipped, and the gate asks what the server does. Rejected equally: reading `436bc84`'s diff and calling the gate met on that basis. The diff explains the mechanism; it does not show the wire behavior, and F1 was a wire behavior.

### 3. The retention measurement is outcome-neutral, and the gate criterion is not

A `lispico-cl` buffer on a `.lisp` path is opened with `languageId=lispico-cl`, the identifier sent on `didOpen` and never on `initialize`. The run records: reader-invalid diagnostics at `didOpen`; the state after one accepted `workspace/didChangeConfiguration`; the state after a second change; the state after an ordinary edit; the state after a `default_dialect` restore. `lisp` and `lispico-clojure` buffers run alongside at the same stages. Retention, loss and recovery are all legitimate observations, and the record states whichever occurred.

The run is designed so a genuine syntax repair cannot be mistaken for a defect: initial and post-change buffer content stay comparable, so the edit after the second settings change leaves the reader-invalid condition in place instead of repairing the form. A record that showed diagnostics disappearing because the buffer became valid would measure the harness, not the server.

The criterion is applied afterwards and only to the observation: G1 is met when the current measurement shows the client-reported identifier retained across the configuration reload, with the reader-invalid diagnostics continuing to be published for that buffer, and both controls unflipped. An incomplete measurement — a missing stage, a missing control, an unattributed binary — does not meet G1, and neither does a failing one. No later input is excluded from restoring the dialect as an expected result; the run may only report what happened.

Rejected: a source-level assertion that the identifier is passed through. That is what decision 2's row already is, and it would make the two marks indistinguishable. Rejected: dropping the control dialects to shorten the run. A `lispico-cl` buffer that never received the dialect produces the same "no diagnostics" reading as one that lost it, so without the control the run cannot distinguish a pass from a harness that never opened the buffer correctly. Rejected: writing the run so that a loss is the only acceptable outcome. The pinned release carries a fix for the recorded defect, and a measurement that cannot record success cannot distinguish a fix from a broken harness.

### 4. F2 is recorded as this repository's fix, with the derivation read rather than asserted

The parent states the derived identifier is a normalization of the display name and that coincidence with a declared identifier is an accident. This record keeps the classification — a zed-lisp fix, not an upstream gate — and adds what the derivation actually yields: the three declared identifier sets read from `dialects/common-lisp.toml`, `dialects/lispico-clojure.toml` and `dialects/lispico-cl.toml`, the derivation read from the editor source, and the resulting per-mode match verdict.

Rejected: recording "all three fail to match" because the parent's argument supports it. The argument establishes that coincidence is unguaranteed; whether the current editor version coincides is a fact about that version, and a record that asserts the outcome without reading the source is the same defect this child is correcting elsewhere. The row states the outcome as read, and the requirement keeps the explicit map regardless of what the read shows, because a coincidence is not a guarantee.

Rejected: opening this as an upstream gate. llsp matches only its declared strings and is behaving as specified; there is nothing upstream to fix, and a gate record naming a nonexistent upstream obligation blocks a cutover on a phantom.

### 5. Implementation admission and release admission are different questions

The parent's gate table and task section read as if all five gates must be met before implementation starts, and the acceptance child's obligation depends on a landed cutover. Taken together those two statements cannot both hold while G3 is unmet on the pinned release, and no amount of sequencing resolves it. This child therefore separates the two admissions explicitly.

**Implementation admission** requires a separately recorded authorization that explicitly permits an atomic unreleased development cutover while G3 and G5 are still open, a current passing G1 measurement, verified dialect identifiers and release contract, and G4 met with any required dependency approval. The single server registration and its removal land together in that state; it is not a release. G2 is then established by the cutover's map and packaging/dispatch evidence, and G5 by complete acceptance on the landed candidate. If the authorization does not explicitly admit this boundary, implementation remains blocked — specification preparation alone authorizes nothing.

**Release admission** requires G1–G5 met on compatible evidence. No release tag, marketplace publication, release package publication or release-readiness claim is permitted before that.

On the `v0.2.1` measurement, G3 is unmet and blocks release. G3 is a host-context requirement that only the separately owned llsp-side work can satisfy, so making it an implementation precondition would make the migration unimplementable rather than merely unready. This child does not grant that authorization; it records the requirement and the evidence the authorization must cite.

### 6. The release contract is read from the published release, and the absent platforms are named

The pinned release's asset list gives five archives: `llsp-x86_64-unknown-linux-musl.tar.gz`, `llsp-aarch64-unknown-linux-musl.tar.gz`, `llsp-x86_64-apple-darwin.tar.gz`, `llsp-aarch64-apple-darwin.tar.gz`, `llsp-x86_64-pc-windows-msvc.zip`, alongside `SHA256SUMS`. The member layout is a single `llsp-<target>/llsp` (or `llsp.exe` on Windows), and `SHA256SUMS` covers every file in the distribution directory, archives included. Windows arm64 has no archive and is recorded as unsupported, so the extension's error text can name the platforms a user is actually refused.

Rejected: transcribing the parent's `v0.2.0` asset list. The pin moved, and an asset list is exactly the kind of value that changes between releases and is then asserted as a constant in error text and documentation. Rejected: extending support to a missing platform by substituting another archive. The parent's own resolution requirement forbids it, and it would produce a binary that cannot run.

### 7. The analysis gap is a blocker on this measurement, and the record says so in its own words

`unresolved_call` defaults to off and no lint exists for missing libraries, for catalog identity, or for phase violations. The record names the three, states that on this measurement the gap blocks release, and states that it is not an approved reduction and not parity with the parent's planned checker. A superseding measurement can change the current status only after external completion and compatible verification of the original requirements; nothing in this split implements, approximates or approves a reduced equivalent, and a code merge upstream that does not ship the host-context surface, catalog, provenance and profile requirements does not close it.

Rejected: describing the available lints as a reduced form of the parent's contract. The set overlaps in name only; the parent's contract is about host-aware resolution the server does not model at all. Rejected: leaving the gap implicit in a table cell. A reader who arrives at the gate list without the surrounding analysis will read "unmet" as a to-do item in this repository rather than a missing upstream capability.

### 8. No capability delta is written

The requirements served here are `ADDED` requirements of the parent change's `llsp-language-server-integration` delta, and the open parent `migrate-to-llsp` is their sole normative delta carrier — for both `llsp-language-server-integration` and `common-lisp-language-server-integration`, plus any audited canonical predecessor amendments. They are not restated and not modified here: re-running a measurement does not change what a requirement says, and a duplicate header collides at archive.

Rejected: an `ADDED` delta for a `llsp-gate-baseline` capability holding the record. The record is evidence, not behavior; a capability whose requirements are "the evidence exists" asserts nothing the extension or the server must do, and it would need its own archive slot and its own purpose.

### 9. Three kinds of open work are named distinctly

The parent requires that the separate llsp-side change's server identities, merged launch requirements, contradictory no-download/Roswell wording and analyzer/checker ownership be amended without reducing semantic requirements or changing completion marks. When this record enumerates what that work still owes, it distinguishes: **open migration tasks** in the parent, which are editable and tracked; **archived task evidence** in `2026-10-01-add-lispico-development-support`, which is immutable history and is not an open task any change may tick; and **currently unsatisfied canonical requirements** in `openspec/specs/lispico-*`, which are requirements rather than tasks and are satisfied only by behavior or by a normative delta applied through the parent's archive.

Rejected: quoting a task range from the archived predecessor as though it were live open work. A reader would conclude that closing those checkboxes is available to this split, and the archive is not editable. Rejected: reporting a canonical requirement as satisfied because a task row was ticked. Task completion and requirement satisfaction are different claims, and the amendment audit belongs to the cutover child.

### 10. External completion is recorded, not awaited

G3's work is owned by a change in another repository whose identity is an execution-time evidence field, not a name this revision may invent. When it completes, this child records the change identity, the completion evidence, the released tag and commit, the artifact provenance, the mapping from implemented behavior to the original requirement set, and the catalog and profile revisions carried by the release. An unidentified owner, an unreleased promise or a merged branch without a shipped surface leaves G3 open.

Rejected: writing the external change's slug into the record as a placeholder. A slug that cannot be verified reads as an identity, and a downstream reader would treat the gap as owned by a known change. Rejected: closing G3 on an upstream merge without local remeasurement. The migration depends on the shipped artifact's behavior, and the original requirement set — host context, catalog, library, phase, project format, source evidence and data ownership — is what closure is judged against.

## Verification

The record is verified by the act of producing it, in this order, and each row's mark says which act produced it:

1. **Live protocol run** against the installed `v0.2.1` binary: `initialize` with no identifier, `didOpen` per identifier, an accepted `workspace/didChangeConfiguration` per buffer, then the second change, the ordinary edit and the `default_dialect` restore, with both control dialects in the same run and the buffer content comparable across stages. The run reports the binary's `--version` and the commit it was built from; provenance may be established from the verified release artifact digest mapped to the commit when `--version` omits a commit hash, so a published binary is not rejected for a field it does not print. A build whose provenance cannot be established is not used.
2. **Source readings at `436bc84`** — the three dialect files, the detection order, and the editor's identifier derivation — each cited by file and line.
3. **Published-asset readings** — the asset list, the member layout and the `SHA256SUMS` coverage for the pinned release.
4. **Parent cross-read** — every parent requirement with no llsp equivalent, matched to the open work that owes it and the change that owns it, per decision 9.

No gate is cleared by writing the record. A gate is cleared by the behavior it names, observed on the release the rest of the split pins, and recorded in the record that owns it.

## Risks and rollback

- **A live run contradicts the fix** — the observed state stands and the row records the observation, not the expectation. The fix commit and its regression test are recorded alongside, and the discrepancy is escalated rather than reconciled by editing the row. G1 is decided from the retention criterion against whatever was observed.
- **A row is read as a pass before the run happens** — unfilled cells say *not observed*, never *met*. A gate that cannot be distinguished from a satisfied one is the failure this child exists to prevent.
- **The authority map is ignored and states fork** — each record's header names its own gate and links to the other owners, and a state contradicted by another owner's evidence is escalated for remeasurement rather than edited in place. A later reader reads one record per gate, and the record's own attribution names the release it was measured on.
- **The derived-identifier read disagrees with the parent's assertion** — the requirement keeps the explicit map; only the record's factual row changes, and the disagreement is stated in the record rather than smoothed over.
- **The record drifts from the release** — every row names its release, so drift is visible at the row rather than discovered at the cutover. If a later llsp release lands, superseding sections are appended; the earlier findings stay.
- **The unreleased boundary is read as a release** — the authorization the cutover child records names the boundary explicitly, and no publication, tag or release-readiness claim is available until G1–G5 are met on compatible evidence.
- **Rollback** — none needed. This child adds documents and changes no code, configuration or capability.

## Cross-repository ownership

This record states what each repository owes and names where that work lives; it patches nothing. The language-ID retention guarantee is owed by **llsp**, where the fix for the previously recorded defect landed in `v0.2.1`. Host-aware context, catalog, library and phase behavior is owed by the separate `llsp`-side change in the parent's dependency chain, implemented under its own approval in its own repository, and no child in this split implements any of it; this child records that change's identity, its completion evidence and the compatible release adopted, and remeasures against it. Catalog, profile and snapshot data remain owned by **go-lispico, zhk and Yagel**; this record only enumerates the parent requirements that depend on them. Editor-side identifier mapping is owned by **this repository** and is the cutover child's work, not this record's.
