# llsp-language-server-integration Specification

## Purpose
Own everything genuinely new in moving this extension onto one shared llsp language server: routing all three language modes through a single server entry, resolving and installing a verified llsp binary, keeping that cache trustworthy, forwarding the user's configuration, holding dialect identity steady across a session, and naming the host-side gates that must clear before the old servers can be dropped.

## Requirements

### Requirement: Single shared server entry for all three language modes

The extension SHALL register exactly one language server for `Common Lisp`, `Lispico Clojure`, and `Lispico CL`, and every buffer in those modes SHALL be launched through that one server entry. A language mode MUST NOT resolve to a second server, and the previous per-mode server entries SHALL NOT remain registered alongside it. Structural editing features for every mode SHALL remain available when the server is absent or fails to start.

#### Scenario: Common Lisp buffer after migration

- **WHEN** a user opens an ordinary Common Lisp buffer
- **THEN** it is launched through the shared llsp server entry rather than through a separate Common Lisp server, with highlighting, bracket matching, indentation, and outline unchanged

#### Scenario: Lispico buffer after migration

- **WHEN** a `Lispico Clojure` or `Lispico CL` buffer opens
- **THEN** it is launched through the same shared server entry as Common Lisp, and no other language server is started for it

#### Scenario: Server unavailable

- **WHEN** the llsp binary cannot be resolved
- **THEN** the extension reports the failure and all structural editing features for all three modes remain usable

### Requirement: Explicit language identifier map for the shared server

The extension SHALL declare an explicit language-ID map for the shared server so that the identifier sent on the wire is the identifier the dialect actually declares, rather than a value derived from the display name. The map SHALL be exactly: `Common Lisp` → `lisp`, `Lispico Clojure` → `lispico-clojure`, `Lispico CL` → `lispico-cl`. A mode left to a derived default identifier SHALL be treated as a defect: the derived form is a normalization of the display name, and whether it coincides with a declared dialect identifier is an accident of that normalization rather than a guarantee — the server matches only its declared strings, exactly.

#### Scenario: Common Lisp wire identifier

- **WHEN** a Common Lisp buffer opens against the shared server
- **THEN** the document is opened with the language ID `lisp`, not a value derived from the display name

#### Scenario: Lispico wire identifiers

- **WHEN** a `Lispico Clojure` or `Lispico CL` buffer opens
- **THEN** the document is opened with `lispico-clojure` or `lispico-cl` respectively

#### Scenario: Unmapped mode

- **WHEN** a new language mode is added to the shared server without an explicit identifier entry
- **THEN** a check fails rather than relying on a derived identifier that the server cannot match

### Requirement: llsp binary resolution precedence

The extension MUST resolve the llsp command in this order and MUST NOT reorder these steps:

1. **User-configured binary**: the language-server binary path from Zed's LSP settings for the worktree.
2. **PATH lookup**: `worktree.which("llsp")` to find an already-installed binary.
3. **Verified release download**: when neither earlier step yields a binary and the current platform has a published archive, fetch the release's `SHA256SUMS` file, download the matching archive, verify the archive against the recorded digest, extract it, and mark the extracted binary executable.

There is no Roswell installation step, no source build, and no `install.sh` execution. When the platform has no published archive, the release listing is unreachable, or the digest does not match, the extension SHALL return an error naming the supported platforms, the reason resolution stopped, and the ways to fix it by configuring a binary path, exposing `llsp` on `PATH`, or placing a verified binary in the cache.

#### Scenario: Configured binary wins

- **WHEN** a user configures an llsp binary path in Zed's LSP settings
- **THEN** the extension uses that binary and performs no PATH lookup and no download

#### Scenario: PATH binary is used

- **WHEN** no path is configured and `llsp` is on `PATH`
- **THEN** the extension uses the discovered binary without contacting any release endpoint

#### Scenario: Verified download when neither earlier step applies

- **WHEN** no path is configured, `llsp` is not on `PATH`, and the platform has a published archive with a recorded digest
- **THEN** the extension reports download progress, verifies the archive against the recorded digest, caches the extracted binary, marks it executable, and starts it

#### Scenario: Digest mismatch is fatal for that attempt

- **WHEN** the downloaded archive's digest does not match the recorded digest
- **THEN** the extension does not produce a usable binary, does not record the cache entry as installed, and reports a verification failure

#### Scenario: Extraction failure is fatal for that attempt

- **WHEN** a digest-verified archive cannot be extracted
- **THEN** the extension does not record an installed state and reports an actionable error

#### Scenario: Unsupported platform

- **WHEN** the current platform has no published archive
- **THEN** the extension reports the supported platform list and the configuration, `PATH`, and cache remedies, and offers no Roswell, source-build, or installer-script alternative

### Requirement: Release archive selection

The extension SHALL select the published archive matching the current platform from the release's asset list, and SHALL reject an asset whose name does not correspond to a supported platform. Supported archives are x86_64 and aarch64 for Linux, x86_64 and aarch64 for macOS, and x86_64 for Windows. An archive with no corresponding supported platform SHALL be an error, not a silent fallback to another asset.

#### Scenario: Supported Linux x86_64

- **WHEN** resolution runs on Linux x86_64
- **THEN** the extension selects the x86_64 Linux release archive for that release

#### Scenario: Unsupported Windows architecture

- **WHEN** resolution runs on a Windows architecture with no published archive
- **THEN** the extension reports that no archive exists for that platform rather than substituting another platform's archive

