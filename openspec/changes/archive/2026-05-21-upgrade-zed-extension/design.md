## Approach

Minimal fixes that keep the existing architecture unchanged.

## LSP Command Resolution Fix

Current code in `src/common_lisp.rs` only forwards `binary_settings.arguments` and `binary_settings.env` when the user provides a custom `binary_settings.path`. For PATH and Roswell resolution paths, args/env are silently dropped.

**Fix**: Extract args/env resolution into a shared helper, or simply check `lsp_settings.binary` for arguments/env at the top and apply them to every resolution path.

```
fn resolve_lsp_command(language_server_id, worktree) -> Result<Command>:
    lsp_settings = LspSettings::for_worktree(...)
    args = lsp_settings.binary.arguments.unwrap_or_default()
    env = lsp_settings.binary.env.map(|h| h.into_iter().collect()).unwrap_or_default()

    if let Some(path) = lsp_settings.binary.path:
        return Ok(Command { command: path, args, env })

    if let Some(cl_lsp) = worktree.which("cl-lsp"):
        return Ok(Command { command: cl_lsp, args, env })

    if let Some(ros) = worktree.which("ros"):
        install via ros, then return Ok(Command { command: cl_lsp, args, env })

    Err(...)
```

## Roswell Install Failure Handling

Current code:
```rust
match output {
    Ok(_) => { /* assume success */ }
    Err(err) => { /* set Failed status */ }
}
```

`Ok(_)` means the process ran, not that it succeeded. The output struct has `status: Option<i32>`.

**Fix**: After `Ok(output)`, check `output.status != Some(0)`. If non-zero, report stderr in the Failed status.

```rust
Ok(output) => {
    if output.status != Some(0) {
        let stderr = String::from_utf8_lossy(&output.stderr);
        set_language_server_installation_status(id, &Failed(format!("ros install failed: {stderr}")));
        return Err("ros install cl-lsp failed".into());
    }
    // ... proceed with which("cl-lsp")
}
```

## Spec Update: Remove GitHub Release Requirement

The `common-lisp-language-server-integration` spec requires downloading from GitHub releases and caching. Since `cxxxr/cl-lsp` has no releases, this is impossible.

**Fix**: Update the spec to only require Roswell-based managed installation. Remove the GitHub release download and binary caching requirements. Keep the three-tier resolution order: user config → PATH → Roswell install.

## Textobjects Query Update

`textobjects.scm` uses deprecated capture names:

Current:
```scheme
(defun) @function.outer
(defun (defun_header ...) @function.inner)
(list_lit) @class.outer
(list_lit ... @class.inner)
```

Updated to current Zed conventions:
```scheme
(defun) @function.around
(defun (defun_header ...) @function.inside
(list_lit) @class.around
(list_lit ... @class.inside)
```

## README Updates

- Remove `tree-sitter-cli` from the install section (not needed for users)
- Fix architecture notes to accurately describe the current three-tier resolution
- Note that managed install is Roswell-only (no GitHub releases available)
- Keep existing configuration examples

## Affected Files

| File | Change |
|------|--------|
| `src/common_lisp.rs` | Fix args/env forwarding, fix Roswell failure handling |
| `languages/commonlisp/textobjects.scm` | Rename capture names to current conventions |
| `openspec/specs/common-lisp-language-server-integration/spec.md` | Remove GitHub release download requirement |
| `openspec/specs/common-lisp-language-support/spec.md` | Update textobjects requirement |
| `README.md` | Refresh install/architecture notes |
