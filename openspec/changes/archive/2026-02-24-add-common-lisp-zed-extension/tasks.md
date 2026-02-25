# Tasks

## 1. Project Scaffolding

- [x] 1.1 Create `extension.toml` with id `common-lisp`, name, version `0.1.0`, schema_version `1`, authors, repository, `[grammars.commonlisp]` pointing to `tree-sitter-grammars/tree-sitter-commonlisp` at a pinned commit, and `[language_servers.cl-lsp]` declaration
- [x] 1.2 Create `Cargo.toml` with crate name `zed_common_lisp`, edition 2021, `cdylib` lib target at `src/common_lisp.rs`, and `zed_extension_api = "0.7.0"` dependency
- [x] 1.3 Create `.gitignore` ignoring `target/`
- [x] 1.4 Create `LICENSE` (Apache-2.0)

## 2. Language Configuration

- [x] 2.1 Create `languages/commonlisp/config.toml` with `name = "Common Lisp"`, `grammar = "commonlisp"`, `path_suffixes = ["lisp", "lsp", "cl", "asd"]`
- [x] 2.2 Add comment syntax: `line_comments = ["; "]`, `block_comment = ["#|", "|#"]`
- [x] 2.3 Add bracket pairs: `()` (close+newline), `""` (close, not in string), `||` (symbol escape, close, not in string)
- [x] 2.4 Set `autoclose_before = ";:.,=}])>\""`, `tab_size = 2`, `hard_tabs = false`
- [x] 2.5 Set `collapsed_placeholder = "#| ... |#"`

## 3. Tree-sitter Query Files

- [x] 3.1 Create `languages/commonlisp/highlights.scm` — highlight comments, strings, numbers, character literals, keyword symbols (`:foo`), `nil`/`t` constants, definition forms (`defun`, `defmacro`, `defclass`, `defvar`, `defparameter`, etc.), control flow (`if`, `when`, `unless`, `cond`, `case`), binding forms (`let`, `let*`, `lambda`, `flet`, `labels`), iteration (`loop`, `do`, `dotimes`, `dolist`), operators (`setf`, `setq`, `and`, `or`, `not`), built-in functions, lambda-list keywords (`&optional`, `&rest`, `&key`, `&body`), quote/backquote/unquote, and parentheses
- [x] 3.2 Create `languages/commonlisp/brackets.scm` — match `("(" @open ")" @close)` pairs
- [x] 3.3 Create `languages/commonlisp/indents.scm` — mark `list_lit` and `quoting_lit` as indent containers
- [x] 3.4 Create `languages/commonlisp/outline.scm` — capture `defun`, `defmacro`, `defgeneric`, `defmethod`, `defclass`, `defvar`, `defparameter`, `defconstant`, `defpackage` forms with their names as outline items
- [x] 3.5 Create `languages/commonlisp/textobjects.scm` — capture `list_lit` as function/class text objects
- [x] 3.6 Create `languages/commonlisp/injections.scm` — inject `comment` language into `(comment)` nodes

## 4. Rust Extension Entrypoint

- [x] 4.1 Create `src/common_lisp.rs` with `CommonLispExtension` struct holding `cached_binary_path: Option<String>`
- [x] 4.2 Implement `language_server_binary()` helper with resolution order: user-configured binary path → `worktree.which("cl-lsp")` → Roswell-managed install
- [x] 4.3 Implement Roswell-based managed install: check for `ros` on PATH, run `ros install cxxxr/cl-lsp`, cache resulting binary path; report installation status via `zed::set_language_server_installation_status`
- [x] 4.4 Implement `zed::Extension` trait: `new()`, `language_server_command()` (delegates to binary resolution, forwards user-configured args)
- [x] 4.5 Implement `language_server_initialization_options()` — pass through `LspSettings::for_worktree(...).initialization_options`
- [x] 4.6 Implement `language_server_workspace_configuration()` — pass through `LspSettings::for_worktree(...).settings`
- [x] 4.7 Implement `label_for_completion()` — format function/method completions with full signature, name as filter range
- [x] 4.8 Register extension with `zed::register_extension!(CommonLispExtension)`

## 5. Build Verification

- [x] 5.1 Run `cargo check` — verify the crate compiles without errors
- [x] 5.2 Run `cargo build --target wasm32-wasip1` — verify WASM target builds (requires `wasm32-wasip1` target installed)
- [x] 5.3 Test extension loading in Zed via "Install Dev Extension"
- [x] 5.4 Verify file detection: open `.lisp`, `.lsp`, `.cl`, `.asd` files and confirm `Common Lisp` mode activates
- [x] 5.5 Verify syntax highlighting renders correctly on sample Common Lisp code
- [x] 5.6 Verify bracket matching, auto-close, and indentation behavior
- [x] 5.7 Verify outline panel shows `defun`/`defmacro`/`defclass` definitions
- [x] 5.8 Verify TODO/FIXME highlighting in comments

## 6. Language Server Integration Verification

- [x] 6.1 Install `cl-lsp` via Roswell (`ros install cxxxr/cl-lsp`) on the test machine
- [x] 6.2 Verify the extension discovers `cl-lsp` on PATH and starts it
- [x] 6.3 Verify completion, hover, and go-to-definition work in a sample CL project
- [x] 6.4 Test user-configured binary path override via Zed settings
- [x] 6.5 Test managed install flow when `cl-lsp` is not on PATH but `ros` is available
- [x] 6.6 Verify graceful error message when neither `cl-lsp` nor `ros` is available

## 7. Documentation

- [x] 7.1 Create `README.md` with extension description, feature list, prerequisites (SBCL, Roswell, cl-lsp), installation instructions, configuration examples (custom binary path, init options), and development instructions
