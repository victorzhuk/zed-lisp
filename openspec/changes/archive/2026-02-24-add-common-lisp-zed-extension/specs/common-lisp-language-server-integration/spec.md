## ADDED Requirements

### Requirement: Language server declaration

The extension SHALL register a language server in `extension.toml` under `[language_servers.cl-lsp]` with `name = "cl-lsp"` and `languages = ["Common Lisp"]` so that Zed associates the server with Common Lisp buffers.

#### Scenario: Opening a Common Lisp project with LSP enabled

- **WHEN** a user opens a workspace containing Common Lisp files
- **THEN** Zed starts the `cl-lsp` language server for Common Lisp buffers

### Requirement: Rust extension entrypoint

The extension SHALL implement the `zed::Extension` trait in `src/common_lisp.rs` with `language_server_command`, `language_server_initialization_options`, and `language_server_workspace_configuration` methods, following the same pattern as the Lua extension.

#### Scenario: Extension initialization

- **WHEN** Zed loads the extension
- **THEN** the extension struct is created and registered via `zed::register_extension!`

### Requirement: Language server binary resolution precedence

The extension MUST resolve the `cl-lsp` command in this order:

1. **User-configured binary**: `LspSettings::for_worktree` binary path from Zed's LSP settings.
2. **PATH lookup**: `worktree.which("cl-lsp")` to find an already-installed binary.
3. **Managed installation**: Attempt to install via Roswell (`ros install cxxxr/cl-lsp`) or download from GitHub releases, caching the binary path.

#### Scenario: User config overrides automatic discovery

- **WHEN** a user provides a language-server binary path in Zed LSP settings (e.g., `{"lsp": {"cl-lsp": {"binary": {"path": "/path/to/cl-lsp"}}}}`)
- **THEN** the extension uses that binary instead of `PATH` lookup or managed installation

#### Scenario: Binary found on PATH

- **WHEN** no user-configured path is set but `cl-lsp` is on `PATH`
- **THEN** the extension uses the discovered binary

### Requirement: Managed installation fallback

If no user-configured or `PATH` binary is available, the extension SHALL attempt a managed installation. It MUST report installation status during check/download phases using `zed::set_language_server_installation_status`. The managed install strategy SHALL:

1. Check for the latest GitHub release of `cxxxr/cl-lsp`.
2. Download the appropriate binary for the current platform/architecture.
3. Cache the binary path to avoid re-downloading on subsequent startups.

If GitHub releases are not available for the platform, the extension SHALL fall back to checking if Roswell is installed and running `ros install cxxxr/cl-lsp`.

#### Scenario: No server binary is present locally

- **WHEN** server startup is requested and no explicit or `PATH` binary is found
- **THEN** the extension attempts managed installation and surfaces status updates (CheckingForUpdate, Downloading) to Zed

#### Scenario: Managed install binary is cached

- **WHEN** a managed install was previously completed and the cached binary path is valid
- **THEN** the extension reuses the cached binary without re-downloading

### Requirement: Custom server arguments pass-through

The extension SHALL forward user-configured command-line arguments from `LspSettings` binary arguments to the server command, allowing users to pass custom flags (e.g., port, log-level).

#### Scenario: User specifies server arguments

- **WHEN** a user configures `{"lsp": {"cl-lsp": {"binary": {"arguments": ["--port", "8080"]}}}}` in Zed settings
- **THEN** those arguments are passed to the `cl-lsp` process

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
