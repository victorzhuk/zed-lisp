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
