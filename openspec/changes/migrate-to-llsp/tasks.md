# Tasks

Verification mode **existing-service-strict**, as approved. Every server-facing task drives a real `llsp` binary; no stub server is used anywhere in this list. Task order is a dependency order: gates and feasibility come before the cutover, the cutover is one atomic phase, acceptance proof follows it, and documentation and archive are last.

Scope note: this change prepares the migration. Approval covers the specifications only — no task below is authorized to start until implementation is separately approved, and the switch from the current `lispico`/`sextant` entries to `llsp` additionally waits for the gates in section 1 and the feasibility answers in section 2. The implementation is one atomic phase (section 3): registration and removal of the old entries land in the same change, never as two steps.

## 1. Gates — recorded, verified, blocking

- [ ] 1.1 Record the upstream language-ID retention gate. Reproduce the settings-change dialect loss against `llsp` `v0.2.0` (`88e3e72`) and capture the evidence: `didOpen` with `languageId=lispico-cl` on a `.lisp` buffer publishes reader-invalid diagnostics, one accepted `workspace/didChangeConfiguration` publishes none, and a second change, an ordinary edit, and `default_dialect` do not restore it. Confirm the control dialects do not flip. *(Reproduced on two independent 0.2.0 binaries; recorded in the change proposal and design.)*
- [ ] 1.2 Record the identifier-map requirement. Confirm the identifiers llsp declares for each of the three dialects, confirm the shape Zed derives from a display name when no map is declared, and record which of the three modes does not match without an explicit map. This is zed-lisp's own fix, not an upstream gate. *(Dialect declarations and the ID-matching function read in `../llsp`; map shape fixed in the new capability's spec.)*
- [ ] 1.3 Record the host-aware parity gap. Confirm which of the parent change's context, catalog, library, and phase requirements have no llsp equivalent, and mark those parent tasks as still open. *(No catalog, host-profile, layer, or phase concept exists upstream; parent 3.x/4.x stay open.)*
- [ ] 1.4 Write the upstream gate record where the parent change and this change can both point at it, stating exactly what upstream must guarantee: a settings-driven re-detect preserves each open document's client-reported language ID. Do not patch llsp from this repository.
- [ ] 1.5 Re-verify the release contract against the published `v0.2.0` assets and `SHA256SUMS`: platform matrix (Linux x86_64/aarch64, macOS x86_64/aarch64, Windows x86_64), archive member layout, and digest coverage. Record any platform with no archive.
- [ ] 1.6 Record the current upstream analysis gap: `unresolved_call` defaults to off upstream, and no lint exists for missing libraries, catalog identity, or phase violations. Describe it as the unmet gap between the parent's checker contract and the available server, which blocks host-context parity (G3). Do not describe it as an approved or accepted reduction, and do not claim parity with the parent's planned checker.

## 2. Feasibility — before any dependency, tooling, or code decision

- [ ] 2.1 Probe whether the extension's WASM host can compute SHA-256 over a multi-megabyte release archive and extract the published archive layout, using standard vetted Rust hashing and extraction crates as ordinary crate dependencies. No specific crate is prescribed and no primitive is to be hand-rolled; the probe records which route was tried and what happened. Record the result, including the failure mode, if it does not work. Any dependency the probe settles on still requires the repository's normal reviewed approval before it lands.
- [ ] 2.2 If no in-sandbox route exists, stop and return this design for approval. Do not add a capability, weaken the digest requirement to best-effort, or commit a dependency before 2.1 has a recorded answer.
- [ ] 2.3 Research only, no contract: observe whether a user-supplied `files.associations` entry keeps a `.lisp` buffer's dialect across a settings change while 1.1 is open. The outcome is recorded as an optional user mitigation; it is not a requirement of the extension, not a shipped setting, and not a substitute for gate G1. Do not turn the finding into a contract.
- [ ] 2.4 Verify whether llsp accepts a project-file initialization option at all. If it does not, remove `lsp.llsp.initialization_options.project_file` from the example settings and mark the `.lispico.json` schemas as a reviewed migration gate pending an upstream decision. Do not filter unsupported options out of the outgoing request.

## 3. Implementation — one atomic cutover

Blocked until implementation is separately authorized, and until 1.1, 1.2, 1.3, and 2.1 are resolved in this repository's favor. This whole section lands as a single change: the new registration and the removal of the old entries are never two steps, and no partial cutover ships. Section 5's acceptance runs against the result before it is released.

