# Proposal

## Why

The extension currently provides Common Lisp syntax and sextant integration. The target projects use go-lispico, including a Clojure dialect in both zhk's `.lisp` workflows and Yagel's `.clj` rules. They need their actual dialect, libraries, host bindings, and loading scopes represented in Zed to make completion, navigation, and diagnostics reliable.

## What Changes

- Add opt-in `Lispico Clojure` and `Lispico CL` language modes with dialect-specific highlighting, brackets, indentation, outline, text objects, and snippets. Preserve ordinary Common Lisp and sextant.
- Add explicit workspace context for dialect, runtime version, enabled libraries, host declarations, and source visibility. Supply portable examples for go-lispico, zhk, and Yagel.
- Supply inert, versioned declarations for Go-defined symbols; index actual Lisp libraries for documentation and navigation. Keep zhk and Yagel bindings separate.
- Introduce a native Lispico language server and matching static checker, sharing analysis in go-lispico. These are new upstream deliverables, not capabilities of the current runtime CLI.
- Support completion, hover, signature help, definitions, references, symbols, and ranged diagnostics on unsaved buffers without executing source, macros, plugins, or host effects.
- Add bounded verification for dialect syntax, context isolation, catalog drift, LSP behavior, packaging, and Common Lisp regressions.

No extension identity change or global reassignment of `.lisp`, `.clj`, or `.edn` is proposed. Implementation starts only after approval of the scope and verification plan.

## Capabilities

### New Capabilities

- `lispico-language-support`: opt-in dialect modes and structural editing.
- `lispico-project-context`: portable configuration, library visibility, host profiles, and declaration catalogs.
- `lispico-language-server`: server launch, semantic editor features, and worktree isolation.
- `lispico-static-diagnostics`: side-effect-free, dialect-aware lint shared by editor and CLI.

### Modified Capabilities

- `common-lisp-language-support`: preserve default recognition while allowing explicit workspace dialect associations.
- `github-actions-ci-release`: bounded tests, grammar/query and integration checks, complete packaging of new resources.

## Impact

- **zed-lisp:** `extension.toml`, `src/common_lisp.rs`, new language/query/snippet resources, project examples, `Makefile`, CI, `README.md`, and existing `CHANGELOG.md` during implementation.
- **go-lispico, prerequisite:** source-aware static analysis, inert core/plugin declarations, native server/checker commands, protocol tests. No evaluator redesign or runtime metadata hot-path change is required.
- **zhk, prerequisite:** host declarations derived from the primitive registry and an ordered-prelude context description.
- **Yagel, prerequisite:** host declarations classified by binding kind and execution phase; context descriptions matching rule layers and per-file scope.
- Runtime and host changes require separately approved work in their owning repositories. This change records their contracts and dependencies; this proposal does not modify those repositories.
- Existing Yagel `rules check` remains an independent host check. Its names-only results and cross-file symbol union are insufficient as the editor's analysis model.

See [review](review.md) for evidence and [design](design.md) for boundaries, rollout, and verification.
