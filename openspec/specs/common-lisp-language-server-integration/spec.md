# Common Lisp language server integration

## Purpose

Connect Common Lisp buffers in Zed to the shared llsp language server, with configurable command resolution (configured binary, `PATH`, verified release download) and language server settings.

## Requirements

### Requirement: Language server declaration

The extension SHALL register the language server in `extension.toml` under `[language_servers.llsp]` with `name = "llsp"` and `languages = ["Common Lisp", "Lispico Clojure", "Lispico CL"]`, so that Zed associates the one server with every language mode the extension owns. It SHALL also declare the explicit language identifier map `[language_servers.llsp.language_ids]` mapping `Common Lisp` = `lisp`, `Lispico Clojure` = `lispico-clojure`, and `Lispico CL` = `lispico-cl`. The derived default identifier MUST NOT be relied on for any of these modes. No separate per-mode language server entry SHALL remain registered for the languages listed above.

#### Scenario: Opening a Common Lisp project with LSP enabled

- **WHEN** a user opens a workspace containing Common Lisp files
- **THEN** Zed starts the `llsp` language server for Common Lisp buffers with the document language ID `lisp`

#### Scenario: Opening a Lispico buffer

- **WHEN** a user opens a buffer in `Lispico Clojure` or `Lispico CL` mode
- **THEN** Zed starts the same `llsp` language server with the document language ID `lispico-clojure` or `lispico-cl` respectively

### Requirement: Rust extension entrypoint

The extension SHALL implement the `zed::Extension` trait in `src/common_lisp.rs` with `language_server_command`, `language_server_initialization_options`, and `language_server_workspace_configuration` methods.

#### Scenario: Extension initialization

- **WHEN** Zed loads the extension
- **THEN** the extension struct is created and registered via `zed::register_extension!`

### Requirement: Language server binary resolution precedence

The extension MUST resolve the `llsp` command in this order:

1. **User-configured binary**: the language-server binary path from Zed's LSP settings for the worktree.
2. **PATH lookup**: `worktree.which("llsp")` to find an already-installed binary.
3. **Verified release download**: for a supported platform (Linux x86_64/aarch64, macOS x86_64/aarch64, Windows x86_64), download the matching published release archive, verify it against the digest published in the release's `SHA256SUMS` file, extract it, cache the extracted binary under a version-named directory, and mark it executable.

There is no Roswell installation step, no source build, and no installer-script execution. The download step reuses a previously verified cache entry for the current version and platform when the network is unreachable, and prunes stale version directories. When the platform has no published archive, the release listing is unreachable, the digest does not match, or extraction fails, the extension SHALL return an error naming the supported platforms, the reason resolution stopped, and the remedies: configure a binary path, expose `llsp` on `PATH`, or place a verified binary in the cache. It MUST NOT fall back to a Roswell build, a source build, or an alternate archive for another platform.

User-configured arguments and environment variables from `LspSettings` SHALL be forwarded to the server command regardless of which resolution path is taken, and forwarded unchanged.

#### Scenario: User config overrides automatic discovery

- **WHEN** a user provides a language-server binary path in Zed LSP settings
- **THEN** the extension uses that binary with the configured arguments and environment

#### Scenario: Binary found on PATH with user arguments

- **WHEN** no user-configured path is set but `llsp` is on `PATH`, and the user has configured arguments
- **THEN** the extension uses the discovered binary with the configured arguments and environment

#### Scenario: Prebuilt binary downloaded when absent from PATH

- **WHEN** no user-configured path is set, `llsp` is not on `PATH`, and the platform has a published archive
- **THEN** the extension reports `Downloading` status, verifies the downloaded archive against the published digest, caches the extracted `llsp` binary, marks it executable, and starts it

#### Scenario: Digest verification failure

- **WHEN** a downloaded archive does not match the published digest
- **THEN** the extension reports a verification failure, does not start a binary, and does not record an installed cache entry

#### Scenario: Roswell build when no downloadable binary

- **WHEN** no user-configured path is set, `llsp` is not on `PATH`, and the platform has no published archive
- **THEN** the extension reports an actionable error naming the supported platforms and the configuration, `PATH`, and cache remedies, and does not attempt a Roswell installation

#### Scenario: Roswell build fails

- **WHEN** a release download, digest verification, or extraction step fails
- **THEN** the extension reports `Failed` status with the reason and the same remedies, having attempted no Roswell installation and no source build

#### Scenario: No binary and no Roswell

- **WHEN** no user-configured path is set, `llsp` is not on `PATH`, and no verified cached binary or downloadable archive is available
- **THEN** the extension returns an error listing the supported platforms and every supported way to supply a binary, with no Roswell, source-build, or installer-script alternative

#### Scenario: Offline reuse of a verified cache

- **WHEN** no user-configured path is set, `llsp` is not on `PATH`, the network is unreachable, and a complete verified cache entry for the current version and platform exists
- **THEN** the extension starts that cached binary without a release lookup

### Requirement: Custom server arguments pass-through

The extension SHALL forward user-configured command-line arguments and environment variables from `LspSettings` binary settings to the server command for **all** resolution paths (user config, PATH lookup, verified release download, and verified cache reuse).

#### Scenario: User specifies server arguments for PATH-resolved binary

- **WHEN** a user configures arguments in Zed LSP settings but no custom path
- **THEN** those arguments are passed to the `llsp` process found on PATH

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
