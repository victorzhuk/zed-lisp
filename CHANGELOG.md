## Unreleased

### Fixed

- Fix the Lispico mode templates and the README association example so their `file_types` patterns select anything in Zed. Zed anchors each pattern against the file extension, the file name, and the worktree-prefixed path, so the shipped patterns (`rules/**/*.clj`, `workflows/**/*.lisp`) matched none of them and left the modes unselected — the association silently did nothing and the `llsp` server never started
- Make the template test model those candidates instead of the repository-relative paths it assumed, and pin the anchored form the templates now ship (`**/rules/**/*.clj` selects, `rules/**/*.clj` does not)

## 0.6.0 (2026-10-05)

### Added

- Add `schemas/lispico-packs.schema.json`, the version-1 shape of an installed-pack snapshot: `source_revision`, `selection_generation`, the installed `packs`, the `entries` each contributed, and the `problems` recorded while reading them. A readable entry carries its pack-relative `source` and `source_digest`; an unreadable one carries neither
- Add `source_files` to the declaration catalog schema: a duplicate-free list of unique relative paths (256 max) paired with the aggregate `source_fingerprint` it was computed over. The pairing is two-way — `source_files` and `source_fingerprint` are either both present or both absent, never one alone. The paths carry no per-file digests
- Add `snapshot` and `expected_fingerprint` to the project schema so a `packs` layer selects either a live `root` or a locked snapshot — never both, never neither. `snapshot` and `expected_fingerprint` are forbidden on the other layer kinds. `expected_fingerprint` is a `sha256:`-prefixed lowercase digest over the snapshot document's raw JSON bytes, not over the source content the snapshot lists; the snapshot schema carries no fingerprint property, so the consumer hashes the bytes it reads. A pack's `digest` stays an opaque store identity, deliberately unlike those prefixed digests
- Add `make acceptance` as the opt-in entry point for the real-server stdio acceptance suite (`tests/llsp_stdio.rs`); the suite selects its candidate from `LLSP_BINARY` or the first `llsp` on `PATH` and still fails loudly on a missing, wrong-version, or non-starting server

### Changed

- Serve all three language modes — `Common Lisp`, `Lispico Clojure`, and `Lispico CL` — through one shared `llsp` language server entry with an explicit language-ID map; the previous `sextant` and `lispico` entries and their resolution code are removed. The extension resolves `llsp` as a configured binary, then `llsp` on `PATH`, then a checksum-verified release download pinned to llsp v0.2.1 (SHA-256 against the published `SHA256SUMS` before extraction, verified offline cache reuse, and one actionable error naming the supported platforms — Linux x86_64, Linux aarch64, macOS x86_64, macOS aarch64, Windows x86_64; Windows aarch64 has no published archive). Roswell, source builds, and installer scripts are gone. Host-aware catalog, library, and phase analysis remains an open upstream gap on this pin (see the README limitations)
- Harden the verified-release install: archive members may extract only inside the cache entry's payload directory (rooted paths, Windows prefixes, and parent traversal are rejected in both the tar and zip extractors), and extraction is bounded by per-member and total size caps. A future re-pin can no longer silently inherit a weaker extraction boundary
- Verify a cache entry before every reuse: the completion state now also records the installed binary's SHA-256, and an entry whose binary no longer matches that digest, or whose recorded asset is not the published archive for the current platform, is re-resolved instead of started
- Keep `make test` hermetic: the real-server acceptance suite is `#[ignore]`-gated and runs through `make acceptance`, so CI and ordinary local runs no longer need a provisioned llsp binary
- Move the project and declaration-catalog schemas to `schema_version: 2` and the three `examples/` templates with them. `source_fingerprint` is now a lowercase SHA-256 instead of any non-empty string, and a `packs` layer no longer requires `root`
- Tighten the packaged-resource check to cover the new snapshot schema alongside the project and catalog schemas. The check asserts only that each declared resource is present and that each shipped schema parses as JSON; the test suite is what compiles and validates documents against the schemas
- Record what the schemas deliberately do not do. No code yet reads a `source_root` checkout, derives a fingerprint, captures an installed-pack snapshot, or hashes a snapshot's raw bytes against an `expected_fingerprint` pin; host context resolution, snapshot ordering, and symlink- and race-safe reads are equally unwritten. That work belongs to go-lispico and the host projects, and [llsp](https://github.com/victorzhuk/llsp) — a separate, future primary Lispico server whose 0.2.1 implements none of the schema-version-2 contracts, including host contexts, and which is not a release of the `lispico-lsp` binary the extension resolves — implements none of it
- Correct the documentation of the `lispico` server prerequisite: the extension resolves a `lispico-lsp` binary you supply, and go-lispico is the runtime the modes and catalogs target rather than a shipped source of that legacy server
- Commit `Cargo.lock` so the shipped cdylib builds reproducibly from pinned dependency versions

