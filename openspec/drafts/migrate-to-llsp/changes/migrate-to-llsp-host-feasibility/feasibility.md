# Feasibility record

Answers three questions with evidence. This file is the record the tasks in `tasks.md` fill in. Nothing below is a claim until the corresponding task has run and its evidence is written here; a section that says *not measured* is a section that keeps its gate open, never a section that is quietly passed.

**Gate state this record controls:** G4 — digest and extraction feasibility in the WASM host (`llsp binary resolution precedence`). **The Gate consequence section below is the single authoritative record of G4's current state.** G1 (language-ID retention) and G3 (host-aware parity) are measured by `migrate-to-llsp-upstream-gates` and are not claimed or restated here. G2 is authoritative in `migrate-to-llsp-cutover`'s design under Identifier-map evidence; G5 is authoritative in `migrate-to-llsp-acceptance`'s record under Real-server proof status. Other documents point at these records rather than copying their states, and this file writes no gate state into the baseline record.

The distinction between the two is deliberate. Measurements are immutable facts attributed to a release and a date; they accumulate and are superseded, never rewritten. The gate consequence is the current live answer, read from the latest superseding measurement and bound to the active release facts and the host route actually exercised.

**Pinned release and commit:** read from the `migrate-to-llsp-upstream-gates` record, not from the parent change's documents. *(fill in: release tag, commit)*

## 1. Release facts consumed from the baseline record

