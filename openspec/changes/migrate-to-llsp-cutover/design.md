# Design

## Authorization record (section 0 of the tasks)

- **0.1 — The implementation authorization.** Granted by Victor Zhuk, the
  repository owner, on 2026-10-02, as the active session goal for this
  repository: *"complete all changes, verify and commit each change, plan
  first."* The instruction explicitly directs completing every recorded
  change of the `migrate-to-llsp` split. The split's own records define its
  single implementation boundary — an **atomic unreleased development
  cutover** (registration and removal as one change) — so the authorization
  is recorded as explicitly admitting that boundary **while G3 and G5 remain
  open**. It contains no release tag, no marketplace publication, no release
  package publication, and no release-readiness claim; those stay blocked
  until G1–G5 are met on compatible evidence.
- **0.2 — The active server pin.** llsp `v0.2.1`
  (`436bc84c0f520c424d6b7a1c086f38e1ce0448e8`), the release this
  implementation and its G1/G4 evidence were measured against. The dialect
  identifiers and the release contract (five archives plus `SHA256SUMS`,
  member layout `llsp-<target>/llsp`, Windows aarch64 unsupported) are the
  ones the baseline record verified for that release.
- **0.3 — Current gate measurements this change consumes.** G1: met on
  `v0.2.1`, run `c6b8142f`, recorded in
  `../migrate-to-llsp-upstream-gates/gates.md` (§G1 measurement). G4: met —
  route proven, dependency approval recorded — in
  `../migrate-to-llsp-host-feasibility/feasibility.md` (§Gate consequence).
  G3: unmet and a release blocker, in the baseline record. G5: open, owned by
  the acceptance child. Each state is read from its owner record; none is
  restated as a second authority here.
- **0.4 — The unreleased boundary.** This change lands as one atomic change:
  single registration plus single removal together, no release tag, no
  marketplace publication, no release package publication, no
  release-readiness claim. The acceptance child runs against this landed,
  unreleased state.

## Context

`extension.toml:18-24` registers `[language_servers.sextant]` over `Common Lisp` and `[language_servers.lispico]` over the two Lispico modes. `src/common_lisp.rs` routes between them in `dispatch_language_server`, which maps the two IDs onto a `ServerKind` and rejects everything else. The two arms then diverge completely:

- `lispico_command` → `resolve_lispico_command`: configured path, then `which("lispico-lsp")`, then a fixed error. `LISPICO_SERVER_BINARY` is `lispico-lsp` (`src/common_lisp.rs:9`).
- `sextant_command`: configured path, then `which("sextant")`, then `download_sextant`, then a Roswell build. `download_latest_sextant` maps three targets to `sextant-linux-x64`, `sextant-linux-arm64`, `sextant-macos-arm64` and returns `None` for anything else; the asset is fetched with `zed::download_file(..., DownloadedFileType::Uncompressed)` and made executable, with no digest check against any checksum file. `latest_cached_sextant_dir` accepts a directory named `sextant-<version>` whose `sextant` file passes `executable_file`, which on non-Unix treats any regular file as ready. `label_for_completion` formats labels only when the server ID is `sextant`.

The example templates name the second server: `examples/{zhk,yagel,go-lispico}/.zed/settings.json` bind each mode's `language_servers` to `["lispico"]` and set `lsp.lispico.initialization_options.project_file`. `scripts/check_package.py` validates that every language named in a server entry has a `languages/<dir>/config.toml` and that example `file_types` keys are real language names; it does not look at `language_ids` and does not require a single server entry. `make test` depends on `check-package`, and CI runs both `make test` and `python3 scripts/check_package.py`, so the packaging check is already on both the local and the CI path. The release job packs `extension.toml`, `languages/`, `snippets/`, `schemas/`, `examples/`, `README.md`, `LICENSE` and the wasm, and validates those members.

Two premises are consumed, not re-derived, here. The identifiers and the release contract — asset names, `SHA256SUMS` entry, archive member layout, supported platforms — come from the baseline record taken against the release this split pins, not from the parent change's `v0.2.0` documents. The digest-before-extraction mechanism comes from the feasibility child's recorded route; a negative result there returns the parent design for approval and this change does not proceed on a weakened digest requirement.

