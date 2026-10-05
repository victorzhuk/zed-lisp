# Design

## Ownership and contract

**llsp** owns the implementation. This record carries original tasks 4.1 through 4.6 without duplicating the parent capability deltas. The parent design and its delta specs remain the authoritative semantic, catalog, context, phase, and evidence contract; changes to that contract require review before implementation.

## Deliverable

Typed project, catalog, and snapshot loading and verification; a centrally context-scoped symbol visibility boundary carrying name and cell identity and declaration provenance; zhk ordered route snapshots with shared-prelude uncertainty; Yagel source layers and verified installed-pack snapshot consumption; and versioned-document lifecycle with invalidation, cancellation, and stale-result suppression.

## Delivery

Confirm approval and actual dependency versions in the owning repository. Establish regression contracts for uncovered behavior before implementation; reuse existing coverage for behavior already implemented. Deliver coherent edits, then verify through bounded project wrappers. No evaluator execution, fake metadata, unverified signatures, or source stubs may satisfy a requirement.

Testing depth: **existing-service-strict**. Deterministic contract evidence plus real checker and LSP smoke are required. Editor acceptance happens in the acceptance record; protocol smoke alone is insufficient. Exact commands and symbol names belong to the reviewed implementation plan in that repository, not invented here.

## Reviewed owner execution plan

Consume the approved [shared contracts](../archive/2026-10-01-add-lispico-development-support/contracts.md), the completed local schema/resource prerequisite, the runtime catalogs, the analyzer parity work, and both host profiles. Metadata and profile producers can develop independently after the wire contracts are fixed; final integration waits for all of them.

Planning baseline: llsp v0.2.1 at `436bc84c0f520c424d6b7a1c086f38e1ce0448e8`. The current global name store and boolean defined-name predicate cannot carry contextual arity, cell, phase, and provenance on their own.

Verified seams: `src/config.rs` (`Layers::load_project`), `src/document.rs` (`Settings::detect`), `src/workspace.rs` (`Index`, source discovery), `src/diagnostics.rs`, `src/server.rs`, `src/main.rs`, and `src/features`.

1. Establish shared typed project, catalog, and snapshot loading and verification contracts. Resolve paths from the selected JSON file, reject overlapping contexts, enforce source manifests and anchored reads, and keep ordinary non-Lispico configuration behavior intact. Explicit project-selection API spelling is new and must be sealed before publishing options.
2. Enforce symbol visibility through a centrally context-scoped lookup boundary with name and cell identity and declaration provenance. Do not inject host declarations into global dialect builtins or suppress unresolved symbols as a substitute for loading metadata.
3. Implement zhk ordered route snapshots and shared-prelude uncertainty; preserve original binding locations and unsaved-prelude invalidation. Indexing never implies importing another route.
4. Implement Yagel source layers and verified installed-pack snapshots with logical keys, per-rule scopes, shadowed-file state, and only proven phase and workflow restrictions. llsp must not open installed packstores or execute host code.
5. Cover versioned documents, cancellation, bounded indexing, catalog, configuration, and snapshot invalidation, explicit-context file watchers, and failed prior-success replacement. Empty Lispico extension lists must not prevent configured sources from being watched.
6. Run bounded `task test` and `task lint` and real saved-snapshot checker and LSP parity, including encoding normalization, navigation, arity, missing metadata, independent worktrees, and no effects. Actual Zed acceptance remains separate.

## Boundaries

Permanent tests drive actual request and checker behavior and error transitions. No global CLI encoding change, context shim, ignored invalid configuration, or stale-result fallback satisfies the contract. Diagnostics and checker records are shared with the analyzer record rather than duplicated here.

## Completion and archival

Keep tasks open until the owning change, revision, verification commands, and observed outcomes are recorded. Parent final readiness waits for all six ownership records, the shared-contract migration, and the separate shared-server migration. Introduce no canonical capability specs and do not archive prematurely or infer upstream approval from local decomposition approval.