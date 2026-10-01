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

All three shipped templates use `schema_version: 2`. The extension ships the
schemas that validate them:

- `schemas/lispico-project.schema.json` (version 2) documents and validates
  `.lispico.json`: contexts with their `files` globs, `dialect`, and
  `profile`; the `prelude` a `zhk` context orders, and the `layers` a Yagel
  context overlays.
- `schemas/lispico-catalog.schema.json` (version 2) documents the declaration
  catalogs referenced by `catalogs`, including the `source_version` or
  `source_revision` and the `source_fingerprint`/`source_files` pair.
  `source_files` is a duplicate-free list of unique relative paths standing
  for the checkout `source_fingerprint` was computed over; it holds no
  per-file digests. A `catalogs[]` reference may separately name a
  `source_root` checkout to verify those paths against — a key the project
  schema has carried since 0.5.0.
- `schemas/lispico-packs.schema.json` (version 1) documents the installed-pack
  snapshot a `packs` layer can lock instead of reading a live directory,
  through the `snapshot` and `expected_fingerprint` pair. The templates ship
  no snapshot file, and nothing in the tree captures one yet.

All three are draft-07 JSON Schemas; point your editor's JSON schema settings
at the shipped copies or at the `$id` URLs.

A packs layer selects either a live `root` or a locked `snapshot` with its
`expected_fingerprint` — never both and never neither. Validating against the
schema only fixes the document's shape. No code yet reads a `source_root`
checkout, derives a fingerprint, captures a snapshot, or recomputes a digest
over raw payloads; host context resolution, snapshot ordering, and symlink-
and race-safe reads are equally unwritten. That work belongs to go-lispico
and the host projects, and [llsp](https://github.com/victorzhuk/llsp) 0.2.1,
the currently released `lispico-lsp`, implements none of it.

## Prerequisites

Semantic features (completion, signatures, navigation, diagnostics)
require the native `lispico-lsp` server from go-lispico and the catalogs
each template references. Without them, structural editing, highlighting,
outline, and text objects keep working and the missing dependency is
reported once when a Lispico buffer opens.
