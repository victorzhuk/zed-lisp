## Why

Zed does not currently have a first-class Common Lisp extension. The existing third-party `zed-cl` extension bundles a custom embedded LSP and REPL infrastructure, making it heavyweight and tightly coupled to SBCL. Following the pattern established by official Zed language extensions (e.g., Lua, Zig, Erlang), a lean extension that wires up an external language server and provides Tree-sitter–based editing queries gives Common Lisp users a maintainable, composable IDE experience in Zed.

### Existing Ecosystem

- **Tree-sitter grammar**: [`tree-sitter-grammars/tree-sitter-commonlisp`](https://github.com/tree-sitter-grammars/tree-sitter-commonlisp) — mature grammar (v0.4.1, 56 stars), used by difftastic and Neovim.
- **Language servers**: [`cl-lsp`](https://github.com/cxxxr/cl-lsp) (236 stars, MIT, supports go-to-definition, completion, hover, formatting) and [`alive-lsp`](https://github.com/nobody-famous/alive-lsp) (111 stars, Unlicense). Both require a CL runtime (typically SBCL) and Roswell or Quicklisp.
- **Existing Zed extension**: [`zed-cl`](https://github.com/etyurkin/zed-cl) bundles its own LSP server in Rust+CL, a Jupyter kernel, and a master REPL process. It is full-featured but not published in the Zed extensions registry and is not structured like official extensions.

## What Changes

- Create a Zed extension package (`extension.toml`, `Cargo.toml`, `src/common_lisp.rs`) following the same structure as the Lua extension.
- Register the `tree-sitter-grammars/tree-sitter-commonlisp` grammar for parsing.
- Add a `Common Lisp` language definition (`languages/commonlisp/config.toml`) with file-type associations (`.lisp`, `.lsp`, `.cl`, `.asd`), comment syntax (`;` line, `#| |#` block), bracket pairs, and indentation settings.
- Provide Tree-sitter query files: `highlights.scm`, `brackets.scm`, `indents.scm`, `outline.scm`, `textobjects.scm`, and `injections.scm`.
- Integrate the `cl-lsp` language server with a binary resolution flow: user-configured path → `PATH` lookup → managed install via Roswell (where available).
- Pass through user/worktree LSP initialization options and workspace settings.
- Add `README.md` documenting usage, prerequisites, and configuration.

## Capabilities

### New Capabilities

- `common-lisp-language-support`: Zed identifies Common Lisp sources by file extension and provides Tree-sitter–powered syntax highlighting, bracket matching, indentation, outline navigation, text object selection, and comment injection.
- `common-lisp-language-server-integration`: Zed starts and configures the `cl-lsp` language server, enabling diagnostics, completion, hover documentation, go-to-definition, find-references, formatting, and signature help.

### Modified Capabilities

- None.

## Impact

- New extension assets: manifest, Rust entrypoint, language config, six query files, documentation.
- New dependency on the `tree-sitter-grammars/tree-sitter-commonlisp` grammar (pinned commit).
- New dependency on `cl-lsp` as the default language server, requiring SBCL + Roswell on the user's system.
- Cross-platform validation required (Linux, macOS, Windows) for language-server discovery and startup.
- No impact on existing Zed extensions or core editor functionality.
