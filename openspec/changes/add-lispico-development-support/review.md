# Project review

## Baseline

Reviewed local working trees on 2026-09-26: zed-lisp `10db097`, go-lispico `1d36a9b`, zhk `f9ce4a1`, Yagel `1f0a5757`. zhk and Yagel contain local changes; notably Yagel's dependency version and zhk's prelude below describe the working trees, not only those commits. go-lispico also contains pending dialect proposals. Recheck these surfaces before implementation.

Paths below are relative to each named repository. Findings come from source inspection; no runtime, grammar, or editor tests were run for this review.

## Findings by priority

### High: the existing semantic target does not match the projects

`zed-lisp/extension.toml:13-15` registers sextant for `Common Lisp` only. `languages/commonlisp/config.toml:1-3` associates `.lisp`, `.lsp`, `.cl`, and `.asd` with that language.

Both hosts use go-lispico's **Clojure dialect**, with `stdlib` and `json`: `zhk/internal/engine/engine.go:575-582`, `yagel/internal/core/engine.go:510-535`. Both working-tree `go.mod` files pin `v0.14.0` (`zhk:7`, `yagel:20`). zhk's `.lisp` suffix therefore gives the wrong default language in the current extension.

go-lispico's CL profile is also distinct from ANSI Common Lisp: Lisp-2, `defun`, `setq`, `progn`, `#'`, and `#(...)`, with brackets disabled (`cl/cl.go:201-225`). Clojure is Lisp-1 with bracket/map literals and flat `cond` pairs (`clojure/clojure.go:18`). Both use `~` and `~@`; comma is whitespace, and literals are `nil`, `true`, `false` (`core/reader.go:131-138,267-273,797-812`). `&` is the rest marker. Lists are valid parameter lists; bindings can be nested lists (`core/eval.go:1686-1720`, `core/bindings.go:15`). Full Clojure and ANSI CL assumptions would both introduce false results.

Both `let` and `let*` are sequential in the reviewed runtime (`core/eval.go:1308-1309,1914-1953`); sibling-initializer visibility is explicitly tested (`core/eval_test.go:259-270`). A generic parallel-`let` rule would produce incorrect navigation and diagnostics.

### High: workspace-wide symbol union would give incorrect context

**zhk:** `ZHK_WORKFLOWS` replaces the embedded tree (`internal/engine/engine.go:100-106`). The loader reads `lib/*.lisp` in filename order, then the selected `<route>/main.lisp`. Other route files contribute to the digest but are not evaluated (`:407-431`). Each evaluation starts a fresh runtime (`:573-587`). Route definitions must not leak into other routes; real prelude helpers should remain navigable.

**Yagel:** layer precedence is embedded, packs, global, project; the same relative path selects the last layer. Unreadable winning files suppress older versions, symlinks are not followed, and collection is bounded (`internal/rules/rules.go:121-133`). Each rule loads into its own child scope (`internal/core/routine.go:1403`). The current static checker collects definitions across every rule (`internal/rules/check.go:113-155`), so reusing that global union would hide unresolved references.

Yagel also distinguishes staging claims, live routine bindings, and root values (`internal/core/primitive_names.go:21-56`). `spawn-and-await`, `concurrent-over`, and `per-item-stage` belong to dynamic workflow context, not every rule (`internal/primitives/workflow_prelude.go:12-40`, `internal/core/routine.go:1380-1394`).

### High: no reusable Lispico editor server or complete declaration API exists

go-lispico's CLI executes files or starts a REPL (`cmd/lispico/main.go:28-90`). `make lint` checks Go. `GoFunc` contains `Name` and `Fn`, without signatures, docs, or source locations (`core/types.go:1292`). `PluginMeta` describes plugins, not individual symbols (`core/plugin.go:26`). The stdlib inventory is a useful name-parity source, not a public signature catalog (`internal/inventory/registered.go:8`).

The reader parses without evaluation, but returns runtime values and discards token spans (`core/reader.go:41,757`). Reader columns count bytes (`:106-113`). Editor analysis needs retained ranges, incomplete-source recovery, and conversion to negotiated LSP positions. Compilation alone is not undefined-name checking: unresolved names become global lookups (`core/compiler/compiler.go:170`).

Yagel has useful partial tooling: `rules surface` emits names as Markdown (`internal/cli/rules_surface.go:11-14,46-60`); `rules check` reports path, parse error, and unknown-symbol names without spans (`internal/rules/check.go:15-20`). Default checking reads active layers, including installed content; it does not represent an unsaved editor snapshot (`internal/cli/rules_check.go:85-135`). `docs/dialect-surface.md:10-11` still names v0.13.0, demonstrating version drift.

### High: evaluating files to discover context can execute effects

