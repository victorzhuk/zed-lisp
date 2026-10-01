# Proposal

## Why

The parent contract promises that the shared `llsp` server can read an explicit project file, match catalogs and snapshots, isolate a zhk route from its peers, respect Yagel layer precedence and installed-pack snapshot selection, and surface host-aware results without ever opening a live installed packstore, executing a host session, or applying host effects. The current llsp configuration has no host, project, or catalog model. Narrowing this to dialect-only support would either re-implement host primitives as global builtins or accept unknown-symbol cascades that mask real context failures.

## What Changes

- Owner: **llsp**. Original task coverage: 4.1, 4.2, 4.3, 4.4, 4.5, 4.6.
- Implement explicit project loading, glob selection, catalog matching, configured roots, and bounded scans, with overlap rejection, no symlink traversal, external-root opt-in, unmatched-file handling, and no ambient discovery.
- Enforce symbol visibility through a centrally context-scoped lookup boundary carrying name and cell identity and declaration provenance, without injecting host declarations into global dialect builtins or suppressing unresolved symbols as a substitute for loading metadata.
- Implement zhk ordered route snapshots and shared-prelude uncertainty, preserving original binding locations and unsaved-prelude invalidation.
- Implement Yagel source layers and verified installed-pack snapshots with logical keys, per-rule scopes, shadowed-file state, and only proven phase and workflow restrictions.
- Verify and complete versioned-buffer handling, cancellation, bounded indexing, reload and invalidation, stale-result suppression, and checker/server parity on identical snapshots with zero execution.
- Track implementation and evidence in a separately approved change in the owning repository. This local record does not authorize external edits and completes only after the owning implementation and every acceptance item have verified evidence.

## Capabilities

No capability deltas. The parent owns all language, context, catalog, server, and diagnostic requirements; duplicating them here would create a second, weaker contract. The requirements this delivery must satisfy are `Portable deterministic project configuration`, `Unambiguous context selection and reload`, `Library availability is explicit`, `Reproducible declared-source verification`, `Explicit installed-pack snapshot selection`, `Contextual completion and documentation`, `Source navigation and symbols`, `Document lifecycle and position accuracy`, and `Bounded responsive service` across the parent's `lispico-project-context` and `lispico-language-server` deltas, together with the shared contracts for declared-source fingerprints and installed-pack snapshots.

## Dependencies

- [add-lispico-development-support](../../../../changes/add-lispico-development-support/proposal.md)
- [add-lispico-runtime-core-catalogs](../add-lispico-runtime-core-catalogs/proposal.md)
- [add-lispico-dialect-analyzer-checker](../add-lispico-dialect-analyzer-checker/proposal.md)
- [add-zhk-host-profile](../add-zhk-host-profile/proposal.md)
- [add-yagel-host-profile](../add-yagel-host-profile/proposal.md)

The parent dependency means its completed corpus and schema resources and explicit upstream approval task 3.1, not its final archive. Catalog and host-profile producers can develop independently after the wire contracts are fixed; final integration waits for all of them. Editor acceptance happens in its own record.

## Impact

Local planning record only until the owning implementation is separately approved. Preserve source formats, host execution behavior, and the selected runtime versions. No local archive updates canonical capability specs.