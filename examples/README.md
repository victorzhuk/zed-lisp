# Project configuration examples

These templates wire Zed and the Lispico tooling to the three target
projects. Copy the matching directory's contents into the project
repository: `.lispico.json` goes to the worktree root and `.zed/settings.json`
to the project's `.zed/` directory.

## What each template does

| Template | Modes | Language server | Notes |
| --- | --- | --- | --- |
| `zhk/` | `Lispico Clojure` for `workflows/**/*.lisp` | `llsp` | Ordered `lib/*.lisp` prelude; one route's `main.lisp` per program. |
| `yagel/` | `Lispico Clojure` for `rules/**/*.clj` and `packs/**/*.clj` | `llsp` | Embedded → packs → project layers; every rule is an isolated scope. |
| `go-lispico/` | `Lispico Clojure` for goldset fixtures, `Lispico CL` for `corpus/` | `llsp` | Narrow fixture paths only; other `.lisp` files keep the default `Common Lisp` mode. |

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
  per-file digests. The pairing is two-way — `source_files` and
  `source_fingerprint` are either both present or both absent, never one
  alone. A `catalogs[]` reference may separately name a
  `source_root` checkout to verify those paths against — a key the project
  schema has carried since 0.5.0.
- `schemas/lispico-packs.schema.json` (version 1) documents the installed-pack
  snapshot a `packs` layer can lock instead of reading a live directory,
  through the `snapshot` and `expected_fingerprint` pair. The templates ship
  no snapshot file, and nothing in the tree captures one yet. A pack's
  `digest` is an opaque store identity, deliberately unlike the prefixed
  digest `source_digest` of read file content: only equality is defined for
  it.

All three are draft-07 JSON Schemas; point your editor's JSON schema settings
at the shipped copies or at the `$id` URLs.

A packs layer selects either a live `root` or a locked `snapshot` with its
`expected_fingerprint` — never both and never neither. `expected_fingerprint`
is a `sha256:`-prefixed lowercase digest over that document's raw JSON bytes,
not over the source content the snapshot lists; the snapshot schema has no
fingerprint property of its own, so whoever consumes the snapshot is what
hashes the bytes it read.

Validating against the
schema only fixes the document's shape. No code yet reads a `source_root`
checkout, derives a fingerprint, captures a snapshot, or hashes a snapshot's
raw bytes against an `expected_fingerprint` pin; host context resolution,
snapshot ordering, and symlink- and race-safe reads are equally unwritten.
That work belongs to go-lispico
and the host projects. [llsp](https://github.com/victorzhuk/llsp) is the one
language server the extension registers for all three modes; on its pinned
release (`v0.2.1`) it serves the dialect-aware editor analysis but implements
none of the host-context surface above.

## Prerequisites

Semantic features (completion, signatures, navigation, diagnostics) are
served by the shared `llsp` language server. The extension resolves it in
order: a configured binary (`lsp.llsp.binary.path`), then `llsp` on `PATH`,
then a checksum-verified download of the pinned release
([SHA256SUMS](https://github.com/victorzhuk/llsp/releases) verified before
extraction). Published platforms: Linux x86_64, Linux aarch64, macOS x86_64,
macOS aarch64, and Windows x86_64; Windows aarch64 has no published archive.
Without the server, structural editing, highlighting, outline, and text
objects keep working and the resolution failure is reported once when a
buffer opens.
