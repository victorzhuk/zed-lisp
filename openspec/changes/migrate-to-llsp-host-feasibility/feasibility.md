# Feasibility record — llsp binary resolution in the WASM extension host

Record owner: `migrate-to-llsp-host-feasibility`. This file is the sole
authoritative record of **G4** (digest-before-extraction feasibility in the
host the extension runs in) and its dependency-approval state, under
**Gate consequence** below. The baseline record
(`../migrate-to-llsp-upstream-gates/gates.md`) carries a pointer to here, not a
second state. When the active pin changes, this finding is remeasured and
superseded in place, attributed to the new pin, with the earlier result
preserved as history.

## Gate consequence

**G4 is met.** The probe succeeded on every step, and the dependency approval
the route waited on has since been recorded: on 2026-10-02 the repository
owner approved adding `sha2`, `flate2` (with its pure-Rust `rust_backend`)
and `tar` — plus `zip` for the Windows archive form — to `Cargo.toml` as
ordinary dependencies of the cutover change, recorded there as part of the
authorization that admits the atomic unreleased development cutover. The
earlier state below is preserved as history.

Historical measurement state at the time of the probe (2026-10-02), preserved
verbatim before the approval landed:

- Route: **proven.** SHA-256 over the whole published archive before anything
  is extracted, then extraction of the published member layout from the
  verified bytes only, both computed inside `wasm32-wasip2` — the target the
  extension host runs — under a standalone WASI runtime. The negative
  direction was exercised: a wrong expected digest stops the probe with
  nothing extracted.
- Dependencies: **not approved at measurement time.** The hashing step needs `sha2` and the
  extraction step needs `flate2` (with its pure-Rust `rust_backend`) plus
  `tar`; none of the three is in the repository's `Cargo.toml`, and per the
  split's rules none is added or assumed by this change. Approval of the
  dependency is a separate, recorded decision. Until it lands, G4 reads as
  open on adoption: the mechanism is proven, the dependency review is not.
- Consequence for the cutover: the cutover's resolution slice may be built on
  this route only after the crates are approved; the digest requirement is
  never weakened to best-effort, the chain is not reordered, and no unverified
  download is offered as a fallback. A negative or partial probe result would
  have returned the parent design for approval; no such return is needed.

## Release facts consumed (not restated)

Read from `../migrate-to-llsp-upstream-gates/gates.md` — the record this
result is bound to — and not from the parent's `v0.2.0` text:

- **Pinned release:** llsp `v0.2.1`, commit `436bc84c0f520c424d6b7a1c086f38e1ce0448e8`,
  published 2026-09-30; every result below is attributable to that release.
- **Archive probed:** `llsp-x86_64-unknown-linux-musl.tar.gz` — the published
  archive for the platform the probe's host matches (Linux x86_64). Its
  published `SHA256SUMS` entry is
  `9225445246f8429854c2f3f1b71c26d3fb3392d29cea602b356254d013ba3581`, and its
  published member layout is the single top-level directory
  `llsp-x86_64-unknown-linux-musl/` containing the binary `llsp` plus
  `LICENSE`, `CHANGELOG.md`, `README.md`. The Windows aarch64 platform has no
  published archive and remains unsupported; it plays no part in this probe.

## Probe answer — digest before trust, then extraction

**Archive and form.** The actual published archive, in the published form —
the exact bytes a user would receive, verified against the baseline record's
digest before the probe used them — downloaded to a scratch directory outside
the repository. Nothing was written inside the repository.

