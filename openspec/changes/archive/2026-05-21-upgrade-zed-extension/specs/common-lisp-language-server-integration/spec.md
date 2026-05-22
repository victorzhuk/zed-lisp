## CHANGED Requirements

### Requirement: Language server binary resolution precedence

The extension MUST resolve the `cl-lsp` command in this order:

1. **User-configured binary**: `LspSettings::for_worktree` binary path from Zed's LSP settings.
2. **PATH lookup**: `worktree.which("cl-lsp")` to find an already-installed binary.
3. **Roswell installation**: Run `ros install cxxxr/cl-lsp`, then retry PATH lookup.

User-configured arguments and environment variables from `LspSettings` SHALL be forwarded to the server command regardless of which resolution path is taken.

#### Scenario: User config overrides automatic discovery

- **WHEN** a user provides a language-server binary path in Zed LSP settings
- **THEN** the extension uses that binary with the configured arguments and environment

#### Scenario: Binary found on PATH with user arguments

- **WHEN** no user-configured path is set but `cl-lsp` is on `PATH`, and the user has configured arguments
- **THEN** the extension uses the discovered binary with the configured arguments and environment

### Requirement: Roswell managed installation

If no user-configured or `PATH` binary is available, the extension SHALL attempt installation via Roswell. It MUST report installation status using `zed::set_language_server_installation_status`. The install MUST:

1. Check that `ros` is available via `worktree.which("ros")`.
2. Run `ros install cxxxr/cl-lsp` and check the process exit status.
3. If the exit status is non-zero, report `Failed` with the stderr content.
4. If the exit status is zero, retry `worktree.which("cl-lsp")` to locate the installed binary.
5. If the binary is still not found after successful install, report an error advising the user to add `~/.roswell/bin` to PATH.

#### Scenario: Roswell install succeeds

- **WHEN** `ros` is available and `ros install cxxxr/cl-lsp` exits with status 0
- **THEN** the extension resolves `cl-lsp` from PATH and starts the server

#### Scenario: Roswell install fails

- **WHEN** `ros install cxxxr/cl-lsp` exits with a non-zero status
- **THEN** the extension reports `Failed` status with the stderr output

#### Scenario: No Roswell available

- **WHEN** no user-configured path, no PATH binary, and no `ros` binary
- **THEN** the extension returns an error message listing the three installation options

### Requirement: Custom server arguments pass-through

The extension SHALL forward user-configured command-line arguments and environment variables from `LspSettings` binary settings to the server command for **all** resolution paths (user config, PATH lookup, and Roswell-managed).

#### Scenario: User specifies server arguments for PATH-resolved binary

- **WHEN** a user configures arguments in Zed LSP settings but no custom path
- **THEN** those arguments are passed to the `cl-lsp` process found on PATH
