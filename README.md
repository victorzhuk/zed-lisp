# Common Lisp for Zed

Common Lisp and Lispico language support for Zed: syntax highlighting, Tree-sitter powered parsing and structural editing, and language server integration via one shared [llsp](https://github.com/victorzhuk/llsp) entry for all three modes.

## Features

- **Common Lisp** (`.lisp`, `.lsp`, `.cl`, `.asd`): syntax highlighting, bracket matching, auto-indentation, outline panel, and language server support.
- **Lispico Clojure** and **Lispico CL** (opt-in): go-lispico dialect modes with dialect-correct highlighting, `()`/`[]`/`{}` structure (Clojure), list/reader-vector structure (CL), outline, text objects, and scoped snippets. They never claim file suffixes globally — selection happens through workspace `file_types` associations or manual language selection.
- **One shared language server** (`llsp`): all three modes are served by a single registered `llsp` entry with an explicit language-ID map (`Common Lisp` → `lisp`, `Lispico Clojure` → `lispico-clojure`, `Lispico CL` → `lispico-cl`), so every buffer reaches the server with its declared dialect identifier.

## Prerequisites

### Language server (llsp)

The extension resolves `llsp` in a fixed order and never falls back to anything else:

1. **Configured binary path** — Zed settings, `lsp.llsp.binary.path`
2. **`llsp` on `PATH`**
3. **Checksum-verified release download** — the pinned llsp release's `SHA256SUMS` entry is fetched, the platform's archive is downloaded as raw bytes, its SHA-256 is computed and compared **before anything is extracted**, and only a matching archive is extracted and installed into a version-named cache entry. A mismatched digest, a failed extraction, or an interrupted download leaves nothing that a later attempt can start.

Published platforms: **Linux x86_64, Linux aarch64, macOS x86_64, macOS aarch64, and Windows x86_64**. Windows aarch64 has no published archive and is unsupported — another platform's archive is never substituted for it.

Installing a release prunes older cache versions. With no usable cache entry and an unreachable release, resolution stops with one error naming the supported platforms, the reason it stopped, and the remedies (configure a binary path, expose `llsp` on `PATH`, or place a verified binary in the cache).

When the server is absent, structural editing, highlighting, outline, and text objects keep working and the resolution failure is reported separately. Semantic analysis comes from the server; the host-aware analysis described by the Lispico project context is currently an open upstream gap (see [Limitations](#limitations)).

Note: Yagel's existing `rules check` remains an independent host-side check; it is not the editor's language server and does not provide ranged or unsaved-buffer diagnostics.

## Installation

1. Clone this repository with its grammar submodules:
   ```bash
   git clone --recurse-submodules https://github.com/victorzhuk/zed-lisp.git
   cd zed-lisp
   ```

2. Build the extension:
   ```bash
   cargo build --release --target wasm32-wasip2
   ```

3. In Zed, open the command palette (Cmd/Ctrl + Shift + P)
4. Run "Install Dev Extension"
5. Select the cloned directory

## Lispico modes

### Selecting a mode

Lispico modes are opt-in. Add a workspace `file_types` association in the project's `.zed/settings.json`, for example for a zhk checkout:

```json
{
  "file_types": {
    "Lispico Clojure": ["workflows/**/*.lisp"]
  }
}
```

Ready-made templates for all three target projects live in [`examples/`](examples/README.md) — zhk, Yagel, and go-lispico — each with a `.zed/settings.json`, a `.lispico.json` context configuration, and opt-in host snippets. Copy them into the target repository.

You can also select **Editor: Set Language** manually on any buffer.

### Lispico project configuration

`.lispico.json` at the worktree root describes the dialect, host profile, enabled libraries, catalogs, and source visibility. Three draft-07 JSON Schemas ship with the extension and are validated in CI:

- [`schemas/lispico-project.schema.json`](schemas/lispico-project.schema.json) — `schema_version: 2`. Requires `schema_version` and a non-empty `contexts` array; each context names its `files` globs, `dialect` (`cl` or `clojure`), and host `profile` (`runtime`, `zhk`, `yagel-rule`, `yagel-workflow`). `prelude` belongs to the `zhk` profile, ordered `layers` to the Yagel profiles, and the `runtime` profile takes neither. A `catalogs[]` reference may name a `source_root` checkout used to verify that catalog's declared files — that key has been part of the schema since 0.5.0 and is unrelated to `source_files` below.
- [`schemas/lispico-catalog.schema.json`](schemas/lispico-catalog.schema.json) — `schema_version: 2`. An inert declaration catalog pinned to exactly one of `source_version` or `source_revision`. `source_files` is a duplicate-free list of unique relative paths, paired with the aggregate `source_fingerprint` it was derived from; the paths carry no per-file digests of their own. The pairing is two-way: `source_files` and `source_fingerprint` are either both present or both absent, never one without the other.
- [`schemas/lispico-packs.schema.json`](schemas/lispico-packs.schema.json) — `schema_version: 1`. The inert shape of an installed-pack snapshot: `packs`, the `entries` each pack contributed, and the `problems` recorded while reading them. A pack's `digest` is an opaque store identity — only equality is defined, and it is deliberately not a content fingerprint. A readable entry carries its pack-relative `source` and `source_digest`; an unreadable one carries neither. A `packs` layer selects either a live `root` or a locked `snapshot` with its `expected_fingerprint` — never both, never neither. `expected_fingerprint` is a `sha256:`-prefixed lowercase digest of the snapshot document's raw JSON bytes, not of any source content the snapshot lists; the snapshot schema holds no fingerprint property, so the consumer hashes the bytes it reads.

See the [examples README](examples/README.md) for a walkthrough.

The configuration is declarative and never executed. Relative paths resolve from the configuration file's directory; there is no ancestor or home-directory scan.

#### What the schemas do not do

The schemas fix document structure only. Nothing yet reads a `source_root` checkout, derives a `source_fingerprint` from a checkout, captures an installed-pack snapshot, or hashes a snapshot's raw bytes to check an `expected_fingerprint` pin. Host context resolution (`zhk` prelude, Yagel layers), snapshot ordering, and symlink- and race-safe filesystem reads are also unwritten. All of it belongs to go-lispico and the host projects. The pinned llsp release (`v0.2.1`) does not consume these portable context resources; consumption is a reviewed migration gate recorded in the extension's change documents, and the schemas carry that marking as `$comment` metadata.

## Server settings

```json
{
  "lsp": {
    "llsp": {
      "binary": {
        "path": "/path/to/llsp",
        "arguments": [],
        "env": {}
      },
      "initialization_options": {},
      "settings": {}
    }
  }
}
```

- `binary.path` takes precedence over `PATH`, and both take precedence over the verified release download.
- Configured `binary.arguments` and `binary.env` are attached to the resolved command on every resolution path.
- `initialization_options` and `settings` are forwarded to the server unchanged — nothing is wrapped, renamed, filtered, or defaulted, and no buffer is reopened or server restarted to force configuration through. An option the server rejects is surfaced as a configuration error.

Server CLI options are owned by the upstream llsp project and are intentionally not documented here. The server accepts no project-file initialization option on the pinned release; its own project file (`.llsp.toml`) is read from the workspace root, and the shipped `.lispico.json` schemas remain a required portable contract whose consumption by llsp is a reviewed migration gate.

## Limitations

Limitations below are recorded against the pinned llsp release (`v0.2.1`, commit `436bc84`) as measured in the extension's gate records.

- **Host-aware analysis is absent (open gate G3).** llsp has no catalog, host-profile, layer, or phase vocabulary: there is no lint for missing libraries, for catalog identity, or for phase violations, and `unresolved_call` defaults to off. This is an unmet upstream capability that blocks host-context parity — it is not an approved reduction, and ordinary Common Lisp results are not host-aware results. What the acceptance sessions observed on this release is dialect-aware behavior for the three dialects — identifiers, diagnostics, and hover-dialect identity across settings changes; the server's remaining analysis surface is owned upstream and is not claimed here.
- **Verified on this release (gates G1, G2, G4, G5).** Language-ID retention across settings changes was observed live on `v0.2.1` (G1) — *historical note, superseded: on llsp `v0.2.0` a settings change discarded the client-reported dialect of a `.lisp` buffer, and a user-supplied `files.associations` entry was mentioned as an optional mitigation while that was the case; no mitigation setup is required or recommended.* The explicit language-ID map is registered and enforced (G2), the verified-download route is proven with the shipped dependencies (G4), and the real-server and editor acceptance sessions are complete (G5): three-mode selection, diagnostics across settings changes, unsaved-edit updates, save/reload, server restart, two-worktree isolation, multibyte/supplementary-Unicode ranges, and the missing/failing-server separation with structural editing intact.

## Development

### Building from Source

1. Clone the repository with submodules (the pinned grammar sources):
   ```bash
   git clone --recurse-submodules https://github.com/victorzhuk/zed-lisp.git
   cd zed-lisp
   ```

2. Build the WebAssembly extension:
   ```bash
   cargo build --release --target wasm32-wasip2
   ```

3. Install as dev extension in Zed (see Installation section)

### Bounded verification

Tests run through a wrapper with a finite wall-clock limit and explicit worker caps; the same limits apply locally and in CI. A failing test or a timeout both fail the run:

```sh
make test          # bounded: 300s build and test limits, 4 build jobs, 4 test threads
```

Override the bounds when needed: `make test BUILD_TIMEOUT_SECONDS=600 TEST_TIMEOUT_SECONDS=600`.

The real-server acceptance tests in `tests/llsp_stdio.rs` drive a real `llsp` binary over stdio; they validate the candidate against the pinned release before running and fail loudly when no pinned binary is available (`LLSP_BINARY` selects it; it defaults to the documented install path).

### Project Structure

```
├── Cargo.toml             # Rust extension manifest
├── extension.toml         # Zed extension manifest
├── src/
│   └── common_lisp.rs     # Extension implementation: dispatch, resolution chain, forwarding
├── languages/
│   ├── commonlisp/        # Common Lisp language configuration
│   ├── lispico-clojure/   # Lispico Clojure mode
│   └── lispico-cl/        # Lispico CL mode
├── snippets/              # Language-scoped snippets
├── schemas/               # .lispico.json, declaration catalog, and installed-pack snapshot JSON schemas
├── examples/              # Project templates (zhk, yagel, go-lispico)
├── grammars/              # Pinned grammar submodules + locally built wasm
├── tests/                 # Corpus, query, configuration, and real-server verification
└── LICENSE
```

### Architecture

The extension is built as a WebAssembly module using the Zed extension API:

- **`zed_extension_api`** — Provides the `Extension` trait that the extension implements to handle language server lifecycle and configuration
- **Server dispatch** — The extension registers a single `[language_servers.llsp]` entry for `Common Lisp`, `Lispico Clojure`, and `Lispico CL`, with an explicit `language_ids` map; the wire identifier is never computed or normalized in Rust. Dispatch rejects every other server ID, and the packaging check fails any manifest that reintroduces a previous server or drops an identifier. Resolution order (fixed):
  - configured binary path from `lsp.llsp.binary`
  - `llsp` found through `PATH`
  - a complete verified cache entry for the pinned version and platform (reused offline, with zero network access)
  - the verified release download: fetch the archive as raw bytes, SHA-256 it against the published `SHA256SUMS` entry, extract only on a match, mark the binary executable through the host API, and record the entry's completion state last
- **Tree-sitter grammars** — [tree-sitter-commonlisp](https://github.com/tree-sitter-grammars/tree-sitter-commonlisp) (pinned `3232350`) for Common Lisp, [tree-sitter-clojure](https://github.com/sogaiu/tree-sitter-clojure) (pinned `e43eff8`) for both Lispico modes; both are registered in `extension.toml` and vendored as pinned submodules for the test harness
- **Verification** — `make test` parses the corpus fixtures from the three target projects plus Common Lisp regressions against the pinned grammars, compiles every shipped query, validates the templates and schemas, exercises the resolution chain's behavior contract (precedence, digest mismatch, interrupted downloads, pruning, offline reuse, unsupported platforms), and runs the real-server stdio acceptance suite against the pinned llsp build. The packaged-resource check (`scripts/check_package.py`, also runnable alone as `make check-package`) additionally fails a manifest whose server entry lacks an explicit language identifier or that declares any server other than `llsp`

## Links

- [llsp](https://github.com/victorzhuk/llsp) — the shared language server for all three modes
- [go-lispico](https://github.com/victorzhuk/go-lispico) — the Lispico runtime the `Lispico` modes and their declaration catalogs target
- [tree-sitter-commonlisp](https://github.com/tree-sitter-grammars/tree-sitter-commonlisp) — Tree-sitter grammar for Common Lisp
- [tree-sitter-clojure](https://github.com/sogaiu/tree-sitter-clojure) — structural grammar used by the Lispico modes

## License

Apache-2.0