go-lispico macro expansion evaluates macro bodies (`core/eval.go:1480`); plugin initialization calls host code (`runtime/plugin.go:74`). zhk exposes shell and file operations in its primitive registry (`internal/engine/engine.go:531-533`). Yagel exposes execution, filesystem, and network functions (`internal/primitives/exec.go:90,733,819,889`). Indexing, hover, or lint must not run these operations.

### Medium: host declaration sources exist but need enrichment

zhk's registry provides 31 host names, argument kinds, and min/max arity (`internal/engine/engine.go:513-554,605-624`). For example, `zhk/read` accepts one or two arguments; `zhk/sh` accepts three. Parameter names, docs, return shapes, and definition locations need explicit source-backed declarations.

Yagel's name groups and registration map are authoritative name sources (`internal/core/primitive_names.go:21-56`, `internal/primitives/primitive.go:61-87`). Functions such as `pub`, `recv`, and `exec` have different signatures; `active-role`, `user-home`, and `user-input-topic` must not be represented as generic functions. Effects and claims also have different phase availability. Unknown signatures should remain unknown.

### Medium: current queries and verification do not cover the target dialect

Common Lisp config offers parentheses but no vector/map pairs; word and completion characters omit `/` and `:` (`languages/commonlisp/config.toml:9-17`). Highlight and outline queries are CL-specific (`highlights.scm:18-56`, `outline.scm:3-13`). There are no project fixtures, context catalogs, snippets, or Rust test modules in the inspected extension tree. `Makefile:23-24` and `.github/workflows/ci.yml:21` run unbounded `cargo test`.

### Separate maintenance findings

- `src/common_lisp.rs:54,209-215` propagates release-download errors before Roswell fallback, contrary to the current spec's language-server resolution requirement and Roswell-fallback scenario.
- Roswell failure can end with a misleading claim that Roswell is unavailable (`src/common_lisp.rs:89-119`).
- List text-object interior captures only one child after the head (`languages/commonlisp/textobjects.scm:8-12`), whereas the current spec requires list contents.

These deserve small independent fixes. They are not prerequisites for the new Lispico analysis model and are not silently included in its implementation tasks.

## Representative acceptance sources

| Repository | Source | Required proof |
| --- | --- | --- |
| go-lispico | `internal/goldset/testdata/*.lisp`, `cl/cl_test.go`, `clojure/clojure_test.go` | Match the fixture's actual dialect; exercise aliases, quoting, binding, and literals. |
| go-lispico | `plugins/stdlib/bootstrap.go` | Core macros present without expanding user macros. |
| zhk | `workflows/lib/00-core.lisp` | List parameters/bindings, maps, vectors, `zhk/*` names. |
| zhk | `workflows/plan/main.lisp`, `workflows/lib/20-plan.lisp` | Navigate route calls to prelude definitions; isolate other routes. |
| Yagel | `rules/defaults/doctor.clj`, `rules/defaults/command-plan.clj` | Vector bindings, typed catch, qualified names, staged claims and callbacks. |
| Yagel | `rules/defaults/zapply-flow.clj`, `zreview-flow.clj`, `zsprint-flow.clj` | Large-source indexing with cancellation and bounded work. |
| Yagel | `packs/memory/pack.edn` | Treat manifest as data, never as executable rule source. |

## External contracts checked

- [Zed language extensions](https://zed.dev/docs/extensions/languages): separate language metadata, grammar/query resources, and LSP registration.
- [Zed language configuration](https://zed.dev/docs/configuring-languages): workspace file associations, server selection, binary options, and settings.
- [Zed snippet extensions](https://zed.dev/docs/extensions/snippets): manifest registration and language-based snippet file matching.
- [Clojure grammar source](https://github.com/sogaiu/tree-sitter-clojure/blob/master/grammar.js): candidate structural grammar with lists, maps, vectors, `~`, `~@`, `#'`, and `#(...)` nodes. Its Clojure node names do not establish Lispico semantics. Compatibility remains a test gate, not a completed result.

## Specification verification

- `timeout 60s openspec validate --all --strict --no-interactive --concurrency 2`: 4 passed, 0 failed. Two informational notices concern long existing baseline requirements.
- `git diff --check`: passed. New change documents also passed local-link and trailing-whitespace checks.
- Planning artifacts are complete: six capability deltas, 27 requirements, 66 scenarios, and 31 pending implementation tasks.
- The three existing main specs used `## ADDED Requirements`, which hid their requirements and prevented archiving modifications. Their titles, purposes, and `## Requirements` headers were normalized; existing requirement bodies were preserved.
- No plugin/runtime behavior changed. Grammar, runtime, protocol, and Zed verification remain implementation acceptance tasks.
