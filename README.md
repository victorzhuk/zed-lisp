# Common Lisp for Zed

Common Lisp language support for Zed with syntax highlighting, LSP integration via cl-lsp, and Tree-sitter powered parsing.

## Features

- **Syntax highlighting** for Common Lisp source files (.lisp, .lsp, .cl, .asd)
- **Tree-sitter powered parsing** with bracket matching and auto-indentation
- **Outline panel** showing defun/defmacro/defclass definitions
- **Language server support** via cl-lsp (completion, hover, go-to-definition, etc.)
- **Support** for both manual and Roswell-managed cl-lsp installation

## Prerequisites

- [SBCL](https://www.sbcl.org/) (Steel Bank Common Lisp) installed
- [Roswell](https://github.com/roswell/roswell) (recommended for automatic cl-lsp installation)
- Or manually installed [cl-lsp](https://github.com/cxxxr/cl-lsp) on PATH

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

Install `cl-lsp` with roswell:

```sh
$ yay -S tree-sitter-cli
$ ros install qlot lem-project/lem lem-project/micros lem-project/lem-mailbox cxxxr/cl-lsp
```

If cl-lsp is installed in a non-standard location, you can specify the path in your Zed settings:

```json
{
  "lsp": {
    "cl-lsp": {
      "binary": {
        "path": "/path/to/cl-lsp"
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
    "cl-lsp": {
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
    "cl-lsp": {
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
    "cl-lsp": {
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
│       └── injections.scm   # Language injection queries
└── LICENSE
```

### Architecture

The extension is built as a WebAssembly module using the Zed extension API:

- **`zed_extension_api`** — Provides the `Extension` trait that the extension implements to handle language server lifecycle and configuration
- **Language server resolution** — The extension searches for cl-lsp in this order:
  1. User-configured path from Zed settings
  2. Binary on system PATH
  3. Roswell installation (`~/.roswell/bin/cl-lsp`)
- **Tree-sitter grammar** — Uses [tree-sitter-commonlisp](https://github.com/tree-sitter-grammars/tree-sitter-commonlisp) for parsing Common Lisp syntax

## Links

- [cl-lsp](https://github.com/cxxxr/cl-lsp) — Common Lisp Language Server Protocol implementation
- [tree-sitter-commonlisp](https://github.com/tree-sitter-grammars/tree-sitter-commonlisp) — Tree-sitter grammar for Common Lisp
- [Roswell](https://github.com/roswell/roswell) — Common Lisp environment setup utility

## License

Apache-2.0
