# lispico-project-context Specification

## Purpose
Resolve Lispico code against explicit dialect, library, and host scopes so editor results match the program that each project actually loads.

## Requirements

### Requirement: Portable deterministic project configuration

The tooling SHALL accept a versioned declarative `.lispico.json` at the worktree root or an explicitly selected project file. The coordinated host-aware implementation SHALL use project and catalog schema version 2 under the [approved shared contracts](../../contracts.md), without silently converting version-1 resources. Relative paths SHALL resolve from the selected file's directory. Configuration SHALL select runtime provenance, catalogs with independently expected host/library provenance, dialect, host profile, source patterns, enabled libraries, and applicable source-loading rules. It MUST NOT execute configuration, scan ancestor projects, or implicitly import user-home state.

#### Scenario: Repository moves
- **WHEN** a repository with relative configuration paths is opened from another directory
- **THEN** the same configured sources, libraries, and catalogs are resolved relative to that repository

#### Scenario: Independent worktrees
- **WHEN** two worktrees select different runtime versions or hosts
- **THEN** their catalog and symbol state remain isolated

### Requirement: Unambiguous context selection and reload

Normalized source paths SHALL match exactly one configured executable-source context. Exclusions SHALL override inclusions. Overlap, unsupported schema versions, missing required resources, and language/dialect disagreement SHALL produce actionable configuration diagnostics. Unmatched source SHALL retain syntax support without guessed host bindings. Configuration changes SHALL invalidate affected results, and invalid configuration MUST NOT silently retain obsolete semantic results.

#### Scenario: Overlapping source patterns
- **WHEN** a file matches both a zhk and Yagel context
- **THEN** the tooling reports ambiguity instead of choosing a host by declaration order

#### Scenario: Profile becomes invalid
- **WHEN** an active context is edited to reference a missing catalog
- **THEN** results derived from the prior catalog are cleared and one context problem is reported instead of an unknown-symbol cascade

### Requirement: Inert versioned host and library declarations

Go-defined symbols SHALL be described by non-executable catalogs with schema version, owner/source provenance, dialect/profile, library identity, symbol kind, namespace cell, documentation, known signature information, phase availability, and source location when known. Unverified signature/type information MUST remain unknown. Incompatible catalogs MUST NOT be silently substituted. Actual Lisp source SHALL remain the authority for its own definitions.

#### Scenario: Known host arity
- **WHEN** the matching zhk catalog describes `zhk/read`
- **THEN** completion and signature help expose its verified one-or-two-argument shape

#### Scenario: Unknown signature
- **WHEN** a verified host name lacks verified argument information
- **THEN** it can appear as a known symbol without invented parameter names or definitive arity diagnostics

#### Scenario: Version mismatch
- **WHEN** a selected catalog's runtime or host provenance does not match the configured source version
- **THEN** the tooling reports the mismatch and does not silently analyze against newer definitions

#### Scenario: Host checkout is unavailable
- **WHEN** a host catalog is explicitly pinned but no host source root is configured
- **THEN** the tooling verifies the expected catalog identity and identifies it as catalog-only provenance without claiming installed-binary compatibility

#### Scenario: Development source changed
- **WHEN** a configured host source root differs from the catalog's recorded source-file fingerprints
- **THEN** the tooling reports stale metadata instead of treating an unchanged version string as proof of a match

### Requirement: Library availability is explicit

Only selected libraries SHALL contribute builtins and macros. Source roots SHALL permit indexing without making every definition globally visible. Dialect aliases and callable signatures SHALL reflect the selected runtime, including CL adapters and separate function/value namespaces. The tooling MUST NOT infer JVM namespaces, ASDF systems, or Lisp imports that the selected host does not provide.

#### Scenario: Disabled plugin
- **WHEN** a runtime plugin exists on disk but is absent from a context's library selection
- **THEN** its symbols are not offered or accepted as available globals

#### Scenario: Unloaded source file
- **WHEN** an indexed file is not loaded into the current source context
- **THEN** its definitions can be located as workspace source without resolving current-file references to them as globals

### Requirement: zhk ordered workflow visibility

The zhk profile SHALL use Lispico Clojure, stdlib, JSON, zhk host declarations, lexically ordered `lib/*.lisp`, and one selected route's `main.lisp`. It SHALL preserve evaluation order, redefinitions, and definition origins. Routes MUST remain isolated. Auxiliary files included only in workflow digests MUST NOT contribute executed definitions. An explicitly selected replacement workflow root SHALL replace the default root wholesale.

Requests inside a shared prelude file SHALL resolve route-independent bindings and identify route-dependent uncertainty. They MUST NOT select an arbitrary route or union route globals. Route-specific snapshots SHALL remain separate even though they reference the same prelude document.

#### Scenario: Route references prelude helper
- **WHEN** a route references a helper defined in its ordered prelude
- **THEN** navigation resolves to the real prelude definition and an unsaved prelude edit updates dependent results

#### Scenario: Unrelated route defines the same name
- **WHEN** another route defines a helper absent from the selected route/prelude
- **THEN** that definition does not satisfy the current route's reference

