# Proposal

## Why

The extension currently provides Common Lisp syntax and sextant integration. The target projects use go-lispico, including a Clojure dialect in both zhk's `.lisp` workflows and Yagel's `.clj` rules. They need their actual dialect, libraries, host bindings, and loading scopes represented in Zed to make completion, navigation, and diagnostics reliable.

## What Changes

- Add opt-in `Lispico Clojure` and `Lispico CL` language modes with dialect-specific highlighting, brackets, indentation, outline, text objects, and snippets. Preserve ordinary Common Lisp; its server entry is now the shared `llsp` server.
- Add explicit workspace context for dialect, runtime version, enabled libraries, host declarations, and source visibility. Supply portable examples for go-lispico, zhk, and Yagel.
- Supply inert, versioned declarations for Go-defined symbols; index actual Lisp libraries for documentation and navigation. Keep zhk and Yagel bindings separate.
- Serve all three modes from the existing `llsp` language server instead of the previously proposed native go-lispico server, and pair it with a matching static checker where one is still required. llsp is a published upstream deliverable; only the host-aware analysis it does not yet provide remains an open upstream prerequisite. See [migrate-to-llsp](../migrate-to-llsp/proposal.md), which owns server registration, language identifiers, resolution, cache, and configuration forwarding for all modes.
- Support completion, hover, signature help, definitions, references, symbols, and ranged diagnostics on unsaved buffers without executing source, macros, plugins, or host effects.
- Add bounded verification for dialect syntax, context isolation, catalog drift, LSP behavior, packaging, and Common Lisp regressions.

No extension identity change or global reassignment of `.lisp`, `.clj`, or `.edn` is proposed. Implementation starts only after approval of the scope and verification plan.

## Capabilities

### New Capabilities

- `lispico-language-support`: opt-in dialect modes and structural editing.
- `lispico-project-context`: portable configuration, library visibility, host profiles, and declaration catalogs.
- `lispico-language-server`: semantic editor features and worktree isolation. Server registration, language identifiers, resolution, cache, and configuration forwarding for all three modes moved to the `llsp-language-server-integration` capability in [migrate-to-llsp](../migrate-to-llsp/proposal.md).
- `lispico-static-diagnostics`: side-effect-free, dialect-aware lint shared by editor and CLI.

### Modified Capabilities

- `common-lisp-language-support`: preserve default recognition while allowing explicit workspace dialect associations.
- `github-actions-ci-release`: bounded tests, grammar/query and integration checks, complete packaging of new resources.

## Impact

- **zed-lisp:** `extension.toml`, `src/common_lisp.rs`, new language/query/snippet resources, project examples, `Makefile`, CI, `README.md`, and existing `CHANGELOG.md` during implementation.
- **llsp, prerequisite and gate owner:** v0.2.1 at `436bc84` retains the client-reported language ID across settings reload, verified on the checksum-matched official binary. Explicit language-ID mapping remains extension work. Host-aware context, catalog, library, and phase parity remains unmet; [migrate-to-llsp](../migrate-to-llsp/proposal.md) owns the separate server cutover.
- **go-lispico, prerequisite:** source-aware static analysis, inert core/plugin declarations, and protocol tests. The previously proposed native server/checker commands are superseded by the published `llsp` server; the host-aware analysis llsp does not yet provide remains an open prerequisite. No evaluator redesign or runtime metadata hot-path change is required.
- **zhk, prerequisite:** host declarations derived from the primitive registry and an ordered-prelude context description.
- **Yagel, prerequisite:** host declarations classified by binding kind and execution phase; context descriptions matching rule layers and per-file scope.
- Runtime and host changes require separately approved work in their owning repositories. This change records their contracts and dependencies; this proposal does not modify those repositories.
- Existing Yagel `rules check` remains an independent host check. Its names-only results and cross-file symbol union are insufficient as the editor's analysis model.

See [review](review.md) for evidence and [design](design.md) for boundaries, rollout, and verification.

## Approved ownership split

This change remains the capability-contract owner and integration umbrella. Its completed tasks remain historical evidence, not proof that upstream dependencies are finished.

- [add-lispico-runtime-core-catalogs](../add-lispico-runtime-core-catalogs/proposal.md): original task 2.4; go-lispico owns runtime declarations.
- [add-lispico-dialect-analyzer-checker](../add-lispico-dialect-analyzer-checker/proposal.md): original tasks 2.1–2.3 and 2.5; llsp owns analysis/checking with runtime parity fixtures.
- [add-zhk-host-profile](../add-zhk-host-profile/proposal.md): original task 3.2 and the zhk portion of 3.4.
- [add-yagel-host-profile](../add-yagel-host-profile/proposal.md): original task 3.3 and the Yagel portion of 3.4.
- [add-llsp-host-context](../add-llsp-host-context/proposal.md): original tasks 4.1–4.6.
- [add-zed-lisp-acceptance](../add-zed-lisp-acceptance/proposal.md): original tasks 6.1–6.4.

Original tasks 3.1 (separate upstream approval) and 6.5 (final readiness) remain open here. Child records do not duplicate capability deltas and do not authorize edits outside this repository. The parent cannot complete or archive until every dependency has real implementation evidence and integrated acceptance passes. Splitting work does not reduce the scope.
