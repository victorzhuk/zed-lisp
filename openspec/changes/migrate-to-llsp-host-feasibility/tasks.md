# Tasks

These are the parent change's section 2 tasks, carried at their own depth. Nothing here changes the extension, adds a capability, or commits a dependency. The probe and the two observations run before any dependency, tooling or code decision anywhere in the migration.

Order is dependency order: the release facts come from the baseline record, the probe consumes them, the escalation decision follows the probe result, and the two observations are independent of all three.

**Gate authority.** G4's current result and dependency-approval state are recorded in `feasibility.md` under **Gate consequence**, and nowhere else. This change does not write G4 — or any other gate — into the baseline record; that record holds attributed, dated measurements plus an index of gate owners, and its G4 row points here. When the active pin changes, the finding here is remeasured and superseded in place, attributed to the new pin, with the earlier result preserved as history.

## 1. Release facts consumed, not restated

- [x] 1.1 Read the pinned release, asset names, `SHA256SUMS` entries and archive member layout from the `migrate-to-llsp-upstream-gates` record. Do not restate them here and do not take them from the parent change's v0.2.0 text. Fix the exact archive to probe, including its published digest and its member layout.
- [x] 1.2 Read the pinned llsp release and commit from the same record, and record both in the feasibility answer so every result below is attributable to one release.

## 2. Feasibility probe — digest before trust, then extraction

- [x] 2.1 Stand up a scratch probe outside the repository. Download an actual published release archive for a platform in the baseline record, in the published form, in the host the extension runs in. Nothing is written inside the repository.
- [x] 2.2 Record the API surface actually available for the step: whether the download can be obtained as raw bytes before extraction, what `download_file`'s `DownloadedFileType` support covers, and that the extension API exposes no hashing helper.
- [x] 2.3 Compute SHA-256 over the whole downloaded archive — the untrusted bytes, before anything is extracted — and compare it against the `SHA256SUMS` entry from 1.1. Use a vetted Rust hashing crate. Do not hand-roll SHA-256, and do not settle for a digest over the extracted binary or a partial read.
- [x] 2.4 Extract the published archive layout from bytes already digest-verified, and confirm the expected member path is produced and its permissions can be set. Use vetted extraction support; do not hand-roll extraction.
- [x] 2.5 Write the answer to `feasibility.md`: archive name, published digest, observed digest, match or mismatch, extraction result, API surface used, the route tried, the crate each step needs, and the exact failure mode with its error where the probe did not work. Record a partial result honestly — a digest that works with no extraction, or the reverse, is a recorded partial, not a pass.
- [x] 2.6 Delete the scratch directory and everything it produced. Confirm no archive, binary or digest is vendored into the repository and that `Cargo.toml` is unchanged.
- [x] 2.7 Name the crate the successful route depends on, and stop there. Do not add it, propose it in `Cargo.toml`, or vendor it; it goes through the repository's normal reviewed approval as its own change.

## 3. Escalation on a negative or partial result

- [x] 3.1 If no route computes a digest over the real archive before extraction inside the host — or if the result is partial — record it and return the parent design for approval. Do not add a capability, do not weaken the digest requirement to best-effort, do not reorder the resolution chain, and do not offer an unverified download as a fallback.
- [x] 3.2 State the G4 consequence in `feasibility.md` under **Gate consequence**, and nowhere else: G4 open on a negative or partial result, with the cutover not proceeding against a weakened digest step. Do not write this state into the baseline record or any other file; the baseline's G4 row is a pointer to this one.
- [x] 3.3 If the probe's success depends on a crate the maintainers have not approved, treat G4 as open until that approval lands. Approval of the dependency is a separate, recorded decision, not a consequence of a working probe; request it and record the decision as it is made. No approval is assumed by this change.

## 4. Research only — the `files.associations` observation

