# Tasks

Original task identifiers remain stable for parent references. All tasks are pending; existing-service-strict evidence is required.

- [ ] 4.1 Implement explicit project loading, glob selection, catalog matching, configured roots, and bounded scans. Verify overlap rejection, no symlink traversal, external-root opt-in, unmatched files, and no ambient discovery.
- [ ] 4.2 Implement zhk ordered sequential-program resolution with binding origins. Verify route and prelude navigation, redefinition order, shared-prelude uncertainty, auxiliary exclusion, and route isolation.
- [ ] 4.3 Implement Yagel overlays and isolated rule and workflow contexts. Verify winners, no cross-rule resolution, proven phase restrictions, workflow-only helpers, and shadowed-file state.
- [ ] 4.4 Complete host-aware completion, hover, signatures, definitions, references, symbols, and diagnostics over real stdio LSP. Verify catalog-backed requests and negotiated position accuracy; retain the existing llsp lifecycle and dialect-reload guarantees.
- [ ] 4.5 Verify and complete versioned-buffer handling, cancellation, bounded indexing, reload and invalidation, and stale-result suppression. Cover edits, close, delete, restart, invalid configuration, independent worktrees, and context switches.
- [ ] 4.6 Verify checker and server parity on identical snapshots and zero execution for effectful forms, macros, and plugins. Record bounded upstream commands and observed results.

## Required evidence

- Overlapping configured contexts are rejected; symlinked path components are refused throughout a read; external roots require explicit configuration; unmatched files keep syntax support with a visible missing-context message.
- Declared-source reads are anchored, bounded, and no-symlink; any invalid input clears the catalog's previously derived symbols rather than retaining them; catalog-only provenance is labeled and never presented as installed-binary compatibility.
- A zhk route resolves its ordered prelude helper to the real definition; another route redefining the same name does not satisfy a reference inside the selected route; an unsaved prelude edit updates dependent results and clears obsolete diagnostics.
- A Yagel project layer supplying the same relative path as an embedded rule makes the project file active and identifies the embedded file as shadowed; an unreadable winning file blocks the lower layer rather than falling back silently.
- A Yagel packs layer selecting a snapshot pins and verifies the raw-byte digest and every readable payload; a corrupt companion after an earlier valid snapshot is rejected and its previous definitions are not retained as a fallback.
- Checker and server report identical codes, severities, and ranges on one saved snapshot after encoding normalization; effectful top-level sources, macro bodies, and plugin registrations are not exercised by indexing, hover, or the batch checker.
- Bounded `task test` and `task lint` plus real saved-snapshot checker and LSP runs are recorded with commands and observed output.

## Not covered by this record

Catalog production, host profile export, and editor behavior belong to their own records. The separate shared-server migration in `migrate-to-llsp` owns server registration, identifiers, resolution, cache, and settings forwarding; this record consumes that migration rather than redoing it. This record introduces no capability deltas and no editable surface in the extension repository.