## 0.5.2 (2026-09-26)

- Fix sextant server resolution to reuse a previously downloaded binary when the network is unavailable, and remove partial or non-executable downloads so they cannot poison later cache lookups
- Reject release archives whose tag does not match the bundled extension/Cargo versions, and validate that the archive contains only the documented extension resources (manifest, languages, snippets, schemas, examples, README, LICENSE, wasm)
- Tighten the Lispico catalog schema: reject Clojure entries whose `cell` is not `value` and reject value entries that also declare an arity

## 0.5.1 (2026-09-26)

- Fix dev-extension install failing with `data did not match any variant of untagged enum ExtensionSnippets`: `snippets` is now a top-level list of files named after each language's snippet scope (`lispico clojure.json`, `lispico cl.json`), so the snippets also reach their modes
- Fix grammar checkout rejecting the bundled `grammars/` submodules: submodule URLs now match the manifest repositories exactly
- Add bracket matching queries for `Lispico Clojure` and `Lispico CL`
- Build and lint for `wasm32-wasip2`, the target Zed compiles extensions with; package checks now verify snippet scopes and submodule URLs/commits against the manifest

## 0.5.0 (2026-09-26)

- Add opt-in `Lispico Clojure` and `Lispico CL` language modes for go-lispico dialects, with dialect-correct highlighting, brackets, indentation, outline, and text objects. Neither mode claims global file suffixes; ordinary `Common Lisp` associations and sextant behavior are unchanged
- Register the `lispico` language server for the Lispico modes: configured binary path, then `lispico-lsp` on `PATH`, then one actionable error — no download, build, Roswell, or sextant fallback. Unknown server IDs never fall through to sextant
- Add language-scoped snippets for both Lispico modes (valid list/vector parameter and binding conventions)
- Add `.lispico.json` project and declaration-catalog JSON schemas plus tested configuration templates for zhk, Yagel, and go-lispico under `examples/`, with opt-in host-specific snippet examples
- Pin the grammars as submodules (tree-sitter-commonlisp `3232350`, tree-sitter-clojure `e43eff8`) and add a corpus/query verification harness: fixtures from the three target projects plus Common Lisp regressions parse against the pinned grammars, every shipped query compiles, templates and schemas validate, and packaged resources are checked in CI
- Bound test verification: `make test` runs the tests with a 300s wall limit, four build jobs, and four test threads; CI uses the same wrapper and packages the new snippet, schema, and example resources in releases
- Fix sextant download failures to fall through to the Roswell build instead of aborting server resolution
- Fix the error shown when the Roswell build fails: it no longer claims Roswell is unavailable when Roswell ran and did not produce a binary
- Fix Common Lisp list text objects to select the whole list interior instead of only the first child after the head

## 0.4.0 (2026-07-04)

- Download a prebuilt, self-contained `sextant` binary from the latest GitHub release (Linux x64/arm64, Apple Silicon macOS) when it is not on `PATH`, before falling back to a Roswell source build

## 0.3.0 (2026-07-04)

- Fix outline panel missing `defun`/`defmacro`/`defgeneric`/`defmethod` definitions
- Fix "select inside function" to select the body instead of the argument list
- Fix CI to run on pushes to `master`
- Add Lisp-aware word selection and completion for symbols like `foo-bar`, `*special*`
- Add highlighting for definition names, call-position symbols, and more special forms
- Add comment continuation for `;;` and `;;;` prefixes
- Stop `"` and `|` auto-closing inside strings and comments
- Switch the language server from cl-lsp to [sextant](https://github.com/victorzhuk/sextant); the extension resolves it from `PATH`, then builds the latest master via `ros install victorzhuk/sextant` when Roswell is available

## 0.2.0 (2025-05-22)

- Add GitHub Actions CI workflow (fmt, clippy, test, wasm build)
- Add tag-based GitHub Release publishing for `v*` tags
- Fix clippy lint in error message (format! → .into())
- Fix textobjects capture names to current Zed conventions
- Update LSP command resolution to forward args/env on all paths
- Fix Roswell install failure handling (check exit status)
- Update specs and README

## 0.1.0 (2025-02-24)

- Initial Common Lisp language support for Zed
- Syntax highlighting via tree-sitter-commonlisp
- LSP integration via cl-lsp with Roswell fallback
- Bracket matching, indentation, outline, text objects
