# Design

## Close-out status (2026-10-02/03)

- **4.1** `make test` — exit 0: check-package ok; 18 + 16 + 2 + 5 + 4 test
  results ok (45 passing, 1 ignored — the opt-in historical detector proof);
  re-run green after the editor sessions landed.
- **4.2** `openspec validate migrate-to-llsp --strict` — exit 0.
- **4.3** readiness confirmation — performed twice. First pass (2026-10-02)
  was **not ready**: G5 was incomplete because the editor sessions were
  unobserved. Second pass (2026-10-03): the actual Zed editor sessions were
  performed and recorded (see the acceptance record §4–§5 and its §6.4) and
  **G5 reads `met`** — every required real-server and editor session is
  complete on the release candidate, G1–G4 met per their owner records, the
  amendment map covers all four predecessor classes, and the change is ready
  to archive. The historical detector proof remains an attributed-history
  item by the acceptance record's own §6.1 and is not part of the G5 session
  checklist.
- **4.4** confirmed: the README, the changelog entry, and
  `examples/README.md` name no capability absent from the acceptance record
  and state every limitation from the current authoritative gate records
  (G3 open on `v0.2.1`; G1/G2/G4/G5 met, described as verified behavior with
  the old association advice kept only as labelled history).
- **4.5–4.7** — executed after 4.3's second pass: the by-change archive
  (`migrate-to-llsp-upstream-gates`, `migrate-to-llsp-host-feasibility`,
  `migrate-to-llsp-cutover`, `migrate-to-llsp-acceptance`, then the parent
  once, then this change last per 4.7) with ordinary validated archives — no
  `--skip-specs`, no `--no-validate`, no force.
- **4.8** no open item remains; this change archives last per 4.7.

## Context

Three children land before this one: `migrate-to-llsp-upstream-gates` records the verified release and the gate baseline, `migrate-to-llsp-host-feasibility` records the host answers, `migrate-to-llsp-cutover` lands the single `[language_servers.llsp]` entry and the resolution chain, `migrate-to-llsp-acceptance` records what a real server and a real editor did. This change turns those records into user-facing prose, one changelog entry, a clean run, and an archive.

The documents being rewritten today describe the pre-migration shape: `README.md` line 3 names sextant and `lispico-lsp`, the `Prerequisites` section documents a Roswell source build for platforms without a binary, and `## Configuration (Common Lisp / sextant)` documents `lsp.sextant.binary`. `examples/README.md` lists `lispico` in its template table and states that the `lispico-lsp` binary must be supplied by the user. `CHANGELOG.md` is hand-maintained Keep a Changelog 1.1.0 with an existing `## Unreleased` section, so the entry is an edit to that section, not a new file.

The bounded check suite is `make test`, which the `Makefile` already bounds: `check-package`, then a 300 s build and a 300 s test run with `CARGO_BUILD_JOBS=4` and `RUST_TEST_THREADS=4`, each under `timeout`. The specification check is `openspec validate migrate-to-llsp --strict`, bounded the same way the repository's own review ran it (`timeout 60s`).

Two facts shape the prose. The cutover may land as an authorized unreleased development state while G3 and G5 are still open, so the documents are written against a candidate whose release status is not yet settled. And each gate has exactly one authoritative record; the observations this change states are copied from those records, never from another child's prose or from an earlier measurement of the same pin.

OVERSIZE: The README, the examples text, the changelog entry, the clean check run and the ordered archive are one close-out against one set of gate records, and every documented value is re-read from the record that owns it before it is written. Splitting them would let prose and changelog be written from one gate state while the checks and the archive ran against another, and the acceptance criterion that archives the four implementation and evidence children in dependency order before this one is unprovable if any part of the sequence lands separately.

## Decisions

### 1. Every documented value is copied from the record that owns it

Gate-backed statements are read from the authoritative record for that gate and compared by attribution — release and commit measured — before they are written:

- G1 and G3 from `migrate-to-llsp-upstream-gates/gates.md`, current attributed measurement plus its immutable history.
- G2 from `migrate-to-llsp-cutover/design.md` under Identifier-map evidence.
- G4 from `migrate-to-llsp-host-feasibility/feasibility.md` under Gate consequence.
- G5 from `migrate-to-llsp-acceptance/acceptance.md` under Real-server proof status.