#### Scenario: Ordered redefinition
- **WHEN** a later loaded source redefines an earlier name
- **THEN** analysis preserves both origins and resolves uses according to the selected program's binding/order semantics

#### Scenario: Shared helper depends on route redefinition
- **WHEN** two routes redefine a name used inside one shared prelude helper and a definition request originates in that helper
- **THEN** the result identifies route dependence without choosing a unique target or borrowing one route's globals for another

### Requirement: Yagel layers and file isolation

The Yagel rule profile SHALL use Lispico Clojure, stdlib, JSON, and Yagel declarations. It SHALL select effective files by relative path in embedded, packs, global, project order and analyze each rule in an isolated file scope. A winning unreadable or invalid file MUST NOT silently reveal an older layer. Only regular UTF-8 source files within configured bounds SHALL be scanned; symlinks MUST NOT be followed. Source-repository profiles SHALL analyze configured checkout sources rather than substituting installed binary snapshots.

#### Scenario: Project overrides embedded source
- **WHEN** a configured project layer supplies the same relative path as an embedded rule
- **THEN** the project file is active, the embedded file is identified as shadowed, and unrelated rule scopes remain independent

#### Scenario: Definition in another rule
- **WHEN** a name is defined in a different rule but is absent from the current rule and host bindings
- **THEN** it is not accepted merely because it exists somewhere in the workspace

#### Scenario: Unreadable override
- **WHEN** a winning source cannot be read
- **THEN** its context reports that failure without falling back to a lower layer's definitions

### Requirement: Yagel phase and workflow context

Yagel declarations SHALL distinguish values, pure functions, staging declarations, and runtime effects. Known callback/declaration boundaries SHALL determine provable phase availability. Dynamic workflow prelude names SHALL appear only in explicit workflow contexts. Uncertain dynamic call paths MUST NOT be reported as proven phase violations.

#### Scenario: Dynamic workflow helper in an ordinary rule
- **WHEN** an ordinary rule references `spawn-and-await` without a local definition
- **THEN** the workflow prelude does not make that name globally available

#### Scenario: Non-callable host binding
- **WHEN** a binding is verified as a host value
- **THEN** completion and hover identify it as a value rather than fabricating a function signature

### Requirement: Reproducible declared-source verification

Catalogs SHALL pair `source_files` with `source_fingerprint` when either is present, using the canonical path ordering, SHA-256 framing, and bounded source reads defined in the [approved shared contracts](../../contracts.md). Configured expected fingerprints MUST match the catalog, and configured source roots SHALL require successful declared-byte verification. Without source roots, provenance MUST remain explicitly catalog-only. Manifest completeness remains the owner's responsibility; matching declared bytes MUST NOT be presented as proof that unlisted inputs cannot affect declarations.

Every source read SHALL enforce regular-file, no-symlink, root-confinement properties throughout the read. Invalid paths, missing or unreadable inputs, resource breaches, and digest mismatches SHALL reject the complete catalog and invalidate previously loaded declarations.

#### Scenario: Source changes after successful load
- **WHEN** a declared metadata source changes after its catalog has successfully loaded
- **THEN** verification reports stale metadata and removes the catalog's previously derived symbols instead of retaining them

#### Scenario: Expected digest without a source checkout
- **WHEN** an independently configured expected fingerprint differs from the selected catalog fingerprint and no source root is available
- **THEN** the catalog is rejected rather than skipping the comparison

#### Scenario: Manifest attempts to escape its root
- **WHEN** a source path traverses outside its configured root or uses a symlinked path component
- **THEN** the read is rejected without opening outside-root content

### Requirement: Explicit installed-pack snapshot selection

A Yagel packs layer SHALL select exactly one source-directory root or inert installed-pack snapshot. Snapshot selection SHALL pin the snapshot's raw-byte fingerprint and verify its readable source payloads. The snapshot SHALL preserve the production engine's selected pack generation, ordered key mapping, and unreadable key claims under the [approved shared contracts](../../contracts.md). llsp MUST NOT discover or open a live installed packstore, boot a host, or execute rules. Snapshot provenance SHALL describe the exported selected set, not claim live installed-store freshness.

The snapshot and its payloads SHALL use the same bounded, race-safe, no-symlink confinement as catalog sources. Invalid replacement SHALL clear prior snapshot-derived results. A winning unreadable pack entry SHALL block lower layers; an explicitly selected higher-layer winner MAY replace it.

#### Scenario: Installed export has protected content
- **WHEN** an explicit Yagel export captures selected v1 packs
- **THEN** it holds the active-set lease through verification and copying and releases it on success or failure without starting a host session

#### Scenario: Pack declares a shadow key
- **WHEN** a selected pack lists a rule-relative path in its verified shadows declaration
- **THEN** the snapshot preserves that unprefixed key and the configured higher layers retain their normal precedence

#### Scenario: Snapshot replacement is corrupt
- **WHEN** a readable companion source fails its declared digest after an earlier valid snapshot was loaded
- **THEN** the snapshot is rejected and its previous definitions are not retained as a fallback
