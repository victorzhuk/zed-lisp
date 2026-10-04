# Design

## Context

This change answers one question with a measurement and two questions with an observation. It writes no contract.

The parent's `llsp-language-server-integration` delta makes the third resolution step mandatory: when no configured binary and no `PATH` binary exist, the extension fetches the release's `SHA256SUMS`, downloads the matching archive, **verifies the archive against the recorded digest, extracts it**, and only then marks the extracted binary executable. A digest mismatch is fatal for the attempt; a digest-verified archive that cannot be extracted is fatal for the attempt. Neither case may fall through to an unverified start.

The measurement question is whether that step is reachable at all inside the host. The API surface available to a Zed extension provides `download_file`, which takes a `DownloadedFileType` and therefore performs download-and-extract as a single host-side step, plus `make_file_executable`, `latest_github_release` and `github_release_by_tag_name`. There is no digest helper on that surface. So the shape of the answer is not "does SHA-256 exist" but "is there a step in this host where the untrusted archive's bytes are still intact and a digest can be computed over them, and is there a step after that where extraction happens on bytes that have already been checked". A `download_file` call that extracts as it downloads collapses those two steps, and the question becomes whether an uncompressed fetch is available as the first one.

The two research questions are the parent's own deferrals, restated here as scope: `files.associations` as a possible user mitigation while the language-ID gate is open, and the project-file initialization option against llsp's real configuration surface.

Boundaries carried from the parent, not re-decided here: the pinned release and its verified assets come from the baseline child; the language-ID retention gate is measured there; the host-aware parity gap and the analysis gap are recorded there; the cutover, the registration, the identifier map and every behavior test are the cutover child's.

The open parent `migrate-to-llsp` is the sole normative delta carrier for both capabilities; this child authors none of its own and amends no requirement. What it does write outside its own directory is bounded and enumerated: three example settings files, and — when the measured server rejects the portable context surface — metadata-only annotations in three named schema files. G4's current state is recorded in `feasibility.md` and nowhere else.

OVERSIZE: The probe, its recorded route or failure mode, the negative-result escalation and the two research answers are one measurement against one pinned release, and `feasibility.md` is the sole authoritative record of G4 that the baseline and documentation children point at. Splitting them leaves G4's consequence cited from a file that records the outcome but not the evidence behind it, or lets the initialization-option answer and the metadata-only schema annotations follow from a configuration surface read on one release while the digest route was probed on another.

## Decisions

### 1. The probe measures the real thing, not a stand-in

The probe downloads an actual published release archive for a platform the baseline record lists, in the platform's published form, and computes SHA-256 over the whole archive as it exists on the wire — the same bytes a user would receive. It compares against the published `SHA256SUMS` entry for that asset. A synthetic fixture, a truncated file, or a digest computed over the extracted binary instead of the archive does not answer the question and is not run.

The probe writes to a scratch directory outside the repository and removes it when it finishes. Nothing it produces — no cached binary, no downloaded archive, no vendored digest — is left behind, and no cache or cache state the extension would later trust is created by it.

Rejected: probing with a small local fixture. A hashing path that works on a 40-byte file tells nothing about a multi-megabyte archive read in one pass inside a memory-bounded WASM sandbox, which is the actual risk being retired.

Rejected: keeping the archive or a copy of the binary in the repository "as a fixture". It is a multi-megabyte vendored binary and a supply-chain surface in a change whose subject is supply-chain trust.

### 2. The route is named, not assumed; no primitive is hand-rolled

The probe records the API surface it actually used, in the order it used it. Two candidate routes are distinguishable in the record and the probe reports which one it exercised:

- **Uncompressed fetch then digest then extract**: obtain the raw archive bytes without host-side extraction, compute SHA-256 over them with a vetted Rust hashing crate, compare to the published digest, and only then extract using the host's own extraction support or a vetted extraction crate. This is the route that satisfies the parent's contract as written.
- **Download-and-extract as one host step**: `download_file` with a typed `DownloadedFileType` performs extraction during download. On its own it does not expose a point at which the untrusted bytes are still unverified, so it can satisfy extraction but not digest-before-trust on its own.

Neither SHA-256 nor archive extraction is hand-rolled in either route. If no vetted crate is usable in the host, that is the failure mode the probe records — not an invitation to write either primitive.

