# Common Lisp for Zed

Common Lisp language support for Zed with syntax highlighting, LSP integration via sextant, and Tree-sitter powered parsing.

## Features

- **Syntax highlighting** for Common Lisp source files (.lisp, .lsp, .cl, .asd)
- **Tree-sitter powered parsing** with bracket matching and auto-indentation
- **Outline panel** showing defun/defmacro/defclass definitions
- **Language server support** via sextant (completion, hover, go-to-definition, etc.)

## Prerequisites

- [SBCL](https://www.sbcl.org/) (Steel Bank Common Lisp) installed
- [Roswell](https://github.com/roswell/roswell) installed

The extension resolves the [sextant](https://github.com/victorzhuk/sextant) language server from `PATH`. If it is missing and Roswell is available, the extension builds the latest master on first launch. You can also install it ahead of time:

```sh
ros install victorzhuk/sextant
```

Make sure `~/.roswell/bin` is on your PATH.

## Installation

1. Clone this repository:
   ```bash
   git clone https://github.com/victorzhuk/zed-lisp.git
   cd zed-lisp
   ```

2. Build the extension:
   ```bash
   cargo build --target wasm32-wasip1
   ```

3. In Zed, open the command palette (Cmd/Ctrl + Shift + P)
4. Run "Install Dev Extension"
5. Select the cloned directory

## Configuration

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

1. Clone the repository:
   ```bash
   git clone https://github.com/victorzhuk/zed-lisp.git
   cd zed-lisp
   ```

2. Build the WebAssembly extension:
   ```bash
   cargo build --target wasm32-wasip1
   ```

3. Install as dev extension in Zed (see Installation section)

### Project Structure

```
├── Cargo.toml           # Rust extension manifest
├── extension.toml       # Zed extension manifest
├── src/
│   └── common_lisp.rs   # Extension implementation
├── languages/
│   └── commonlisp/      # Language configuration
│       ├── config.toml  # Language metadata
│       ├── highlights.scm   # Syntax highlighting queries
│       ├── brackets.scm     # Bracket matching rules
│       ├── indents.scm      # Indentation rules
│       ├── outline.scm      # Outline panel queries
│       ├── textobjects.scm  # Text object queries
│       ├── overrides.scm    # Scope overrides (string/comment)
│       └── injections.scm   # Language injection queries
└── LICENSE
```

### Architecture

The extension is built as a WebAssembly module using the Zed extension API:

- **`zed_extension_api`** — Provides the `Extension` trait that the extension implements to handle language server lifecycle and configuration
- **Language server resolution** — The extension searches for sextant in this order:
  1. User-configured path from Zed settings (with optional args/env)
  2. Binary on system PATH (with optional args/env)
  3. Roswell build of the latest master (`ros install victorzhuk/sextant`), then PATH lookup
- **Tree-sitter grammar** — Uses [tree-sitter-commonlisp](https://github.com/tree-sitter-grammars/tree-sitter-commonlisp) for parsing Common Lisp syntax

## Links

- [sextant](https://github.com/victorzhuk/sextant) — Common Lisp Language Server Protocol implementation
- [tree-sitter-commonlisp](https://github.com/tree-sitter-grammars/tree-sitter-commonlisp) — Tree-sitter grammar for Common Lisp
- [Roswell](https://github.com/roswell/roswell) — Common Lisp environment setup utility

## License

Apache-2.0
