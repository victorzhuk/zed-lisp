# Common Lisp for Zed

Common Lisp and Lispico language support for Zed: syntax highlighting, Tree-sitter powered parsing and structural editing, and language server integration via sextant (Common Lisp) or a separately supplied `lispico-lsp` binary (Lispico dialects).

## Features

- **Common Lisp** (`.lisp`, `.lsp`, `.cl`, `.asd`): syntax highlighting, bracket matching, auto-indentation, outline panel, and language server support via [sextant](https://github.com/victorzhuk/sextant).
- **Lispico Clojure** and **Lispico CL** (opt-in): go-lispico dialect modes with dialect-correct highlighting, `()`/`[]`/`{}` structure (Clojure), list/reader-vector structure (CL), outline, text objects, and scoped snippets. They never claim file suffixes globally — selection happens through workspace `file_types` associations or manual language selection.
- **Lispico language server** (`lispico`): completion, hover, signatures, navigation, and static diagnostics from a `lispico-lsp` binary you supply, resolved independently of sextant.

## Prerequisites

### Common Lisp (sextant)

On Linux (x64/arm64) and Apple Silicon macOS, the extension downloads a self-contained [sextant](https://github.com/victorzhuk/sextant) binary from GitHub releases automatically — no other setup required.

On other platforms (e.g. Intel macOS), or to build from source, install [Roswell](https://github.com/roswell/roswell) and the extension will build the latest sextant on first launch. You can also install it ahead of time:

```sh
ros install victorzhuk/sextant
```

Make sure `~/.roswell/bin` is on your PATH.

### Lispico modes (`lispico-lsp`)

The Lispico server is an existing `lispico-lsp` binary you supply yourself — the extension only resolves and launches it, and never builds or downloads it. The go-lispico runtime is a verified source of the Lispico runtime the modes and catalogs target, but it does not itself ship this legacy server binary, so there is nothing to install from it automatically. The extension resolves the binary in this order and never falls back to sextant, Roswell, a download, or a build:

1. Configured binary path (Zed settings, `lsp.lispico.binary`)
2. `lispico-lsp` on `PATH`

When the server is absent, structural editing, highlighting, outline, and text objects keep working and the missing dependency is reported once per buffer. Semantic features (completion, hover, signatures, navigation, diagnostics) additionally require a [`.lispico.json`](#lispico-project-configuration) project context and the declaration catalogs it references; those catalogs are owned and shipped by the upstream projects (go-lispico, zhk, Yagel) and are not yet published — until they are, Lispico buffers get syntax-level support only.

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

The schemas fix document structure only. Nothing yet reads a `source_root` checkout, derives a `source_fingerprint` from a checkout, captures an installed-pack snapshot, or hashes a snapshot's raw bytes to check an `expected_fingerprint` pin. Host context resolution (`zhk` prelude, Yagel layers), snapshot ordering, and symlink- and race-safe filesystem reads are also unwritten. All of it belongs to go-lispico and the host projects. [llsp](https://github.com/victorzhuk/llsp) is a separate, future primary Lispico server — at its current 0.2.1 it implements none of the schema-version-2 contracts, including host contexts, and it is not a release of the `lispico-lsp` binary resolved above. A configuration that therefore validates still gets syntax-level Lispico support only until that work lands.

### Server settings

```json
{
  "lsp": {
    "lispico": {
      "binary": {
        "path": "/path/to/lispico-lsp",
        "arguments": [],
        "env": {}
      },
      "initialization_options": {
        "project_file": ".lispico.json"
      },
      "settings": {}
    }
  }
}
```

Server CLI options are owned by the upstream go-lispico change and are intentionally not documented here.

Configured arguments and environment apply to both the configured path and the PATH-resolved binary. Initialization options and workspace settings are forwarded unchanged to the server.

## Configuration (Common Lisp / sextant)

### Custom Binary Path

If sextant is installed in a non-standard location, you can specify the path in your Zed settings:

```json
{
  "lsp": {
    "sextant": {
      "binary": {
        "path": "/path/to/sextant"
      }
    }
  }
}
```

### Custom Arguments

Pass additional arguments to the language server:

```json
{
  "lsp": {
    "sextant": {
      "binary": {
        "arguments": ["--port", "8080"]
      }
    }
  }
}
```

### Initialization Options

Pass initialization options to the language server:

```json
{
  "lsp": {
    "sextant": {
      "initialization_options": {
        "some-option": "value"
      }
    }
  }
}
```

### Workspace Settings

Configure workspace-specific settings:

```json
{
  "lsp": {
    "sextant": {
      "settings": {
        "workspace-setting": "value"
      }
    }
  }
}
```

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

### Project Structure

```
├── Cargo.toml             # Rust extension manifest
├── extension.toml         # Zed extension manifest
├── src/
│   └── common_lisp.rs     # Extension implementation
├── languages/
│   ├── commonlisp/        # Common Lisp language configuration
│   ├── lispico-clojure/   # Lispico Clojure mode
│   └── lispico-cl/        # Lispico CL mode
├── snippets/              # Language-scoped snippets
├── schemas/               # .lispico.json, declaration catalog, and installed-pack snapshot JSON schemas
├── examples/              # Project templates (zhk, yagel, go-lispico)
├── grammars/              # Pinned grammar submodules + locally built wasm
├── tests/                 # Corpus, query, and configuration verification
└── LICENSE
```

### Architecture

The extension is built as a WebAssembly module using the Zed extension API:

- **`zed_extension_api`** — Provides the `Extension` trait that the extension implements to handle language server lifecycle and configuration
- **Server dispatch** — The extension dispatches by server ID before any resolution. Unknown IDs never fall through to sextant. Resolution order:
  - `lispico` (Lispico Clojure, Lispico CL): configured path → `lispico-lsp` on PATH → actionable error. No download, build, Roswell, or sextant fallback.
  - `sextant` (Common Lisp): configured path (with optional args/env) → PATH → prebuilt binary from the latest [sextant GitHub release](https://github.com/victorzhuk/sextant/releases) → Roswell build (`ros install victorzhuk/sextant`), then PATH lookup
- **Tree-sitter grammars** — [tree-sitter-commonlisp](https://github.com/tree-sitter-grammars/tree-sitter-commonlisp) (pinned `3232350`) for Common Lisp, [tree-sitter-clojure](https://github.com/sogaiu/tree-sitter-clojure) (pinned `e43eff8`) for both Lispico modes; both are registered in `extension.toml` and vendored as pinned submodules for the test harness
- **Verification** — `make test` parses the corpus fixtures from the three target projects plus Common Lisp regressions against the pinned grammars, compiles every shipped query, and validates the templates and schemas against them. The packaged-resource check (`scripts/check_package.py`, also runnable alone as `make check-package`) only asserts that each declared resource is present and that each shipped schema parses as JSON; the tests are what actually compile and validate against the schemas

## Links

- [sextant](https://github.com/victorzhuk/sextant) — Common Lisp Language Server Protocol implementation
- [go-lispico](https://github.com/victorzhuk/go-lispico) — the Lispico runtime the `Lispico` modes and their declaration catalogs target
- [llsp](https://github.com/victorzhuk/llsp) — a separate, future primary Lispico language server, not a release of the `lispico-lsp` binary resolved above; at 0.2.1 it implements none of the schema-version-2 contracts
- [tree-sitter-commonlisp](https://github.com/tree-sitter-grammars/tree-sitter-commonlisp) — Tree-sitter grammar for Common Lisp
- [tree-sitter-clojure](https://github.com/sogaiu/tree-sitter-clojure) — structural grammar used by the Lispico modes
- [Roswell](https://github.com/roswell/roswell) — Common Lisp environment setup utility

## License

Apache-2.0
