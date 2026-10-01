# Design

## Ownership and contract

**go-lispico** owns the implementation. This record carries original task 2.4 without duplicating the parent capability deltas. The parent design and its delta specs remain the authoritative semantic, catalog, context, phase, and evidence contract; changes to that contract require review before implementation.

## Deliverable

An inert owner-side producer plus the published core, dialect, stdlib/macros, and JSON catalogs. The catalogs describe Go-defined symbols: exact name, kind, namespace cell, verified argument count and range, rest behavior, parameter labels where known, documentation, verified result information where known, phase availability, and source location with a catalog-location fallback. Actual Lisp source remains the authority for its own definitions.

Unknown signature, parameter, and result information stays absent. A first argument-count guard is not a complete signature and must not be widened into permissive invented varargs.

## Delivery

Confirm approval and actual dependency versions in the owning repository. Establish regression contracts for uncovered behavior before implementation; reuse existing coverage for behavior already implemented. Deliver coherent edits, then verify through bounded project wrappers. No evaluator execution, fake metadata, unverified signatures, or source stubs may satisfy a requirement.

Testing depth: **existing-service-strict**. Deterministic contract evidence plus a real inert producer smoke are required. Editor acceptance happens in the acceptance record; protocol smoke alone is insufficient. Exact commands and symbol names belong to the reviewed implementation plan in that repository, not invented here.

## Reviewed owner execution plan

Consume the approved [shared contracts](../../../../changes/add-lispico-development-support/contracts.md) and the completed local schema/resource prerequisite. Implementation remains separately authorized upstream.

Planning baseline: go-lispico `12e0990147c41a78186ad9424fb70a6369fab544`; both hosts instead pin v0.14.0 at `c68e2c8452798b383833d56e58b388d7241f2413`. Generate host-adjacent runtime catalogs from that exact source tree. Do not upgrade hosts or claim checkout HEAD metadata describes the release. Preserve unrelated worktrees and concurrent edits.

Verified seams: `internal/inventory/registered.go` owns frozen stdlib names and CL adapter IDs; `plugins/stdlib/registered_surface_test.go` checks the registered surface; `plugins/stdlib/inventory_source_test.go` demonstrates source attribution through Go AST positions; `cl/cl.go` owns vocabulary and adapters; `core/dialect.go` owns forms; `plugins/stdlib/bootstrap.go` owns macros; `plugins/json/plugin.go` owns JSON declarations. The tag's CL builder form differs from the checkout's dialect-spec literal; do not build a general Go expression interpreter to bridge them.

1. Establish catalog-consumer contracts for enabled-library visibility, CL aliases and adapters, both verified CL cells, unknown-signature behavior, and source provenance. Reuse the frozen-surface tests; add only missing behavioral boundaries.
2. Implement the inert producer from authoritative inventories and explicitly source-verified metadata. Source parsing may locate names and spans; it must not infer a complete signature from an argument-count guard. Do not initialize an engine, expand macros, add fields to runtime values, or reflect over callables.
3. Emit core, stdlib/macros, and JSON catalogs per actual dialect and library identity, including aliases and adapters in the library that exposes them. Permit those libraries in host profiles; do not invent a separate vocabulary library and do not discard a proven value-cell registration.
4. Emit the paired bounded source manifest and fingerprint with actual tag provenance. Validate against schema version 2; test real source mutation and invalid replacement after a prior successful load through the consumer contract rather than through copied hash code.
5. Run the bounded project test, lint, and build targets plus a real inert producer smoke. Record produced artifacts, exact selected source inputs, revisions, commands, and observed results.

## Boundaries

The producer owns manifest completeness and metadata truth. The consumer owns filesystem verification and behavior; local schema tests alone cannot complete the consumer-side verification requirement. Catalog refresh is explicit — snapshots are never silently replaced, and a newer catalog is never silently selected over the configured one.

## Completion and archival

Keep tasks open until the owning change, revision, verification commands, and observed outcomes are recorded. Parent final readiness waits for all six ownership records, the shared-contract migration, and the separate shared-server migration. Introduce no canonical capability specs and do not archive prematurely or infer upstream approval from local decomposition approval.