Implementation admission is not release admission. This change lands atomically in an explicitly unreleased development state under a separately recorded authorization that admits exactly that boundary: current G1 passing, verified identifiers and release contract, G4 met with its dependency approval. G2 is then established by this change's own map, packaging and dispatch evidence, and G5 by the acceptance child afterwards. G3 remains an external release blocker until the separately owned llsp-side work ships and a compatible release is re-measured locally, and it does not block this unreleased implementation. No release tag, marketplace publication, release package publication, or release-readiness claim is permitted until G1–G5 are met on compatible evidence. Specification approval alone authorizes nothing here.

Normative delta ownership is likewise fixed rather than negotiated per change. The open parent `migrate-to-llsp` is the sole delta carrier for `llsp-language-server-integration`, `common-lisp-language-server-integration`, and any audited canonical predecessor amendment; this child authors implementation and evidence and writes no competing `MODIFIED` requirement body of its own.

OVERSIZE: These rows are one atomic cutover by construction — the single `[language_servers.llsp]` registration, the explicit `language_ids` map, the deletion of the `sextant` and `lispico` entries, the three-step resolver, the forwarding paths and their behavior tests only exist together, and any subset lands the extension in a state the packaging check and `dispatch_language_server` are written to reject. Splitting them would leave the cutover's own Identifier-map evidence section cited by the baseline and documentation records as the authoritative G2 record while it is incomplete, and would let the resolver's error text name platforms from a release contract verified against a different pin than the one the cache and download rows install.

## Decisions

### 1. Registration and removal are one edit, not two

`[language_servers.llsp]` and the deletion of `[language_servers.sextant]` and `[language_servers.lispico]` land in the same change, with the dispatch table reduced to the single ID. No commit, branch or review state exists in which both the old entries and the new one are live, and no state exists in which neither is.

Rejected: registering `llsp` first and removing the old entries in a follow-up. Between the two, a `Common Lisp` buffer has two registered servers and the user has no way to know which one answers; a settings reload during that window produces evidence that belongs to neither design. Rejected: keeping `sextant` for `Common Lisp` — two analyzers over the same dialect, two resolution chains, and the label-formatting branch kept alive for a server that is no longer registered.

### 2. The identifier map lives in the manifest

`[language_servers.llsp.language_ids]` is declared in `extension.toml` with all three modes mapped explicitly. The extension does not compute, normalize, or patch an identifier in Rust.

Rejected: deriving the wire ID in the extension from the display name. That reproduces the defect it is meant to remove — the derived form is a normalization whose agreement with a declared dialect ID is an accident. Rejected: sending the identifier on `initialize`. The identifier belongs to `didOpen`; a server that only honors the open-time ID is unaffected by what the client claims at initialize.

#### Identifier-map evidence

This section is the single authoritative G2 record. Missing evidence of any
kind means G2 is not met; no other file holds or copies this state, and
upstream-gates' index links here rather than restating it.

- **Extension build identifier:** `common-lisp` version `0.5.2`
  (`extension.toml` / `Cargo.toml`), the landed build this change compiles.
- **Exact declared map** (`extension.toml`, the one `language_servers` entry):
  `[language_servers.llsp]` with `name = "llsp"`,
  `languages = ["Common Lisp", "Lispico Clojure", "Lispico CL"]`, and
  `[language_servers.llsp.language_ids]` mapping `"Common Lisp" = "lisp"`,
  `"Lispico Clojure" = "lispico-clojure"`, `"Lispico CL" = "lispico-cl"` —
  the identifiers the baseline record read from the dialects themselves
  (`dialects/lispico-cl.toml:2`, `dialects/lispico-clojure.toml:2`,
  `dialects/common-lisp.toml:52` at `436bc84`). No identifier is computed,
  normalized, or patched in Rust.
- **Both packaging mutations fail without the map** (proven by the mutations,
  not the clean run — `tests/config.rs`):
  - `check_package_fails_when_a_language_has_no_identifier_entry` removes the
    `"Lispico CL" = "lispico-cl"` line; `scripts/check_package.py` exits 1
    naming `no language_ids entry`.
  - `check_package_fails_when_a_previous_server_entry_survives` re-adds a
    `[language_servers.sextant]` entry; the checker exits 1 naming the server
    and `only 'llsp'`.
  - `check_package_fails_when_the_llsp_entry_is_absent` drops every
    `language_servers` entry; the checker exits 1 naming the absent
    `'llsp'` server.
