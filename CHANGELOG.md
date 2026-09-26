## Unreleased

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
