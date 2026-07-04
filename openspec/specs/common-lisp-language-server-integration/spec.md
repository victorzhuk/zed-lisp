## ADDED Requirements

### Requirement: Language server declaration

The extension SHALL register a language server in `extension.toml` under `[language_servers.sextant]` with `name = "sextant"` and `languages = ["Common Lisp"]` so that Zed associates the server with Common Lisp buffers.

#### Scenario: Opening a Common Lisp project with LSP enabled

- **WHEN** a user opens a workspace containing Common Lisp files
- **THEN** Zed starts the `sextant` language server for Common Lisp buffers

### Requirement: Rust extension entrypoint

The extension SHALL implement the `zed::Extension` trait in `src/common_lisp.rs` with `language_server_command`, `language_server_initialization_options`, and `language_server_workspace_configuration` methods.

#### Scenario: Extension initialization

- **WHEN** Zed loads the extension
- **THEN** the extension struct is created and registered via `zed::register_extension!`

### Requirement: Language server binary resolution precedence

The extension MUST resolve the `sextant` command in this order:

1. **User-configured binary**: `LspSettings::for_worktree` binary path from Zed's LSP settings.
2. **PATH lookup**: `worktree.which("sextant")` to find an already-installed binary.
3. **GitHub release download**: for a supported platform (Linux x64/arm64, Apple Silicon macOS), download the matching self-contained `sextant-<platform>` asset from the latest `victorzhuk/sextant` release, cache it under a version-named directory, and mark it executable.
4. **Roswell build**: if `worktree.which("ros")` is available, build the latest master with `ros install victorzhuk/sextant`, then retry the PATH lookup.

The download step reuses a previously cached binary when present and prunes stale version directories. When the platform has no published asset, no release is reachable, or the download fails, the extension falls through to the Roswell build. If none of the resolution paths yields a binary, the extension SHALL return an error explaining how to install Roswell, the `ros install victorzhuk/sextant` command, how to expose the binary on `PATH`, and how to configure a binary path in Zed settings.

User-configured arguments and environment variables from `LspSettings` SHALL be forwarded to the server command regardless of which resolution path is taken.

#### Scenario: User config overrides automatic discovery

- **WHEN** a user provides a language-server binary path in Zed LSP settings
- **THEN** the extension uses that binary with the configured arguments and environment

#### Scenario: Binary found on PATH with user arguments

- **WHEN** no user-configured path is set but `sextant` is on `PATH`, and the user has configured arguments
- **THEN** the extension uses the discovered binary with the configured arguments and environment

#### Scenario: Prebuilt binary downloaded when absent from PATH

- **WHEN** no user-configured path is set, `sextant` is not on `PATH`, and the platform has a published release asset
- **THEN** the extension reports `Downloading` status, downloads and caches the `sextant-<platform>` binary, marks it executable, and starts it

#### Scenario: Roswell build when no downloadable binary

- **WHEN** no user-configured path is set, `sextant` is not on `PATH`, no release asset is available for the platform (or the download fails), and `ros` is available
- **THEN** the extension reports `Downloading` status, runs `ros install victorzhuk/sextant`, and on success resolves the built binary from `PATH`

#### Scenario: Roswell build fails

- **WHEN** `ros install victorzhuk/sextant` exits with a non-zero status
- **THEN** the extension reports `Failed` status with the captured stderr

#### Scenario: No binary and no Roswell

- **WHEN** no user-configured path is set, `sextant` is not on `PATH`, and `ros` is not available
- **THEN** the extension returns an error message with the Roswell install command and the Zed settings example

### Requirement: Custom server arguments pass-through

The extension SHALL forward user-configured command-line arguments and environment variables from `LspSettings` binary settings to the server command for **all** resolution paths (user config, PATH lookup, downloaded release, and Roswell-built).

#### Scenario: User specifies server arguments for PATH-resolved binary

- **WHEN** a user configures arguments in Zed LSP settings but no custom path
- **THEN** those arguments are passed to the `sextant` process found on PATH

### Requirement: LSP initialization options pass-through

The extension SHALL forward user/worktree initialization options to the language server via `language_server_initialization_options`, returning the value from `LspSettings::for_worktree(...).initialization_options`.

#### Scenario: Custom initialization options are defined

- **WHEN** initialization options are configured in Zed LSP settings
- **THEN** those values are sent unchanged in the `initialize` request to the server

### Requirement: LSP workspace configuration pass-through

The extension SHALL forward user/worktree workspace settings to the language server via `language_server_workspace_configuration`, returning the value from `LspSettings::for_worktree(...).settings`.

#### Scenario: Custom workspace settings are defined

- **WHEN** workspace settings are configured in Zed LSP settings
- **THEN** those values are sent unchanged in `workspace/didChangeConfiguration` notifications

### Requirement: Code label formatting

The extension SHOULD implement `label_for_completion` to provide styled code labels for LSP completions. Function and method completions SHALL display the full signature with the name portion as the filter range. Symbol completions SHOULD display with appropriate syntax highlighting spans.

#### Scenario: Function completion is shown

- **WHEN** the LSP returns a function completion like `format(stream control-string &rest args)`
- **THEN** Zed displays the full signature as a code label with the function name as the filterable portion
