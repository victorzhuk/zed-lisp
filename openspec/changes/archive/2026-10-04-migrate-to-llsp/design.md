# Design

## Context

The archived change `add-lispico-development-support` (archived 2026-10-01 as `2026-10-01-add-lispico-development-support`) planned a language server and batch checker inside go-lispico (`lispico-lsp`, `lispico-check`) to serve the two opt-in Lispico modes. Its entire server-launch requirement is a no-download, no-Roswell adapter over a binary that does not exist. The canonical Lispico baselines it prepared now exist under `openspec/specs/lispico-*`, and this change is the sole normative delta carrier for the two server-integration capabilities (see the cutover's [amendment map](../2026-10-04-migrate-to-llsp-cutover/amendment.md)).

`llsp` (`github.com/victorzhuk/llsp`, Apache-2.0, Rust) already ships those dialects. Verified at `v0.2.0`, `88e3e72`:

- `dialects/common-lisp.toml:52` declares `language_ids = ["lisp", "commonlisp", "common-lisp"]`.
- `dialects/lispico-clojure.toml:2` and `dialects/lispico-cl.toml:2` declare `language_ids = ["lispico-clojure"]` and `["lispico-cl"]`, and **no** `extensions` key — they are selectable by language ID or `files.associations` only.
- `llsp` is the stdio LSP server; `check`/`format`/`config`/`dialects` are batch subcommands.
- Release `v0.2.0` (2026-09-30) publishes `llsp-{aarch64,x86_64}-apple-darwin.tar.gz`, `llsp-{aarch64,x86_64}-unknown-linux-musl.tar.gz`, `llsp-x86_64-pc-windows-msvc.zip`, and `SHA256SUMS`. Archives contain `llsp-<target>/llsp`. There is no Windows arm64 asset.
- Diagnostic codes are `unused-binding`, `duplicate-definition`, `unresolved-call` — a different set from the parent's planned one.

Design depth: Standard. Verification mode: **existing-service-strict**. This is a migration onto an existing published server, not a new analyzer.

## Blocking findings

**Superseded history — 2026-10-02.** Everything below is a measurement of llsp
`v0.2.0` (`88e3e72`), code that no longer ships. The current gate authority is
[`migrate-to-llsp-upstream-gates/gates.md`](../2026-10-04-migrate-to-llsp-upstream-gates/gates.md):
on `v0.2.1` (`436bc84`) G1 is **met** (the language-ID retention fix is
observed live), and the host-aware parity and analysis gaps (F3) remain **unmet
and block release**. These `v0.2.0` findings are preserved as the history that
motivated this change; no current gate state is inherited from them.

F1 is reproduced on two independent `llsp 0.2.0` binaries (the released binary and a local release build), with identical output. F2 and F3 are not upstream defects: F2 is an extension-side mapping requirement, and F3 is an absence of host-context capability. They are separated here because only F1 blocks on another repository.

**F1 — `workspace/didChangeConfiguration` discards the client language ID.** `reload_documents` (`src/server.rs:548-564`) rebuilds every open document and calls `Settings::detect(path, None, text)` (`:553`), dropping step 2 of the detection order (`src/document.rs:227-255`). `Document` (`src/document.rs:20-29`) has no `language_id` field; `did_open` reads it (`:447-461`) and discards it. A `lispico-cl` buffer opened on `buf.lisp` publishes two `invalid-syntax` diagnostics at didOpen, then publishes **none** after one accepted settings change. Not self-healing: repeated config changes, ordinary edits, and `default_dialect` do not restore it. Only a `files.associations` entry or close/reopen does. Controls pinned by extension (`clojure`/`.clj`, `racket`/`.rkt`) do not flip, so the defect is specific to language-ID-only dialects. The same `None` appears at `src/workspace.rs:234` and `src/main.rs:216,309`, so the on-disk scan path has no language ID available at all and always resolves those dialects by path and text. This is recorded as a code fact; the user-visible consequences beyond F1 are not measured by this change.

**F2 — the derived wire identifier is not a dialect identifier.** Zed's fallback for a language server entry with no `language_ids` map is a value derived from the display name by `language_name.lsp_id()`, which normalizes the name rather than copying it (`crates/language_core/src/language_name.rs:48-53`). `by_language_id` (`src/dialect.rs:444-448`) matches only the exact strings each dialect declares — `lisp`, `commonlisp`, `common-lisp` for Common Lisp, and `lispico-clojure` / `lispico-cl` for the two Lispico dialects (`dialects/*.toml:2`, `dialects/common-lisp.toml:52`). So this is an identifier-shape mismatch, not a lowercase/casing bug: a derived value may coincide with a declared identifier by accident of normalization, and no such coincidence is a guarantee. Which of the three modes actually fails to match is determined by task 1.2, not asserted here. The extension fixes this on its own side by declaring `[language_servers.llsp.language_ids]` explicitly for all three modes; no upstream change is involved. F1 and F2 are independent — the map makes the identifier correct on the wire, and F1 is that the server drops it again on a settings change.

**F3 — no host-aware contract upstream.** llsp has no catalog, host-profile, layer, or phase vocabulary; that is zed-lisp's own model, and the zhk/Yagel catalogs it needs are still unbuilt. The 3.x/4.x parent tasks stay open.

## Decisions

### 1. One server, one entry, explicit identifiers

Register `[language_servers.llsp]` with `languages = ["Common Lisp", "Lispico Clojure", "Lispico CL"]` plus the `language_ids` map. The three modes share one registration and one resolution chain instead of two registrations and two chains; how many processes Zed spawns for a given session is Zed's own behavior and is not claimed here.

Rejected: keeping `sextant` alongside `llsp` for Common Lisp. Two analyzers over the same dialect, two resolution chains, and the user choosing a Common Lisp server without knowing which is which. The baseline `Code label formatting` requirement is removed rather than retargeted — the extension ships no label hook for the shared server, and leaving it would assert unverified formatting behavior.

Rejected: any shim, alias entry, silent fallback to the old servers, or reopen-on-settings-change workaround. The cutover is one atomic change; if a gate fails, the cutover does not ship.

### 2. Resolution: config → PATH → verified download → actionable error

Exactly the chain in the specs. Two structural departures from the sextant chain, both deliberate:

- **No Roswell.** `ros install` is a source build of a different project, needs a Common Lisp toolchain, and is unrelated to a Rust binary that publishes prebuilt archives. The parent's "preserve Roswell" framing came from the baseline Common Lisp contract, not from a Lispico requirement.
- **No `install.sh`.** The extension has no `process:exec` capability, so shelling out is unavailable regardless of policy. The digest check and extraction therefore run in-process, subject to the feasibility result in decision 3.

The error text names the supported platforms, the stopping reason, and the three remedies. No invented endpoints: the release asset list, the `SHA256SUMS` entry, and the archive member layout are read from what the release actually publishes.

### 3. Feasibility first: digest and extraction in the WASM host

`zed_extension_api` exposes `download_file`, `make_file_executable`, `latest_github_release`, and `github_release_by_tag_name`. `download_file` has built-in download-and-extract support via `DownloadedFileType` — it accepts `DownloadedFileType::GzipTar`, `DownloadedFileType::Zip`, and `DownloadedFileType::Uncompressed`. The crate provides no hashing helper, so the digest-before-extraction workflow is the open question.

The feasibility task therefore asks a single question: whether the host can compute SHA-256 over a multi-megabyte archive before any of its contents are trusted and then extract it safely. The workflow to be proven is raw download → SHA-256 check against the published `SHA256SUMS` entry → safe extraction, using the API that is already available or a vetted dependency for whichever step needs one — the same mechanism the extension already uses for its existing dependencies. This change does **not** mandate a specific crate, does not propose hand-rolling either hashing or extraction, and does not require any change to the `zed_extension_api` surface. Any dependency that would be added is subject to the repository's normal reviewed-approval process before it lands. If no such route works in the sandbox, the design returns for approval — the digest requirement is never weakened to "download and hope", and the change never claims the download path works without proof.

### 4. `.lispico.json` stays a reviewed gate, not a shipped option

llsp reads no `.lispico.json` and there is no upstream proposal for one. The parent change's `project_file` initialization option and its JSON schemas therefore **cannot** be assumed supported by llsp. This was verified by the feasibility child: llsp reads initialization options but accepts no project-file setting (its project file is `.llsp.toml` at the workspace root), so `project_file` was removed from the example settings and the three shipped schemas carry the reviewed-gate `$comment` marking — recorded in [`migrate-to-llsp-host-feasibility/feasibility.md`](../2026-10-04-migrate-to-llsp-host-feasibility/feasibility.md), which owns that answer.

Rejected: filtering unknown initialization options out before sending them. That would silently discard user configuration, and the cutover explicitly prohibits config wrapping, config suppression, and any "reopen so the server picks it up" trick. An option the server rejects is surfaced as a configuration error.

### 5. The predecessor is amended against the canonical baseline, not rewritten

The predecessor change is archived (`2026-10-01-add-lispico-development-support`), and the canonical Lispico baselines it prepared now exist under `openspec/specs/`. The amendment of its server identities, merged launch ownership, contradictory no-download/Roswell wording and analyzer/checker ownership is audited against those canonical baselines in the cutover's [amendment map](../2026-10-04-migrate-to-llsp-cutover/amendment.md), which records each class as already satisfied in the canonical text and authors no duplicate delta. Catalog and declaration ownership stays with go-lispico/zhk/Yagel.

Untouched: dialect syntax, context selection, catalog and library requirements, zhk route order, Yagel layers and phases, `.lispico.json` schema intent, opt-in suffix semantics, task completion marks, upstream task ownership and the review's source-evidence pass. No requirement is deleted, no open task is marked done, and no catalog-owned capability is silently reassigned to llsp.

### 6. Archive shape

The open parent `migrate-to-llsp` is the **sole normative delta carrier** for `llsp-language-server-integration` and `common-lisp-language-server-integration` (and any audited canonical predecessor amendment); the four no-delta children of the split own no capability delta, and no child's record restates a requirement. Archive ordering is **by change**, never by capability name: `migrate-to-llsp-upstream-gates`, then `migrate-to-llsp-host-feasibility`, then `migrate-to-llsp-cutover`, then `migrate-to-llsp-acceptance`, then this parent **once** — applying both capability deltas in the same operation — and `migrate-to-llsp-docs-release` last as the close-out record. No archive command names a capability, no `--skip-specs`, no `--no-validate`, no force. `MODIFIED` deltas against the four `lispico-*` capabilities are deliberately absent: their canonical baselines already carry the llsp-aware wording (see the cutover's amendment map), and a duplicate header would collide at archive.