The probe names the crate it would need in each branch. Naming is not landing. Nothing enters `Cargo.toml` here: a dependency the probe settles on goes through the repository's normal reviewed approval as its own change, and until it does, the answer to G4 stands as recorded, not as implemented.

### 3. A negative result escalates; it is never converted

If no route computes a digest over the real archive before extraction inside the host, the probe records what was tried, what failed and how, and the parent's design goes back for approval. The digest requirement is not weakened to best-effort, the resolution chain is not reordered, an unverified download is not offered as a fallback, and no capability is added on the strength of a negative answer.

Rejected: recording "download without verification, verify next time if the cache allows". That converts a trust requirement into a first-run exception, and the exception is exactly the case an attacker reaches first.

Rejected: keeping the requirement in the spec and shipping the cutover against an unproven host. The chain's third step is the only step that puts third-party bytes on the machine; leaving it aspirational is the one failure this gate exists to prevent.

### 4. The `files.associations` answer is an observation, not a setting

The observation is: open a `.lisp` buffer in `Lispico CL` with a user-supplied `files.associations` entry for that pattern, publish reader-invalid diagnostics, apply an accepted settings change, and see whether they survive. The record states what was observed, in one direction or the other.

Whichever way it lands, the finding is recorded as an optional user mitigation that some setups may choose while the language-ID retention gate is open. It is not added to the extension's shipped settings, not written into the example settings as a required key, not turned into a requirement, and not presented as a resolution for the gate. Falling back to suffix-based or default-dialect selection remains prohibited, and a `files.associations` entry does exactly that kind of selection — which is why it can only be a user choice, taken knowingly, while the gate is open, and why the gate is not weakened by anyone choosing it.

Rejected: shipping the association in the example settings because the probe shows it works. It would convert a documented upstream defect into a configuration the extension appears to need, and it would then be read as the fix.

Rejected: publishing it in `examples/README.md` from this change. The parent routes that note to the cutover's settings slice, which is where the example files are edited; this change records the observation and points at that path.

### 5. The initialization-option answer is read from the server, then the examples and the schemas follow

The answer comes from llsp's real configuration surface on the pinned release: what the server reads from `workspace/didChangeConfiguration`, what it accepts in initialization options, and whether any project-file path is a recognized setting at all. The record names the surface it was read from, not a recollection of the parent's wording.

If the server does not accept a project-file initialization option, `lsp.lispico.initialization_options.project_file` is removed from all three example settings, and the three shipped schema resources are marked as a reviewed migration gate pending an upstream decision or an upstream proposal — a portable context description of zed-lisp's own, not a supported server option. If the server does accept one, the examples name the setting the server actually reads, under the server entry the cutover lands.

The marking is metadata-only, and it is owned here. In `schemas/lispico-project.schema.json`, `schemas/lispico-catalog.schema.json` and `schemas/lispico-packs.schema.json` it consists of a top-level `$comment`:

> Reviewed llsp migration gate: this portable context resource remains a required contract; consumption by the pinned llsp release is not verified unless recorded in migrate-to-llsp-host-feasibility/feasibility.md. This annotation does not change schema validation or approve reduced host-aware semantics.

In the project schema's top-level `description`, only the sentence *Place as `.lispico.json` at the worktree root, or select a different file with the server's `project_file` initialization option.* is replaced, with:

> The portable project contract places `.lispico.json` at the worktree root or permits an explicitly selected project file. Support by the pinned llsp release is a reviewed migration gate; no `project_file` initialization option is recommended without verified server support.

Every validation keyword, `$id`, title, schema version, property, required field and reference is preserved, and the catalog and packs descriptions need no unrelated rewrite. The annotation marks unsupported consumption; it never withdraws the required project contract, and it approves no reduction of it. The cutover, not this child, renames the `lsp.lispico` server entry — no document here treats `lsp.llsp` as an entry that already exists.

Either way, no option is filtered out of the outgoing request. An option the server rejects is surfaced as a configuration error; silently dropping it would discard user configuration and contradict the forwarding requirement the cutover child implements.

Rejected: keeping `project_file` in the examples because the parent's text still contains it. The parent's own decision 4 already says the option cannot be assumed supported; leaving it shipped asserts support that was never verified.