Release facts — platforms, archive asset names, the `SHA256SUMS` entry, archive member layout — come from the upstream-gates baseline against the release this split pins, and the configuration surface from the landed `extension.toml` and the cutover's behavior tests. A snapshot taken against a superseded pin is not quoted: its measurement is re-read, or the fact is omitted. This change remeasures and clears no gate; it reports what its owners recorded.

Rejected: restating the parent's release facts because they are already written down. They describe a release this split does not pin, so quoting them would document a surface nobody verified.

Rejected: describing the server from its published manual or its upstream repository. The contract that matters here is what the extension selects and what the baseline checked, and the two can differ in an asset name.

### 2. Limitations are documented from the current state, and superseded ones are history

Document each relevant gate from its authoritative current record and name the measured release. If open, state the observed limitation and its release consequence. If met, describe the verified behavior and link the historical limitation where useful. G1 loss and the `v0.2.1` host/analysis gap are historical once superseded by compatible evidence; they are never required current claims. No reduction of required semantics is implied by either state.

Rejected: stating retention, host-context parity or the analysis gap as permanently open. The baseline child may observe G1 met, and a document that must describe the verified state cannot also assert an eternal defect.

Rejected: stating them as closed before an authoritative record says so. The close-out text follows the record; it does not lead it.

### 3. `files.associations` appears only as an optional user mitigation

While the language-ID gate is open, the finding may be mentioned as a user-side measure some setups may apply: never a required setting, never a shipped template value, never a substitute for the gate. If the gate is met by the time the text is written, the current setup advice is omitted; the association note survives only as clearly labeled historical context.

Rejected: documenting it as the recommended workaround, or adding it to the README's setup steps unconditionally. That converts an upstream defect into a shipped configuration obligation and hides the gate behind a setting.

### 4. The configuration section documents the settings surface, not CLI flags

The `Configuration` sections are rewritten around `lsp.llsp`: `binary.path`, `binary.arguments`, `binary.env`, `initialization_options` and `settings`, matching what the cutover forwards unchanged. The existing note that server CLI options are owned upstream and intentionally undocumented stays, and no flag or endpoint is written unless the feasibility child's recorded answer names it.

Rejected: enumerating llsp's command-line options for discoverability. The repository deliberately documents no upstream CLI surface, and an unverified flag in a README is worse than an absent one.

### 5. One changelog entry under the existing `## Unreleased`

The entry lands in the `### Changed` group of the existing `## Unreleased` section, describing the measured candidate: three language modes served by one `llsp` entry, resolution by configured binary then `PATH` then a verified release download, and the supported platforms. Currently open gate-backed limitations appear in the same entry so a release reader does not have to cross-reference the README; a limitation whose gate has closed is described as verified behavior or moved to attributed history. If a gate closes before close-out, the same entry is revised — no second migration entry is added.

Rejected: one entry per child task, which would produce a changelog describing the migration as several unrelated changes.

Rejected: creating a new changelog file, or a generated release notes artifact. The file is hand-maintained Keep a Changelog 1.1.0 and the section already exists.

### 6. Ordinary change archives, in dependency order, with the parent applied once

The predecessor `2026-10-01-add-lispico-development-support` is already archived and is immutable history: it is neither an archive target nor a completion to redo. What is required of it is that the current canonical requirements it produced are satisfied and that any normative amendment found by the cutover's audit is present in the open parent.

Before any archive command:

- The current canonical predecessor baseline is in place, the cutover's amendment audit covers all four amendment classes, G1–G5 are met on the current active pin, the change is ready to archive, and the bounded checks are clean. Readiness is the completion of every implementation, evidence and pre-archive task of the parent and of each child; the archive operations and the post-archive verification are excluded, because they are what readiness authorizes, not what it requires.

Then, in this order, using ordinary validated change archives:

1. Archive the no-delta implementation and evidence changes: `migrate-to-llsp-upstream-gates`, then `migrate-to-llsp-host-feasibility`, then `migrate-to-llsp-cutover`, then `migrate-to-llsp-acceptance`. Their evidence links are retained through their actual archive locations.
2. Archive `migrate-to-llsp` once. That single operation applies both capability deltas — `llsp-language-server-integration` and `common-lisp-language-server-integration` — together with any audited predecessor amendments. The llsp capability needs no earlier separate archive because no `MODIFIED` operation targets it.
3. Verify the resulting canonical requirements and that every evidence link resolves. A failed item stops here; nothing is archived on the strength of a passing check.
4. Archive `migrate-to-llsp-docs-release` last, as the close-out record, and only once that verification has passed.

No archive command takes a capability name.

Rejected: `--skip-specs` or `--no-validate` to get past a failing validation. Both flags exist for infrastructure and doc-only changes; this change is neither, and the recorded order is the point.

Rejected: archiving the predecessor again, or treating its presence as the completeness test. Its requirements are checked in canonical form; its history is not rewritten.

Rejected: hand-editing `openspec/specs/` after the archive. Both capabilities' `## Purpose` is already written, so archive leaves no `TBD` purpose behind; a hand edit afterwards would put the specs out of step with the archived change.

### 7. This change writes no spec delta

Neither capability's requirements change here. Both capability deltas are owned by the open parent `migrate-to-llsp`; children supply implementation and evidence. A delta authored here would be a restatement of the parent's contracts, and a restatement is how two descriptions start to disagree.

Rejected: a `MODIFIED` delta for `common-lisp-language-server-integration` covering only documentation-visible wording. A requirement that exists to shape a README is a requirement that will outlive the README.

## Verification

- Each documented value is traceable to the record that owns it: release facts to the upstream-gates baseline record, the digest-before-extraction route and the initialization-option decision to the feasibility record, the server name, resolution order and settings keys to the landed `extension.toml` and the cutover's behavior tests, and every limitation or capability claim to the acceptance record's Real-server proof status. Each gate statement names the release its measurement came from and matches the current authoritative record for that gate.
- No limitation is asserted as currently open that its authoritative record does not carry as open; a superseded limitation appears only as labelled history with its attribution and measurement release.
- The changelog gains exactly one entry and no new file appears in the tree.
- `make test` exits 0 under the Makefile's own bounds, and `openspec validate migrate-to-llsp --strict` reports the change valid; both results are recorded with their output.
- After the four child archives and the parent archive, `openspec/specs/llsp-language-server-integration/spec.md` and `openspec/specs/common-lisp-language-server-integration/spec.md` exist with a written `## Purpose` each; `common-lisp-language-server-integration` names only llsp with the three-ID map; resolution is config/`PATH`/verified download with verified offline reuse and no Roswell, source or install-script fallback; exactly one Custom server arguments pass-through requirement exists, carrying the parent's authoritative body; no Code label formatting survives; the llsp capability contains every `ADDED` requirement the parent declared, including forwarding, rejection and the host-aware release gates; any canonical predecessor amendment preserves its full semantic scenarios; and every archived evidence link resolves. `migrate-to-llsp-docs-release` is archived last.

## Risks and rollback

- **Documentation drifts from the extension.** Mitigated by the dependency on the landed cutover: the README is written from `extension.toml` as it is after the atomic change, and `scripts/check_package.py` plus the release packaging list run inside the bounded suite that gates the archive.
- **A stale record is quoted as current.** Mitigated by reading the authoritative record and comparing its attribution against the release this split pins; a mismatched snapshot is re-read or dropped.
- **A release note claims an unobserved capability.** The entry is written from the acceptance record only, and its gate statements follow the current authoritative records rather than the worst recorded state.
- **A gate closes between the first draft and close-out.** Mitigated by revising the same `## Unreleased` entry instead of adding a second migration entry, and by re-running the trace check against the new authoritative record.
- **Premature archive.** Prevented by running the checks first and by the readiness list in decision 6; an incomplete gate, task or audit leaves the change unarchived with the blocking items named. Passing checks alone do not clear a gate.
- **Rollback** — revert the `README.md`, `CHANGELOG.md` and `examples/README.md` edits. Archival is a separate, later step; if it must be undone, each archived change directory and the `openspec/specs/` deltas it applied are restored together, because one archive operation edits both.