- **Dispatch evidence** (`src/tests.rs`): `dispatch_accepts_only_llsp`;
  `dispatch_rejects_both_previous_server_ids` asserts `sextant`, `lispico`
  and `lispico-lsp` are rejected as unknown; `unknown_server_ids_name_the_one_registered_server`
  asserts an unknown ID's error names the one registered server.
- **Resulting G2 result: met.** Every language in the single server entry
  carries an explicit wire identifier equal to a declared dialect identifier,
  the extension derives nothing, both previous server IDs are rejected as
  unknown, and the packaging check turns any reintroduction into a packaging
  failure.

### 3. Digest before extraction, or nothing

On the download path the archive is fetched, its SHA-256 is compared with the `SHA256SUMS` entry for the selected asset, and only a match is extracted. A mismatch, an extraction failure, or a failure to set the executable bit leaves no entry a later attempt can start.

Rejected: extract-then-hash, which runs arbitrary archive content on the host before its integrity is known. Rejected: best-effort hashing — a mismatch that starts the binary anyway is a checksum requirement in name only, and it is the outcome the parent design refuses. Rejected: substituting another platform's archive when the current one has none; that is a silent wrong-binary delivery, and the correct result is the documented error.

### 4. Cache completeness is recorded, not inferred

The cache holds one directory per release version. An entry is usable only when it is complete and verified: the extracted binary exists, is executable, and the entry's recorded completion state says the digest check, extraction and executable bit all finished. The completion state is written last. A download, digest or extraction failure removes what it wrote rather than leaving a directory that a later scan could mistake for an install.

Rejected: the current heuristic — a directory named `<server>-<version>` containing a file that passes `executable_file`. It cannot tell a verified install from an interrupted one, and `executable_file` returns `true` for any regular file on a platform with no POSIX permission bits, so on the wasm host the check collapses to "a file is there". Rejected: repairing or completing a partial entry in place instead of discarding and re-resolving; a partial entry has no verified provenance to complete from.

### 5. Offline reuse is reuse of a verified entry only

After configured binary and `PATH` miss, reuse a complete verified cache entry for the active version and platform without network access. If none exists, attempt the supported platform's verified release download. With an empty or unusable cache and an offline/unreachable release, return the documented actionable error; do not retry indefinitely or start another version. Digest mismatch and extraction failure remain fatal for that attempt.

A cache miss is therefore not a stopping condition on its own: with an empty cache and a reachable release, resolution reaches verified download and the first installation succeeds. Only an offline or unreachable release, or another specified failure that prevents installation, ends resolution with the error.

Rejected: treating any absent cache entry as terminal, which would make a first installation unreachable. Rejected: falling back to the newest cached directory whatever its state, as the current `latest_cached_sextant_dir` does. Rejected: searching for a different version offline and starting it while reporting the version the user asked for.

### 6. Configuration reaches the server as written

Arguments and environment from `LspSettings` are attached to the command on every path; `language_server_initialization_options` returns `lsp_settings.initialization_options` and `language_server_workspace_configuration` returns `lsp_settings.settings` unchanged, as they already do. No buffer is reopened and no server restarted to force configuration through.

Rejected: filtering unknown initialization options out of the outgoing request. It silently discards user configuration and hides a real mismatch behind a working session. The feasibility child records whether llsp accepts a project-file initialization option; if it does not, the option is removed from the example templates, and in every case it is the user's option and it is sent as written — a server that rejects it surfaces as a configuration error.

### 7. The invariants are enforced where the code already is

Two checks, both on existing paths. `scripts/check_package.py` — already run by `make test` and by CI — fails when a language in a server entry has no `language_ids` entry, and fails when any `language_servers` entry other than `llsp` is present, so a reintroduced `sextant` or `lispico` entry is a packaging failure rather than a review opinion. `src/tests.rs` asserts the dispatch rejects both previous IDs as unknown and that an unknown ID reports the one registered server.

Rejected: a `TODO`, a code comment, or a manual review checklist as the guard. The failure mode this prevents — a per-mode entry surviving the cutover, or a new mode added without an identifier — is silent when nothing fails.

### 8. The package gains no resource

The release member list is re-checked against what the manifest now needs and is left as it is: the extension registers a server it resolves at runtime and ships no server binary, so the archive gains nothing. The check records that conclusion rather than adding an entry.

Rejected: vendoring an llsp binary or a generated checksum table into the tarball. That would pin a platform the package does not target and duplicate a record that changes every release.

