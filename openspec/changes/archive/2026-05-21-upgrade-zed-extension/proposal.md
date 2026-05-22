## Summary

Align the Common Lisp Zed extension with current Zed extension documentation, fix LSP command resolution gaps, modernize Tree-sitter query capture names, and refresh documentation.

## Motivation

The extension was built against earlier Zed extension docs. Several issues have accumulated:

- `src/common_lisp.rs` only forwards user-configured args/env when a custom binary path is set. For PATH and Roswell-resolved `cl-lsp`, args/env are silently ignored.
- The Roswell install path treats `ros install cxxxr/cl-lsp` as successful even if the process exits non-zero, because it only checks `Ok(_)` on `output()` and never inspects `output.status`.
- The spec requires downloading from GitHub releases and caching, but `cxxxr/cl-lsp` has **zero** GitHub releases. The spec is impossible to implement as written.
- `textobjects.scm` uses `@function.outer`/`@function.inner`/`@class.outer`/`@class.inner` but current Zed uses `@function.around`/`@function.inside`/`@class.around`/`@class.inside`.
- README mentions `tree-sitter-cli` in the install section which is irrelevant for users; the development/architecture notes could be more precise.

## Scope

- Fix LSP command resolution to always forward args/env
- Fix Roswell install failure handling (check exit status)
- Update specs to remove impossible GitHub release download requirement
- Update textobjects capture names to current Zed conventions
- Refresh README

## Non-goals

- Adding new language features, snippets, or themes
- Changing the grammar version (already at latest upstream HEAD)
- Changing the `zed_extension_api` version (already at latest 0.7.0)
- Adding debug adapter support