The new capability's `## Purpose` is written out (not left `TBD`) so no hand-edit of `openspec/specs/` is needed after archive.

## Gates before cutover

Each gate's current state lives in exactly one authoritative record; this
table points at them and holds no second copy:

| Gate | Requirement | Authoritative record |
|---|---|---|
| G1 Language-ID retention across settings reload | `Dialect identity is stable for the life of a buffer` | [`migrate-to-llsp-upstream-gates/gates.md`](../2026-10-04-migrate-to-llsp-upstream-gates/gates.md) — met on `v0.2.1` (`436bc84`) |
| G2 Wire identifier shape matches a declared dialect ID | `Explicit language identifier map` | [`migrate-to-llsp-cutover/design.md`](../2026-10-04-migrate-to-llsp-cutover/design.md), Identifier-map evidence |
| G3 Host-aware context, catalog, phase parity | `Host-aware release gates` | [`migrate-to-llsp-upstream-gates/gates.md`](../2026-10-04-migrate-to-llsp-upstream-gates/gates.md) — unmet on `v0.2.1`, blocks release |
| G4 Digest and extraction feasibility in the WASM host | `llsp binary resolution precedence` | [`migrate-to-llsp-host-feasibility/feasibility.md`](../2026-10-04-migrate-to-llsp-host-feasibility/feasibility.md), Gate consequence — met with dependency approval |
| G5 Real-server proof | Full proof tasks | [`migrate-to-llsp-acceptance/acceptance.md`](../2026-10-04-migrate-to-llsp-acceptance/acceptance.md), Real-server proof status |