Re-checked on 2026-10-02: the release job's member list (`.github/workflows/ci.yml` — `extension.toml`, `languages/`, `snippets/`, `schemas/`, `examples/`, `README.md`, `LICENSE`, the compiled wasm) is exactly what the manifest now needs; the shared server is resolved at runtime, so the list is unchanged and the check records that conclusion.

### 9. The predecessor is amended against the canonical baseline, not by renaming history

`2026-10-01-add-lispico-development-support` is immutable archived history. It is not rewritten, re-marked, or re-archived; its completion marks are what they are. The canonical specs under `openspec/specs/` are the real baseline: `lispico-language-server/spec.md:8-28` already assigns shared routing and launch ownership to `llsp-language-server-integration` while keeping the buffer-observable isolation requirement, and `lispico-static-diagnostics/spec.md` under "Shared editor and batch results" already assigns llsp batch ownership and records host-aware parity as an unmet external prerequisite.

The open parent `migrate-to-llsp` is the delta carrier for anything still missing; this child is the author of those amendments. That makes the work an audit-and-gap exercise across four amendment classes — server identities, the merged launch requirement, the conflicting no-download/Roswell wording, and analyzer/checker ownership — in which a class whose canonical wording is already correct yields no delta at all. Catalog and declaration ownership stays with go-lispico/zhk/Yagel, and llsp owns only behavior it actually supplies.

Preserved without exception across any amendment: semantic requirements, source-evidence requirements, data ownership, every existing completion mark, and every preserved scenario.

Rejected: a blind rename of the archived files. History is not where a contract is corrected; the correction belongs in a delta against the current baseline. Rejected: re-adding wording the canonical specs already state correctly, which produces duplicate requirement bodies competing at archive time.

## Verification

`existing-service-strict`, in-process against the real resolution code. No stub server, no recorded transcript.

1. `scripts/check_package.py` fails on a manifest that drops one `language_ids` entry, and on a manifest that still declares `[language_servers.sextant]`. Both are proven by the mutation, not by the passing clean run.
2. Behavior contract tests in `src/tests.rs` over the real resolver: configured binary wins with no `which` call and no download; `PATH` hit wins with no release lookup; a verified download caches, marks executable and starts; with an empty cache and a reachable release, a first installation downloads, verifies and installs; digest mismatch produces no usable entry; extraction failure produces no installed state; an interrupted download leaves nothing startable; a non-executable cache entry is not started and re-resolves; installing a version prunes older version directories; offline reuse starts a complete verified entry with no release lookup; offline with no usable current entry returns the documented error; an unsupported platform returns the supported list rather than another platform's archive; an unknown server ID and both previous IDs are rejected.
3. Forwarding assertions on all four paths — configured binary, `PATH`, verified download, verified cache — that the exact argument list and environment reach the returned command, that initialization options and workspace settings are returned unmodified, and that a rejected initialization option surfaces as a configuration error.
4. `make test` and CI's `python3 scripts/check_package.py`, both bounded by the existing wall-clock limits.
5. Common Lisp non-regression: the same recognition, highlighting, outline, text objects and settings surface, now reached through the shared entry.

Each test asserts observable resolution behavior — which binary was selected, what state the cache is in, what the error names. None asserts that a value was copied, that a collection grew, or that a result is non-empty.

## Risks and rollback

- **Gate state not current.** This change consumes gate records; it clears none. A gate state inherited from the parent's `v0.2.0` documents is not evidence for it. Implementation is admitted only under a separately recorded authorization for an atomic unreleased development cutover with G1 current and passing and G4 met with its dependency approval; G2 is established here by the identifier-map evidence above and G5 by the acceptance child. G3 and G5 may still be open at that point, and no release tag, publication, or release-readiness claim may be made until G1–G5 are met on compatible evidence. An authorization that does not explicitly admit that boundary leaves implementation blocked.
- **Feasibility answer negative.** The design returns for approval; the digest requirement is not weakened and the download path does not ship.
- **Removal under-reaches.** A surviving `lispico-lsp` string, cache directory or test would leave two servers partially live. The packaging check and the dispatch tests are the detectors, and neither accepts the previous state.
- **Rollback.** Revert the manifest, resolver and example changes as one commit. Grammars, queries, snippets and `.lispico.json` examples are untouched by this change, so a rollback leaves them exactly as they are. The three `.lispico.json` schema resources were handled by the feasibility child's reviewed metadata marking; cutover neither changes nor rolls back that predecessor work.
