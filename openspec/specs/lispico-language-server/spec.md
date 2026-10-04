# lispico-language-server Specification

## Purpose
Connect Zed to a native static Lispico service for accurate semantic editing of runtime and host projects.

## Requirements

### Requirement: Lispico mode isolation from ordinary Common Lisp

The extension SHALL route `Lispico Clojure` and `Lispico CL` buffers to the shared language server with their explicit language identifiers, and SHALL keep their configuration independent of any other language's settings. Registered server entries, language identifiers, binary resolution, cache handling, and settings forwarding are owned by the [llsp-language-server-integration](../llsp-language-server-integration/spec.md) capability; this requirement covers only what a Lispico buffer must observe. Launch SHALL never evaluate runtime code, run a host session, or execute project sources.

#### Scenario: Explicit binary settings
- **WHEN** a user configures a binary path, arguments, and environment for the Lispico modes
- **THEN** that command receives those values and no other server is started for the buffer

#### Scenario: PATH-resolved binary with settings
- **WHEN** only arguments/environment are configured and the server binary exists on PATH
- **THEN** those settings also apply to the PATH-resolved binary

#### Scenario: Common Lisp buffer
- **WHEN** an ordinary Common Lisp buffer opens
- **THEN** its own settings and dialect selection remain independent of Lispico mode configuration, and changing Lispico configuration does not change the Common Lisp buffer's dialect

#### Scenario: Unavailable server
- **WHEN** the shared server cannot be resolved or started
- **THEN** the extension reports the failure and leaves structural editing usable in both Lispico modes

### Requirement: Contextual completion and documentation

Completion, hover, and signature help SHALL use the current lexical scope, selected libraries, dialect, and host catalog. They SHALL preserve qualified symbol ranges, distinguish values/functions/macros, and show verified signatures/docs/provenance. In Lisp-2 contexts the resolver SHALL honor function/value cells. Unavailable bindings MUST NOT be offered as available globals.

#### Scenario: zhk host call
- **WHEN** completion or signature help is requested for `zhk/step` in a zhk route
- **THEN** it shows the matching host declaration and verified two-argument signature

#### Scenario: Same function and value name in CL
- **WHEN** a Lispico CL context contains both bindings for one name
- **THEN** call-position and value-position requests resolve their respective namespace cells

### Requirement: Source navigation and symbols

The service SHALL support definitions, statically resolved references, document symbols, and workspace symbol search. Definitions SHALL lead to original Lisp source, verified host source, or a readable local catalog when host source is unavailable. Results SHALL retain context/provenance, distinguish shadowed sources, and avoid claiming completeness for dynamic or macro-expanded references.

#### Scenario: Prelude definition navigation
- **WHEN** a zhk route requests the definition of a visible prelude helper
- **THEN** navigation selects that helper's real source location, not a copied stub

#### Scenario: Host checkout absent
- **WHEN** a Go-hosted symbol's source checkout is unavailable but its compatible catalog is present
- **THEN** navigation and documentation remain available from that declaration without a fabricated source location

### Requirement: Document lifecycle and position accuracy

The service SHALL analyze unsaved document versions over disk contents, support negotiated position encoding with UTF-16 fallback, and ignore stale analysis results. Changes, save/reload, deletion, closure, and context switches SHALL update or clear relevant results. Incomplete buffers SHALL preserve useful unaffected symbols and diagnostics.

#### Scenario: Unicode before an error
- **WHEN** multibyte or supplementary Unicode characters precede an erroneous form
- **THEN** diagnostic and navigation ranges select the correct text in Zed

#### Scenario: Rapid edits
- **WHEN** a newer document version arrives while an older analysis is running
- **THEN** the old result cannot overwrite the new version's diagnostics or symbol state

#### Scenario: Settings change reloads a Lispico buffer
- **WHEN** a workspace settings change causes the server to reload the open documents of a buffer whose dialect came from its language ID
- **THEN** that buffer keeps its dialect and its dialect-specific results, and a silent re-route to the default Common Lisp dialect is a failure rather than an accepted reload

### Requirement: Bounded responsive service

The service SHALL support initialization, shutdown/exit, cancellation, and bounded document/index work. Source/config/catalog changes SHALL invalidate affected contexts. Hitting a resource limit SHALL report an incomplete-analysis condition without crashing, hanging indefinitely, or presenting partial work as complete. It MUST advertise only implemented capabilities.

#### Scenario: Large workspace exceeds a limit
- **WHEN** configured indexing limits are exceeded
- **THEN** the service reports the limit and remains responsive to edits and shutdown

#### Scenario: Server restart
- **WHEN** Zed restarts the server
- **THEN** current open documents and their selected contexts rebuild without depending on a previous process's state