- [ ] 3.1 Registration slice: register `[language_servers.llsp]` for all three languages and the `language_ids` map exactly as specified, **and in the same change** remove the previous `lispico` and `sextant` server entries and the code paths that served them. No shim, no alias, no silent fallback, no parallel registration, no reopen-on-settings-change trick. Add a check that fails when a language is added to the server without an explicit identifier, and that the previous per-mode entries are gone. Update `scripts/check_package.py` and the release packaging list for whatever the manifest now needs.
- [ ] 3.2 Resolution slice: configured binary → `PATH` → verified download, with the cache layout, per-entry completion state, pruning, and offline reuse. Error text names supported platforms, stopping reason, and the three remedies. No Roswell, no source build, no `install.sh`, no alternate-platform substitution.
- [ ] 3.3 Forwarding slice: arguments, environment, initialization options, and workspace settings forwarded unchanged on every path. No wrapping, renaming, default injection, or suppression; no reopen or restart to force configuration through. A configuration the server rejects is surfaced.
- [ ] 3.4 Behavior contract tests for 3.1–3.3: precedence, forwarding on each path, digest mismatch, extraction failure, interrupted-download state, non-executable cache entry, pruning, offline reuse with and without a usable entry, unsupported platform, unknown server ID. Each case must be able to fail by a real defect; no wiring-copy assertions, no length-grew checks, no non-empty checks.
- [ ] 3.5 Removal slice: delete the now-dead sextant resolution code, its cache layout, and its tests. No compatibility path, no deprecation window, no conditional on an old setting.
- [ ] 3.6 Resource slice: update the example settings to the new server entry, preserving every display name, opt-in suffix semantic, snippet, and grammar. The 2.3 finding is carried into `examples/README.md` only as an optional user mitigation note while gate 1.1 is open, never as a required setting in the shipped templates.
- [ ] 3.7 Confirm the Common Lisp experience after cutover: same recognition, highlighting, outline, text objects, and settings surface, now served by the shared entry.
- [ ] 3.8 Update the parent change's tasks so the section 5 and 6 acceptance items name the actual server and the real gates instead of the removed upstream server, without changing any completion mark.

## 4. Acceptance proof — real server, real editor

Runs against the landed atomic cutover, before anything is released. No gate is cleared by this section; the section supplies the evidence the gates need.

- [ ] 4.1 Real-server stdio smoke: initialize, `didOpen` for `lisp`, `lispico-clojure`, and `lispico-cl`, then an accepted `workspace/didChangeConfiguration` per buffer. Assert both signals: dialect unchanged **and** `lispico-cl` reader-invalid diagnostics still published. The identifier goes on `didOpen`, never faked on `initialize`. Control dialects must not flip.
- [ ] 4.2 Real-server smoke for the rest of the lifecycle: unsaved edits clear diagnostics without save, save and reload, close and reopen, server restart, and two workspaces with different configurations in one session.
- [ ] 4.3 Actual Zed acceptance for all three modes: unsaved-buffer behavior, multibyte and supplementary Unicode before an error landing on the correct range, save/reload, restart, and two worktrees side by side with isolated results. Record versions, actions, and observed results.
- [ ] 4.4 Actual Zed acceptance that a missing or failing server leaves all three modes' structural editing usable and reports the failure separately.
- [ ] 4.5 Record the unmet analysis gap from 1.6 in the acceptance record as an open parity blocker, so no reader concludes the migration reaches the parent's planned checker semantics and no reader takes it as an approved reduction.

## 5. Documentation and release

- [ ] 5.1 Update the existing README architecture and configuration sections for the shared server, the resolution chain, the supported platforms, and the gate-backed limitations. No invented CLI flags or endpoints; every documented value comes from what 1.5 and 2.4 verified. Any note on `files.associations` is presented as an optional user mitigation, never as a required setting.
- [ ] 5.2 Add the change to the existing `CHANGELOG.md` under `[Unreleased]`, describing the user-visible outcome in one entry. No new changelog file.
- [ ] 5.3 Run the bounded full check suite and `openspec validate migrate-to-llsp --strict`; require both clean before requesting the archive.
- [ ] 5.4 Archive **only after** `add-lispico-development-support` is also complete, in the order recorded in the design: `llsp-language-server-integration` first, the baseline `common-lisp-language-server-integration` delta with it, and the amended parent last. If the parent is not complete, leave this change unarchived. No forced or premature archive, and no `TBD` purpose left behind in `openspec/specs/`.