*(fill in: archived asset names for the probed platform; the `SHA256SUMS` entry and its published digest; the archive member layout; the supported-platform list. Copied from the baseline record, never restated independently and never taken from the parent change's v0.2.0 text.)*

## 2. Probe — SHA-256 over the real archive, before extraction

- **Archive probed:** *(fill in)*
- **Scratch location:** *(fill in — outside the repository)*
- **API surface available for the step:** *(fill in — whether a raw, un-extracted download is obtainable; what `download_file` with its `DownloadedFileType` support covers; that the extension API exposes no hashing helper)*
- **Route tried:** *(fill in — one of: raw fetch → digest → extract; download-and-extract as a single host step; or the route actually attempted)*
- **Digest over the whole archive, before extraction:** *(fill in — the value computed over the untrusted bytes)*
- **Published digest for that asset:** *(fill in)*
- **Match:** *(fill in — yes / no)*
- **Extraction of the published member layout from digest-verified bytes:** *(fill in — member path produced, permissions settable)*
- **Crate each step would need:** *(fill in — named only; not added, not proposed in `Cargo.toml`, subject to the repository's normal reviewed approval)*
- **Hand-rolled primitive used:** none. *(assert only if true)*
- **Failure mode, where the probe did not work:** *(fill in — the exact error and the step it occurred at. A partial result is recorded as partial: digest works / extraction does not, or the reverse.)*
- **Result:** *(fill in — viable route / partial / negative)*

### Cleanup

- Scratch directory removed after the run: *(fill in)*
- No archive, binary or digest vendored into the repository: *(fill in)*
- `Cargo.toml` unchanged: *(fill in)*

### Gate consequence

**This section is the authoritative current G4 state.** Nothing else states it: the baseline record's G4 row is a pointer here, and no snapshot, index or integration check copies it.

*(fill in — G4 met on a viable route with the dependency approved; G4 open on a partial, a negative, or a route whose crate has not been approved, or on any remeasurement not yet run against the active pin. On partial or negative: the parent design is returned for approval, the digest requirement is not weakened to best-effort, the resolution chain is not reordered, no unverified download is offered as a fallback, no capability is added, and the cutover does not proceed. Record the escalation here: (fill in).)*

**Basis:** the latest measurement in section 2 above, and the approval state of the crate it names. *(fill in — which measurement, which release, which crate, approved or not)*

**When the active pin changes:** this section is superseded in place by the remeasurement, attributed and dated to the new pin. The earlier measurement and its consequence stay in the record as history; they are not edited to agree with the new one. A finding that is not compatible with the new pin — a changed release layout, a changed host route, changed dependencies — is not carried forward; a reader who needs the current answer reads this section, and a reader who needs the history reads the measurements above it.

## 3. Research only — `files.associations` while the language-ID gate is open

- **Setup:** `.lisp` buffer, `Lispico CL`, user-supplied `files.associations` entry *(fill in)*. Editor version *(fill in)*, server version *(fill in)*.
- **Before the settings change:** reader-invalid diagnostics published? *(fill in)*
- **After one accepted settings change:** dialect retained? diagnostics still published? *(fill in)*
- **After a second settings change and an ordinary edit:** *(fill in)*
- **Observation:** *(fill in — recorded in whichever direction it lands)*

**Status: an optional user mitigation only.** Not a shipped setting, not an extension requirement, not documented as required, not a substitute for the language-ID retention gate. Suffix- and default-dialect selection remain prohibited as a mechanism. An optional-mitigation note may later appear only in the cutover change's settings slice, editing `examples/README.md`, and only while G1 is currently open in the baseline record's authority — if G1 is met on the current measurement, no such note is written. *(Confirm no such note was added by this change: fill in)*

## 4. Research only — the project-file initialization option

- **Source read on the pinned release:** *(fill in — the file and symbol the answer comes from, not the parent change's wording)*
- **Settings the server reads from `workspace/didChangeConfiguration`:** *(fill in)*
- **Initialization options read at all:** *(fill in)*
- **Project-file path recognized as a setting:** *(fill in — yes with the exact setting name / no)*

**Consequence for the example settings:**

- *(fill in — `lsp.lispico.initialization_options.project_file` removed from `examples/go-lispico/.zed/settings.json`, `examples/yagel/.zed/settings.json` and `examples/zhk/.zed/settings.json`; or the setting the server actually reads, named here for the cutover's entry.)*

**Consequence for the shipped schemas:**

- **Files in scope:** `schemas/lispico-project.schema.json`, `schemas/lispico-catalog.schema.json` and `schemas/lispico-packs.schema.json`. Nothing else under `schemas/` is in scope. *(fill in — marked / not marked, and why)*

- **Top-level `$comment` added to each of the three:**

  > Reviewed llsp migration gate: this portable context resource remains a required contract; consumption by the pinned llsp release is not verified unless recorded in migrate-to-llsp-host-feasibility/feasibility.md. This annotation does not change schema validation or approve reduced host-aware semantics.

- **Project schema `description`:** only the sentence recommending the server's `project_file` initialization option is replaced, with:

  > The portable project contract places `.lispico.json` at the worktree root or permits an explicitly selected project file. Support by the pinned llsp release is a reviewed migration gate; no `project_file` initialization option is recommended without verified server support.

- **Metadata changed, per file:** *(fill in — the exact keys and values edited in each of the three files)*

- **Semantics preserved:** every validation keyword, `$id`, title, schema version, property, required field and reference unchanged; the existing schema-check path classifies every previously valid and previously invalid case the same way. *(fill in — the case set run and its result)*

- **Per-surface support:** whether the catalog and packs resources are supported is recorded separately from the project option. Project-option support alone does not establish support for either. *(fill in — project / catalog / packs, each yes with the exact surface or no)*

If the option is supported, this subsection reads *not marked* with the surface that made the marking unnecessary, and no option is removed from the examples.

**Forwarding is unaffected in both branches:** no unsupported initialization option is filtered out of the outgoing request. An option the server rejects is surfaced as a configuration error. *(fill in — confirm no filtering was introduced)*

## 5. What the integration checks consume from this record

- **For `migrate-to-llsp-cutover`:** the API surface, the route, and the result from section 2 — the mechanism the resolution chain's digest-before-extraction step implements, and the fact that a negative answer stops the cutover rather than weakening the step. The cutover reads G4 from this record; it does not restate it.
- **For `migrate-to-llsp-docs-release` and `integ-02`:** the initialization-option answer from section 4, its example-settings consequence and its schema marking, so the outgoing request, the example settings, the three schema files and the README describe one configuration surface. `integ-02` resolves G4 through the Gate consequence section above and rejects any state copied from the baseline record.
- **Shared-file ordering:** these settings and schema edits land before `migrate-to-llsp-cutover` retargets the example settings and lands the renamed server entry. A cutover rollback does not revert the schema marking recorded here.
- **Not claimed here:** the release asset names, `SHA256SUMS` entry, member layout and platform list, and the language-ID, identifier-map, host-aware-parity and analysis-gap findings. Those are the baseline record's.