**Implementation admission** — a separately recorded authorization that
explicitly admits an atomic unreleased development cutover while G3 and G5
are open, plus a current passing G1 measurement, verified dialect identifiers
and release contract, and G4 met with any required dependency approval — is
what admits the cutover's implementation. **Release admission** — G1–G5 met
on compatible evidence — is what admits any release tag, marketplace
publication, release package publication, or release-readiness claim. The two
admissions are different boundaries, and neither is satisfied by a passing
check. No gate is satisfied by shipping compensating configuration:
`files.associations` remains an optional user measure, never a required
setting or a substitute for the gate, and falling back to suffix or
`default_dialect` selection is prohibited.

## Authorization

This change was approved as specification preparation; specification
preparation alone authorizes no implementation. A separate authorization —
recorded in [`migrate-to-llsp-cutover/design.md`](../2026-10-04-migrate-to-llsp-cutover/design.md)
(§Authorization record) — names who granted it, when, and that it explicitly
admits the atomic unreleased development cutover while G3 and G5 remain open;
the cutover's implementation started only under that recorded authorization.
Release admission stays separate: no release tag, marketplace publication,
release package publication, or release-readiness claim is authorized by any
record until G1–G5 are met on compatible evidence.

## Verification

`existing-service-strict`, in increasing cost:

1. **Feasibility probe** — SHA-256 and archive extraction over a real release archive in the sandbox, discarded afterward.
2. **Behavior contract tests, in-process** — resolution precedence, argument/environment forwarding on every path, digest mismatch, extraction failure, cache pruning, interrupted-download state, offline reuse, unsupported platform, unknown server ID. Real code paths, no stubs. Deliberately not a wiring-copy check: each case must be able to fail by a real defect.
3. **Real-server LSP smoke** — drive an actual `llsp` binary over stdio: initialize, `didOpen` for each of the three language IDs, then an accepted `workspace/didChangeConfiguration`, asserting the dialect is unchanged **and** that `lispico-cl` reader-invalid diagnostics are still published. Both signals are required: hover alone can be null for unrelated reasons, diagnostics alone are empty for clean files. The control dialects pin extension-implied selection and must not flip.
4. **Actual Zed acceptance** — all three modes in a live session: unsaved-buffer behavior, multibyte and supplementary Unicode before an error, save/reload, server restart, two worktrees side by side with isolated catalogs, and plain Common Lisp unaffected. Recorded with versions, actions, and observed results.
5. **Bounded full checks plus `openspec validate --strict`.**

No fake `initialize` language ID: the identifier belongs to `didOpen`. No stub server anywhere in the proof chain.

## Risks and rollback

- **G1 never clears** — the cutover does not ship. `files.associations` remains an optional user measure, not a documented requirement of the shipped configuration.
- **G4 fails** — design returns for approval; the digest requirement is not weakened.
- **Analysis parity is not reached** — llsp's `unresolved_call` lint defaults to off, and it has no lint for missing libraries, catalog identity, or phase violations. This is the current state of the upstream gap between what the parent's contract requires and what any available server provides. It blocks host-context parity (G3); it is not an approved reduction, and nothing in this change accepts a reduced outcome in its place.
- **Rollback** — revert the server entry, map, and resolution changes together. Modes, queries, snippets, schemas, and examples are unaffected, because they are owned by the parent change and are not modified here.

## Cross-repository ownership

This change records the upstream gates; it does not patch llsp, go-lispico, zhk, or Yagel. Runtime and host changes require separately approved work in their owning repositories. Existing Yagel `rules check` remains an independent host check and is not presented as ranged editor analysis.