Rejected: removing the option and adding a filter that strips unknown initialization options before sending. The parent's decision 4 prohibits it, and it makes a configuration error invisible to the user who wrote it.

Rejected: deleting, renaming or weakening a schema so it stops describing a context the server cannot yet consume. The contract is required and unverified, not optional and unmet; deleting it would silently reduce what users may configure.

### 6. No spec delta, by construction

This child writes no `specs/` delta. A feasibility measurement and two observations do not change what any requirement obliges the extension to do: `llsp binary resolution precedence` and `User configuration forwarding on every path` are authored by the open parent `migrate-to-llsp`, the sole normative delta carrier, and are untouched here, as are its `common-lisp-language-server-integration` requirements. Schema metadata is not a capability delta: annotating a shipped resource changes no requirement's contract. If a probe result ever requires a requirement's contract to change, the design comes back for approval and the change is amended — that is what an escalation means.

### 7. G4 is authoritative here, and nowhere else

G4's current result — viable route, partial, negative, and whether the dependency each step needs has been approved — lives in `feasibility.md` under **Gate consequence**, and it lives only there. It is bound to the active release facts this change consumed and to the host route actually exercised.

The baseline record is not edited to carry it. That record holds attributed, dated measurements plus a gate ownership index; its G4 cell points here. A reader looking for the current G4 state looks in this file, and the two cannot disagree because only one of them states it. When the active pin changes, this record is remeasured and superseded in place, attributed to the new pin; the earlier finding is preserved as history rather than rewritten.

Nothing here clears a gate by describing it elsewhere: the state in `feasibility.md` is a claim about the probe that ran, and an unmeasured or unapproved route reads as G4 open.

## Verification

- **The probe's own evidence**, read in `feasibility.md`: the archive name, its published digest, the digest observed over the whole downloaded file, whether they match, the extraction result against the published member layout, the API surface used, the crate each step would need, and the scratch directory's removal.
- **Nothing left behind**: the probe's scratch directory is gone after the run, no archive or binary is vendored into the repository, and `Cargo.toml` is unchanged.
- **The observation's evidence**: a recorded before/after pair for a `.lisp` buffer under a user-supplied association across an accepted settings change, labelled as an observation and not attached to any requirement.
- **The configuration answer's evidence**: the llsp source location on the pinned release the answer was read from, and a diff of the three example settings that shows either the removal of the unsupported option or its retarget to the setting the server reads.
- **The schema marking's evidence**: the exact metadata that changed in each of the three schema files, and the existing schema-check path still classifying every previously valid and invalid case identically — the marking is annotation, not behaviour.
- **Gate authority**: `feasibility.md` states G4's current result and the baseline record's G4 row reads as a pointer, not a second state.
- **Structural**: `openspec validate migrate-to-llsp-host-feasibility --strict` clean, and this change contains no `specs/` delta and no `## Purpose` placeholder.

## Risks and rollback

- **The probe cannot run in this environment** (no network, no release access). The record says exactly that, names what was and was not measured, and G4 stays open. An unmeasured gate is not a cleared gate.
- **The probe succeeds but the answer is a crate the maintainers decline to approve.** G4 is still open: the probe answered a technical question, not a dependency review. The escalation path is the same one as for a negative result.
- **A negative result forces a design change.** That is the designed outcome, not a failure of this change. The digest requirement stands until the design is re-approved with a different mechanism.
- **The schema annotation is mistaken for a contract reduction.** It is not one: validation keywords, versions, properties and required fields are untouched, and the annotation points at the unverified consumption of a contract that remains required. A reviewer who believes otherwise rejects the annotation; nothing downstream depends on it having been accepted.
- **Rollback**: delete the directory, revert the three example settings files and revert the three schema files to their pre-marking content. No extension code, no registration, no cache, no test depends on anything this change lands. A later cutover rollback does not undo the schema marking — it is this change's reviewed metadata, not the cutover's — and the documentation child's own prohibition on schema edits still holds for that child.

## Cross-repository ownership

The probe reads the published release and llsp's configuration surface; it does not patch llsp, go-lispico, zhk or Yagel. The observed `files.associations` behavior is a property of a specific editor and server pair on specific versions, and it is recorded as an observation about that pair, not as a general guarantee.