### Requirement: Cache integrity and pruning

Cached llsp binaries SHALL be stored under a version-named directory, and the cache SHALL record enough per-entry state to distinguish a completed verified install from an interrupted one. A cache entry that is incomplete, not executable, or unverified MUST NOT be used to start the server. When a new version is installed, stale version directories SHALL be pruned. A failed download, verification, or extraction MUST NOT leave an entry that a later attempt can mistake for a usable install.

#### Scenario: Interrupted download leaves no usable entry

- **WHEN** a download or extraction is interrupted partway through
- **THEN** no cache entry is left in a state the extension can later start

#### Scenario: Non-executable cache entry is rejected

- **WHEN** a cache directory contains a binary that is not marked executable
- **THEN** the extension does not start it and re-resolves instead

#### Scenario: Stale versions are pruned

- **WHEN** a new version is installed successfully
- **THEN** older version directories are removed from the cache

### Requirement: Offline reuse of a verified cache

When neither a configured binary nor a `PATH` binary exists, the extension SHALL reuse a previously verified cache entry for the current version and platform without contacting the network. A cached entry SHALL be reused only when it is complete and verified, so that an offline session either starts the same binary it verified before or reports the same actionable error it would report with a network.

#### Scenario: Offline start from a verified cache

- **WHEN** the network is unreachable and a previously verified cache entry for the current version and platform exists
- **THEN** the extension starts that cached binary without a release lookup

#### Scenario: Offline with no usable cache

- **WHEN** the network is unreachable and no complete verified cache entry exists
- **THEN** the extension reports the resolution failure instead of retrying indefinitely

### Requirement: User configuration forwarding on every path

The extension SHALL forward user-configured command-line arguments, environment variables, initialization options, and workspace settings to the server on every resolution path — configured binary, `PATH`, verified download, and verified cache. Configuration MUST be forwarded as the user wrote it: the extension MUST NOT wrap, rename, restructure, inject defaults into, or suppress it, and MUST NOT open a buffer again or restart the server to force configuration through. Initialization options that the server does not support are a configuration error to be reported, not a value to be filtered out on the user's behalf.

#### Scenario: Arguments and environment on the downloaded path

- **WHEN** the binary was obtained through a verified download and the user configured arguments and environment
- **THEN** those values reach the server process unchanged

#### Scenario: Workspace settings reach the server

- **WHEN** workspace settings are configured in Zed's LSP settings
- **THEN** they are sent unchanged to the server through the workspace configuration channel

#### Scenario: Unsupported initialization option

- **WHEN** a user configures an initialization option the server does not accept
- **THEN** the extension reports the rejected configuration rather than quietly dropping the option or starting the server with something else

### Requirement: Dialect identity is stable for the life of a buffer

Once a buffer is launched with a dialect selected by its language ID, the extension's configuration and the server's own behavior MUST NOT silently change that buffer's dialect for the remainder of the session. A settings change that causes the server to re-derive dialects from paths and text alone, while the buffer's dialect was implied by its language ID alone, is a failure: the buffer would be re-routed to a different dialect and its dialect-specific results would disappear. Dialect-specific evidence already established for a buffer — such as reader-invalid diagnostics — MUST remain present across such a settings change.

This requirement is a release gate on upstream behavior, not a workaround. The extension MUST NOT clear it by adding configuration that compensates for a dropped language ID as if that compensation were the mechanism, and MUST NOT degrade dialect selection to file-suffix or default-dialect selection, which would silently re-route host buffers to `common-lisp`.

#### Scenario: Settings change preserves the buffer dialect

- **WHEN** a `Lispico CL` buffer whose dialect came from its language ID receives an accepted workspace settings change
- **THEN** the buffer keeps the Lispico CL dialect and its reader-invalid diagnostics are still published

#### Scenario: Repeated settings changes do not accumulate drift

- **WHEN** several settings changes are applied in succession to the same buffer
- **THEN** the buffer's dialect is unchanged after each one

#### Scenario: Buffer reopened after the settings change

- **WHEN** a buffer that lost its dialect is closed and reopened
- **THEN** it is restored from its language ID on open

#### Scenario: Gate is unmet

- **WHEN** the upstream behavior required by this requirement is not available
- **THEN** the full cutover does not ship, and the limitation is reported rather than hidden

### Requirement: Host-aware release gates

The cutover from the previous servers to the shared server SHALL NOT be released while any of the following is unmet, and each unmet gate SHALL be visible as a reported limitation rather than silently absorbed:

- the host server retains each open buffer's client-reported language identifier across a settings-driven reload;
- the identifier the extension sends is matched by the server for all three dialects;
- the host-aware context, catalog, and phase behavior the Lispico modes depend on is present for the same profiles the previous plan required. Its absence is an open gap, not a reduced scope that satisfies this requirement.

#### Scenario: Upstream language-ID retention is missing

- **WHEN** the server re-derives dialects on a settings change without retaining the client's language identifier
- **THEN** the cutover stays blocked and the behavior is reported

#### Scenario: Host-aware context is unavailable

- **WHEN** a Lispico buffer has no host catalog, phase, or context support
- **THEN** the extension reports the reduced capability and does not present ordinary Common Lisp results as host-aware results

#### Scenario: All gates met

- **WHEN** every host-aware gate is met and the shipped behavior is verified
- **THEN** the cutover may be released as one change, with the previous server entries removed rather than left as fallbacks