**Host.** A scratch Rust probe built for `wasm32-wasip2` (the extension host
target, same as the repository's `Makefile`) and executed under wasmtime
v49.0.2 with two private preopened directories. The download step itself was
not re-implemented: the host API performs it, and the probe consumes the
resulting bytes; what the probe proves is the two primitives the API does not
provide.

**API surface used (recorded, task 2.2).** Read from `zed_extension_api`
0.7.0 (the repository's dependency):

- `download_file(url, path, DownloadedFileType)` — WIT import
  `extension.wit:57`; the type covers `gzip`, `gzip-tar`, `zip`,
  `uncompressed` (`extension.wit:23-32`). **`Uncompressed` fetches the raw
  bytes without any host-side extraction**, which is the first step the
  digest-before-extraction route needs: there is a point at which the
  untrusted archive's bytes are intact on disk and nothing has been extracted.
- `make_file_executable(path)` — WIT import `extension.wit:60`; the host
  applies the executable bit, which the WASI filesystem surface itself cannot
  do (see below).
- `latest_github_release` / `github_release_by_tag_name` — WIT import
  `github.wit:29`; release and asset metadata.
- **No hashing helper exists anywhere on the surface** (no digest, SHA, or
  hash function is exported), which is why the digest step needs a crate.

**Step 1 — SHA-256 over the whole untrusted archive (task 2.3).** The probe
read the full 1,982,445-byte archive in one pass and computed SHA-256 with
`sha2` 0.10 (RustCrypto), inside `wasm32-wasip2`. Observed digest:
`9225445246f8429854c2f3f1b71c26d3fb3392d29cea602b356254d013ba3581` — equal to
the published `SHA256SUMS` entry. **Match.** No primitive was hand-rolled.

**Step 2 — extraction from the verified bytes (task 2.4).** Only after the
match did the probe extract, from the same bytes, using `tar` 0.4 over
`flate2` 1 (pure-Rust `rust_backend`) — vetted crates, no hand-rolled gzip or
tar parsing. Result: all five published members produced exactly —
`llsp-x86_64-unknown-linux-musl/{,CHANGELOG.md,LICENSE,README.md,llsp}` — and
the extracted binary hashed to
`60b14c494615bd1c6f73aff52e00268444105b50e6e593ea13985f1741526e2d`, the same
digest the baseline record attributes to the published binary. **Extraction is
byte-exact.**

One real failure mode was recorded on the way (task 2.5's exact failure
requirement): the tar crate's own `Entry::unpack` attempts to apply the
archive's permission bits, and the WASI runtime rejected that chmod
(`failed to set permissions to 755 for /out/llsp-x86_64-unknown-linux-musl/`).
The recorded route therefore writes each member's bytes through the crate's
decoder instead of `Entry::unpack`'s permission application — the format
parsing is still the vetted crate's. The executable bit itself is out of
reach of the WASI std (`std::fs::Permissions` on `wasm32-wasip2` exposes only
`readonly`/`set_readonly`; WASI preview 2 has no chmod), so the resolution
chain's executable-bit step goes through the host's existing
`make_file_executable` import — the API call the cutover uses for exactly
this, and the reason the import exists.

**Negative control.** The same probe run with a wrong expected digest stopped
with `MISMATCH — digest check failed; nothing will be extracted`, exit status
1, and an empty output directory. The digest step gates extraction, and the
check can fail.

**Route summary (task 2.5).** Uncompressed fetch (`download_file` with
`DownloadedFileType::Uncompressed`) → SHA-256 over the whole file (`sha2`) →
compare against the published `SHA256SUMS` entry → extract only on match
(`tar` + `flate2`) → `make_file_executable` for the executable bit. Every
step recorded a working result; the route is not partial.

**Scratch cleanup (task 2.6).** The scratch directory and everything it
produced (probe sources, wasm binary, the archive copy, the extracted tree)
was deleted after the run. No archive, binary, or digest is vendored into the
repository, and the repository's `Cargo.toml` is unchanged by this change.

**Crate dependency (task 2.7).** The route depends on: `sha2` (digest),
`flate2` with default-features off and `rust_backend` (gzip decode), and
`tar` (archive member iteration/decode). Naming them is not landing them:
none is added to `Cargo.toml`, none is vendored, and each goes through the
repository's normal reviewed approval as its own change before the cutover
builds on it.

## Escalation state

No escalation is triggered: the probe is a full positive on every step, so
tasks 3.1's return-for-approval branch and the partial-result branch do not
apply. The dependency approval that was the only open item on G4 has since
been recorded (see **Gate consequence**); the resolution chain was not
reordered and no capability was added here.

## Observation — `files.associations` as a dialect mitigation

Research only. Label: an **observation about one editor/server pair**, not a
requirement, not a setting, and not attached to any capability.

- **Editor side.** Not observed in a live Zed editor session — this
  repository's environment provided no editor session for the run, and the
  observation below is the server-side stdio equivalent of the task's
  sequence. Editor-session evidence, if wanted, belongs to the acceptance
  child's recorded sessions.
- **Server side, observed (run `843334c9`, 2026-10-02, llsp `v0.2.1`
  `436bc84` binary):** a `.lisp` buffer opened with `languageId=lisp` and no
  association resolved to the `common-lisp` dialect (hover fence identifier
  `lisp`, builtin name `common-lisp`; zero diagnostics). After one accepted
  `workspace/didChangeConfiguration` carrying the user-supplied association
  `{"*.lisp": "lispico-cl"}`, the buffer resolved to the `lispico-cl` dialect
  and published its four reader-invalid diagnostics; after a second accepted
  settings change with the same association, and after an ordinary edit that
  preserves the reader-invalid condition, the buffer kept the `lispico-cl`
  dialect and kept publishing the diagnostics. No rejection warning was
  received at any stage.
- **Reading.** On this release a user-supplied `files.associations` entry
  survives accepted settings changes and ordinary edits, because associations
  precede the language ID in the detection order — consistent with the G1
  retention the baseline record measured on the same release.
- **Consequence (task 4.3).** The baseline record currently marks **G1 met**
  on `v0.2.1`; per this task, no mitigation note is written anywhere — not in
  the shipped settings, not in the example settings, not in
  `examples/README.md`, and not into any requirement. The only place an
  optional-mitigation note may later appear is the cutover change's settings
  slice, and it is required only while G1 is open on the baseline record's
  current measurement. The observation stays an observation.

## Observation — the project-file initialization option

**Answer (task 5.1): llsp accepts initialization options, but no project-file
path is a recognized setting on the pinned release.** Read at `436bc84`:

- Initialization options are read: `src/server.rs:83-85` stores
  `initializationOptions` into the configuration layers, and the merged table
  is resolved (`src/config.rs:205-217`). The merged surface is
  `[files]` (`associations`, `default_dialect`, `max_file_size`),
  `[workspace]` (`index`, `exclude`, `max_files`, `max_symbols`),
  `[completion]`, `[diagnostics]`, `[format]`, `[log]`, and `[dialects]`
  (`src/config.rs:15-22`). All sections use `deny_unknown_fields`
  (`src/config.rs:14,26,45,71,97,123,141`).
- `workspace/didChangeConfiguration` is read when the settings carry an
  `llsp` key, unwrapped and merged the same way (`src/server.rs:521-548`).
- **A project-file path is not a recognized setting in any section.** The
  project file is the fixed name `.llsp.toml` (`PROJECT_FILE`,
  `src/config.rs:9`), loaded from the workspace root only
  (`src/config.rs:198-203`, `src/server.rs:79`); no configuration key selects
  a different path, and `.lispico.json` is not a name llsp reads.

**Example-settings consequence (tasks 5.2, 5.5).** The option is unsupported,
so `lsp.lispico.initialization_options.project_file` was removed from
`examples/go-lispico/.zed/settings.json`, `examples/yagel/.zed/settings.json`
and `examples/zhk/.zed/settings.json`. Each file's `lsp` object contained only
that option, so the whole `lsp` object was removed from all three — the
`file_types` and `languages` blocks are untouched. No `lsp.llsp` entry is
written or assumed here; renaming the server entry is the cutover's settings
slice. The schemas are marked as below. The outgoing request filters
nothing (task 5.4): an option the server rejects is surfaced as a
configuration error by the forwarding requirement the cutover implements;
this change adds no filter in either branch.

**Schema marking (task 5.6).** The three shipped schema resources are marked
as a reviewed migration gate pending an upstream decision:

- `schemas/lispico-project.schema.json` — top-level `$comment` added with the
  agreed gate text, and in the top-level `description` only the sentence
  *"Place as `.lispico.json` at the worktree root, or select a different file
  with the server's `project_file` initialization option."* was replaced with
  the agreed sentence about the portable project contract and the reviewed
  migration gate. Every other keyword, `$id`, title, schema version, property,
  required field and reference is untouched.
- `schemas/lispico-catalog.schema.json` — top-level `$comment` added with the
  same gate text; description otherwise untouched.
- `schemas/lispico-packs.schema.json` — top-level `$comment` added with the
  same gate text; description otherwise untouched.

**Proof that classification is unchanged.** The existing schema-check path
(`cargo test --test config`, 13 tests) was run after the marking: all
previously valid fixtures still validate (`project_templates_validate_against_the_schema`,
`installed_pack_snapshot_contracts`, `packs_layer_selection_contracts`) and
all previously invalid cases still reject
(`project_schema_rejects_unknown_and_invalid_configuration`,
`catalog_schema_rejects_invalid_entries_and_provenance`). `$comment` is an
annotation keyword and changes no validation behavior — the run proves it.

## Close-out

- **No spec delta (task 6.1).** This change contains no `specs/` directory:
  it adds no capability and modifies no requirement. The schema marking is
  metadata, not a delta; `skip_specs: true` is declared in this change's
  `.openspec.yaml` for that reason. (The split tooling's `parent`/`revision`/
  `split` keys were removed from this child's `.openspec.yaml` because the
  installed `openspec` validator rejects unknown keys under `--strict`; the
  linkage remains in `openspec/activation/migrate-to-llsp-host-feasibility.json`.)
- **Validation (task 6.2).** `openspec validate migrate-to-llsp-host-feasibility --strict`
  is clean (exit 0).
- **Exposed for the integration checks (task 6.3).** For the cutover: the API
  surface and the route above — raw-bytes fetch via `DownloadedFileType::Uncompressed`,
  `sha2` digest over the whole archive, `tar` + `flate2` extraction on match,
  `make_file_executable` for the executable bit — with the dependency-approval
  state open. For the documentation: initialization options are read by the
  server, no project-file option exists, `project_file` was removed from the
  three examples, and the three schemas carry the reviewed-gate `$comment` —
  state all four identically. G4's state is recorded here and nowhere else;
  `integ-02` reads it from this file.
- **Pin changes (task 6.4).** No pin change occurred during this change. If
  the active pin changes before or while these findings are relied on, the
  probe and both observations are remeasured against the new pin and the
  finding superseded in place, attributed and dated, with the earlier result
  preserved as history.
