## 0.4.0 (2026-07-04)

- Download a prebuilt, self-contained `sextant` binary from the latest GitHub release (Linux x64/arm64, Apple Silicon macOS) when it is not on `PATH`, before falling back to a Roswell source build

## 0.3.0 (2026-07-04)

- Fix outline panel missing `defun`/`defmacro`/`defgeneric`/`defmethod` definitions
- Fix "select inside function" to select the body instead of the argument list
- Fix CI to run on pushes to `master`
- Add Lisp-aware word selection and completion for symbols like `foo-bar`, `*special*`
- Add highlighting for definition names, call-position symbols, and more special forms
- Add comment continuation for `;;` and `;;;` prefixes
- Stop `"` and `|` auto-closing inside strings and comments
- Switch the language server from cl-lsp to [sextant](https://github.com/victorzhuk/sextant); the extension resolves it from `PATH`, then builds the latest master via `ros install victorzhuk/sextant` when Roswell is available

## 0.2.0 (2025-05-22)

- Add GitHub Actions CI workflow (fmt, clippy, test, wasm build)
- Add tag-based GitHub Release publishing for `v*` tags
- Fix clippy lint in error message (format! → .into())
- Fix textobjects capture names to current Zed conventions
- Update LSP command resolution to forward args/env on all paths
- Fix Roswell install failure handling (check exit status)
- Update specs and README

## 0.1.0 (2025-02-24)

- Initial Common Lisp language support for Zed
- Syntax highlighting via tree-sitter-commonlisp
- LSP integration via cl-lsp with Roswell fallback
- Bracket matching, indentation, outline, text objects
