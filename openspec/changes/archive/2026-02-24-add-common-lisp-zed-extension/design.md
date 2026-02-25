# Design: Common Lisp Zed Extension

## Technical Approach

Follow the structure and conventions established by the official [Lua Zed extension](https://github.com/zed-extensions/lua). The extension is a Rust WASM module compiled as `cdylib` against `zed_extension_api`. It declares a grammar source, a language configuration, Tree-sitter query files, and a language server integration — all wired together through `extension.toml`.

The grammar comes from the existing [`tree-sitter-grammars/tree-sitter-commonlisp`](https://github.com/tree-sitter-grammars/tree-sitter-commonlisp) repository (v0.4.1, pinned by commit). Since this grammar does not ship highlights or editor queries (only `tags.scm`), all query files must be authored from scratch for Zed.

The language server is [`cl-lsp`](https://github.com/cxxxr/cl-lsp) by cxxxr. It is the most mature Common Lisp LSP implementation (236 stars, MIT) and supports go-to-definition, find-references, hover, completion, signature help, and formatting. It requires a Common Lisp runtime (SBCL) and is typically installed via [Roswell](https://github.com/roswell/roswell).

## Architecture Decisions

### Decision: Lean extension following official patterns (not a custom LSP)

Use the same architecture as the Lua extension — a thin Rust shim that resolves and launches an external language server — rather than bundling a custom LSP like `zed-cl` does.

**Rationale:**
- Matches the pattern all official Zed language extensions follow
- Much simpler to maintain (< 200 lines of Rust)
- Users can swap or upgrade the language server independently
- Avoids bundling SBCL or managing a master REPL process

**Trade-off:** No built-in REPL integration. Users who want interactive evaluation would need a separate tool or a future extension.

### Decision: `cl-lsp` as the default language server

Use `cl-lsp` (cxxxr) rather than `alive-lsp` (nobody-famous).

**Rationale:**
- More stars (236 vs 111), broader adoption
- MIT license (vs Unlicense)
- Installable via Roswell (`ros install cxxxr/cl-lsp`) which produces a standalone binary
- Supports the core LSP features needed: completion, hover, go-to-definition, formatting

**Trade-off:** `alive-lsp` has tighter VS Code / Swank integration. Users preferring `alive-lsp` can configure it via Zed LSP settings.

### Decision: `tree-sitter-grammars/tree-sitter-commonlisp` grammar

Use the community-maintained grammar from the `tree-sitter-grammars` organization.

**Rationale:**
- Only maintained Common Lisp Tree-sitter grammar
- Used by difftastic and Neovim ecosystem
- Active development (last release March 2025)
- Handles CL-specific syntax: reader macros, `#|...|#` comments, keyword symbols, character literals, etc.

### Decision: Author query files from scratch

Write all Zed query files (`highlights.scm`, `brackets.scm`, `indents.scm`, `outline.scm`, `textobjects.scm`, `injections.scm`) from scratch rather than porting from another editor.

**Rationale:**
- The upstream grammar only includes `tags.scm` — no highlights or editor queries
- Zed's query format and capture names differ from Neovim/Helix
- The `zed-cl` extension's `highlights.scm` can serve as a reference but needs adaptation for our grammar pin and Zed conventions

### Decision: Binary resolution order: user config → PATH → Roswell install

Follow the Lua extension's binary resolution pattern with a Roswell-based fallback instead of GitHub release downloads.

**Rationale:**
- `cl-lsp` does not publish pre-built platform binaries on GitHub Releases
- Roswell is the standard Common Lisp toolchain manager and can build `cl-lsp` from source on any platform
- Users who install `cl-lsp` manually (via Roswell or otherwise) just need it on PATH
- The user-config-first approach matches Zed conventions and allows overriding for any LSP server

**Trade-off:** Managed install requires Roswell to be installed, which is an extra prerequisite. If Roswell is absent, the extension reports a clear error directing the user to install `cl-lsp` manually.

### Decision: File extensions `.lisp`, `.lsp`, `.cl`, `.asd`

Register these four suffixes for the Common Lisp language mode.

**Rationale:**
- `.lisp` — most common CL source file extension
- `.lsp` — traditional shorter variant
- `.cl` — used by some projects (e.g., SBCL source)
- `.asd` — ASDF system definition files, which are valid Common Lisp

**Excluded:** `.l` (conflicts with Lex/Flex), `.ros` (Roswell scripts, niche use case — can be added later).

## Data Flow

```
┌────────────────────────┐
│     Zed Editor         │
│                        │
│  ┌──────────────────┐  │
│  │ Buffer (.lisp)   │  │
│  │                  │  │
│  │ tree-sitter-     │  │      ┌──────────────┐
│  │ commonlisp       │──┼─────►│ Query Files  │
│  │ (WASM grammar)   │  │      │ highlights   │
│  │                  │  │      │ brackets     │
│  └──────────────────┘  │      │ indents      │
│                        │      │ outline      │
│  ┌──────────────────┐  │      │ textobjects  │
│  │ Extension        │  │      │ injections   │
│  │ (common_lisp.rs) │  │      └──────────────┘
│  │                  │  │
│  │ resolve binary ──┼──┼───► cl-lsp process (SBCL)
│  │ forward settings │  │      │
│  │ format labels    │  │      ├─ completion
│  └──────────────────┘  │      ├─ hover
│                        │      ├─ go-to-definition
└────────────────────────┘      ├─ find-references
                                ├─ formatting
                                └─ signature help
```

## File Changes

### New Files

| File | Purpose |
|------|---------|
| `extension.toml` | Extension manifest: id, name, version, grammar source, language server declaration |
| `Cargo.toml` | Rust crate config: `cdylib` target, `zed_extension_api` dependency |
| `Cargo.lock` | Locked dependency versions |
| `src/common_lisp.rs` | Rust entrypoint: `Extension` trait impl, binary resolution, label formatting, settings pass-through |
| `languages/commonlisp/config.toml` | Language config: name, grammar, file suffixes, comments, brackets, indentation |
| `languages/commonlisp/highlights.scm` | Syntax highlighting queries |
| `languages/commonlisp/brackets.scm` | Bracket pair queries for matching/rainbow brackets |
| `languages/commonlisp/indents.scm` | Indentation queries for auto-indent |
| `languages/commonlisp/outline.scm` | Outline/symbol panel queries for definition forms |
| `languages/commonlisp/textobjects.scm` | Text object queries for structural selection |
| `languages/commonlisp/injections.scm` | Injection queries for comment highlighting (TODO/FIXME) |
| `README.md` | Usage documentation, prerequisites, configuration |
| `LICENSE` | Apache-2.0 (matching Zed extension convention) |
| `.gitignore` | Ignore `target/` build directory |