- [x] 4.1 Open a `.lisp` buffer in `Lispico CL` under a user-supplied `files.associations` entry for that pattern, confirm reader-invalid diagnostics are published, apply one accepted settings change, and record whether the buffer keeps its dialect and keeps publishing them. Do the same across a second settings change and an ordinary edit.
- [x] 4.2 Record the observation in `feasibility.md` with the editor version, the server version and the association used. Record the negative direction just as explicitly as the positive one.
- [x] 4.3 Keep it an observation. Do not add it to the extension's shipped settings, do not add it to the example settings, do not write it into a requirement, and do not describe it as resolving or substituting for the language-ID retention gate. Point at the cutover change's settings slice as the only place an optional-mitigation note may later appear, and record that such a note is required only while G1 is currently open on the baseline record's current measurement; if G1 is met, none is written.

## 5. Research only — the project-file initialization option

- [x] 5.1 Read llsp's real configuration surface on the pinned release and record what it accepts: which settings are read from `workspace/didChangeConfiguration`, whether initialization options are read at all, and whether any project-file path is a recognized setting. Cite the source location read, not the parent change's wording.
- [x] 5.2 If no project-file initialization option is accepted, remove `lsp.lispico.initialization_options.project_file` from `examples/go-lispico/.zed/settings.json`, `examples/yagel/.zed/settings.json` and `examples/zhk/.zed/settings.json`, and mark the three shipped schema resources — `schemas/lispico-project.schema.json`, `schemas/lispico-catalog.schema.json` and `schemas/lispico-packs.schema.json` — as a reviewed migration gate pending an upstream decision. The key removed is the one the current templates actually contain; the cutover later renames the `lsp.lispico` server entry, and nothing here writes or assumes an `lsp.llsp` entry.
- [x] 5.3 If a project-file option is accepted, record the exact surface the server exposes and name it in the answer, so the examples follow the server rather than the parent text. Leave the example edit to the entry the cutover lands, and record the setting name here. Support for a project-file option is not evidence of support for the catalog or packs resources: record each surface separately, or record each as unsupported.
- [x] 5.4 Do not filter unsupported initialization options out of the outgoing request, in either branch. Record that an option the server rejects is surfaced as a configuration error.
- [x] 5.5 Record the example-settings consequence in `feasibility.md` so the cutover and the documentation describe the same configuration surface the server actually has.
- [x] 5.6 When — and only when — the option is unsupported, mark the three schemas and record the marking. In `schemas/lispico-project.schema.json`, `schemas/lispico-catalog.schema.json` and `schemas/lispico-packs.schema.json` add the top-level `$comment`:

  > Reviewed llsp migration gate: this portable context resource remains a required contract; consumption by the pinned llsp release is not verified unless recorded in migrate-to-llsp-host-feasibility/feasibility.md. This annotation does not change schema validation or approve reduced host-aware semantics.

  In `schemas/lispico-project.schema.json`'s top-level `description`, replace only the sentence *Place as `.lispico.json` at the worktree root, or select a different file with the server's `project_file` initialization option.* with:

  > The portable project contract places `.lispico.json` at the worktree root or permits an explicitly selected project file. Support by the pinned llsp release is a reviewed migration gate; no `project_file` initialization option is recommended without verified server support.

  Preserve every validation keyword, `$id`, title, schema version, property, required field and reference; leave the catalog and packs descriptions otherwise untouched. Then record exactly which metadata changed in each of the three files and prove, through the existing schema-check path, that every previously valid and previously invalid case still classifies the same way. If the option is supported, omit the marking and the option removal entirely and record the surface that made them unnecessary.

## 6. Close-out

- [x] 6.1 Confirm this change contains no `specs/` delta: it adds no capability and modifies no requirement. The schema marking is metadata, not a delta.
- [x] 6.2 Run `openspec validate migrate-to-llsp-host-feasibility --strict` and require it clean.
- [x] 6.3 Expose what the integration checks need: the API surface and the route or failure mode from 2.5 for the cutover to implement; the initialization-option answer, its example-settings consequence and its schema marking from 5.5 and 5.6 for the documentation to state identically; and the G4 state as recorded here, which `integ-02` reads from `feasibility.md` rather than from a copy in the baseline record.
- [x] 6.4 If the active pin changes before this change is complete or while its findings are being relied on, remeasure against the new pin and supersede the finding in place, attributed and dated, preserving the earlier result as history. Do not rewrite an earlier measurement in the light of a later one.
