## Tasks

### 1. Fix LSP command resolution

- [x] 1.1 Refactor `src/common_lisp.rs` to extract args/env from `lsp_settings.binary` once at the top of `language_server_command`, before the resolution chain
- [x] 1.2 Apply the resolved args/env to all three resolution paths (user config, PATH, Roswell)
- [x] 1.3 Verify the user-config path still works (binary path + args + env)
- [x] 1.4 Verify PATH resolution still works (found binary + args + env)

### 2. Fix Roswell install failure handling

- [x] 2.1 After `Ok(output)` from `ros install`, check `output.status != Some(0)` before treating as success
- [x] 2.2 Include stderr content in the Failed status message when ros install exits non-zero
- [x] 2.3 Clear installation status to `None` on success (already done)

### 3. Update textobjects query captures

- [x] 3.1 In `languages/commonlisp/textobjects.scm`, rename `@function.outer` → `@function.around`
- [x] 3.2 Rename `@function.inner` → `@function.inside`
- [x] 3.3 Rename `@class.outer` → `@class.around`
- [x] 3.4 Rename `@class.inner` → `@class.inside`

### 4. Update specs

- [x] 4.1 In `openspec/specs/common-lisp-language-server-integration/spec.md`, remove the GitHub release download/caching requirements
- [x] 4.2 Update the managed installation requirement to be Roswell-only
- [x] 4.3 In `openspec/specs/common-lisp-language-support/spec.md`, update textobjects requirement to reference `@function.around`/`@function.inside`/`@class.around`/`@class.inside`

### 5. Refresh README

- [x] 5.1 Remove `tree-sitter-cli` mention from the install section
- [x] 5.2 Update architecture section to reflect Roswell-only managed install (no GitHub releases)
- [x] 5.3 Verify all configuration examples are accurate

### 6. Verify

- [x] 6.1 Run `cargo fmt --check`
- [x] 6.2 Run `cargo check`
- [x] 6.3 Run `cargo build --target wasm32-wasip1 --release` (if wasm target available)
