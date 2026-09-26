# Project configuration examples

These templates wire Zed and the Lispico tooling to the three target
projects. Copy the matching directory's contents into the project
repository: `.lispico.json` goes to the worktree root and `.zed/settings.json`
to the project's `.zed/` directory.

## What each template does

| Template | Modes | Language server | Notes |
| --- | --- | --- | --- |
| `zhk/` | `Lispico Clojure` for `workflows/**/*.lisp` | `lispico` | Ordered `lib/*.lisp` prelude; one route's `main.lisp` per program. |
| `yagel/` | `Lispico Clojure` for `rules/**/*.clj` and `packs/**/*.clj` | `lispico` | Embedded → packs → project layers; every rule is an isolated scope. |
| `go-lispico/` | `Lispico Clojure` for goldset fixtures, `Lispico CL` for `corpus/` | `lispico` | Narrow fixture paths only; other `.lisp` files keep the default `Common Lisp` mode. |

Ordinary Common Lisp files keep their existing `Common Lisp` associations.
Lispico modes never claim `.lisp`, `.lsp`, `.cl`, `.asd`, `.clj`, or `.edn`
globally: selection happens only through these workspace `file_types`
entries or manual language selection.

## Host-specific snippets (opt-in)

`zhk/snippets/` and `yagel/snippets/` contain host-specific snippet
examples. They are **not** shipped as automatic suggestions: zhk contexts
must not offer Yagel declarations and vice versa. Copy the desired file
into your own extension's snippet directory if you want it available by
default; otherwise rely on catalog-driven completion from the language
server.

## Configuration schema

`schemas/lispico-project.schema.json` documents and validates
`.lispico.json`; `schemas/lispico-catalog.schema.json` documents the
declaration catalogs referenced by `catalogs`. Both are draft-07 JSON
Schemas; point your editor's JSON schema settings at the shipped copies or
at the `$id` URLs.

## Prerequisites

Semantic features (completion, signatures, navigation, diagnostics)
require the native `lispico-lsp` server from go-lispico and the catalogs
each template references. Without them, structural editing, highlighting,
outline, and text objects keep working and the missing dependency is
reported once when a Lispico buffer opens.
