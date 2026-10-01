# Proposal

## Why

The parent contract promises that a zhk route analyzes against its own ordered prelude and route `main.lisp`, with binding origins preserved, route isolation enforced, and a wholesale replacement workflow root honored, all without the analyzer executing a workflow. Those facts live in zhk's primitive registry and loader. This record assigns that producer and fixture work so the parent capability contract is delivered rather than approximated by a generic dialect analysis.

## What Changes

- Owner: **zhk**. Original task coverage: 3.2 and the zhk portion of 3.4.
- Derive names and verified arity and type facts from the primitive registry without changing execution behavior: every entry, optional and zero arguments, source labels, documentation, references, and zhk/sh included.
- Export schema version 2 host metadata with an exact owner revision and bounded source manifest, keeping runtime core/stdlib/JSON catalogs separate and pinned to v0.14.0.
- Publish portable source-layout fixtures for ordered preludes, route isolation, and replacement workflow roots that distinguish source-checkout from installed-runtime context and declare semantics without executing a workflow.
- Track implementation and evidence in a separately approved change in the owning repository. This local record does not authorize external edits and completes only after the owning implementation and every acceptance item have verified evidence.

## Capabilities

No capability deltas. The parent owns all language, context, catalog, server, and diagnostic requirements; duplicating them here would create a second, weaker contract. The requirements this delivery must satisfy are `Inert versioned host and library declarations`, `Library availability is explicit`, and `zhk ordered workflow visibility` in the parent's `lispico-project-context` delta, together with the shared contracts for declared-source fingerprints and the runtime profile gating.

## Dependencies

- [add-lispico-development-support](../../../../changes/add-lispico-development-support/proposal.md)
- [add-lispico-runtime-core-catalogs](../add-lispico-runtime-core-catalogs/proposal.md)

The parent dependency means its completed corpus and schema resources and explicit upstream approval task 3.1, not its final archive. The host-context record is a downstream consumer; this record does not own its visibility boundary.

## Impact

Local planning record only until the owning implementation is separately approved. Preserve source formats, host execution behavior, and the selected runtime versions. No local archive updates canonical capability specs.