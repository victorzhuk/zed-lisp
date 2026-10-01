# Proposal

## Why

The Lispico editor contract promises verified names, signatures, aliases, adapters, macro availability, and provenance for Go-defined symbols in the runtime itself. Those facts live in go-lispico, and no public API currently exposes them: the CLI executes files or starts a REPL, and `GoFunc` carries a name and a callable without signatures, documentation, or source locations. This record assigns that producer work so the parent capability contract is delivered rather than narrowed to generic dialect support.

## What Changes

- Owner: **go-lispico**. Original task coverage: 2.4.
- Add an inert, owner-side declaration producer driven by the authoritative inventories and explicitly source-verified metadata, emitting core, dialect, stdlib/macros, and JSON catalogs under schema version 2.
- Publish the paired bounded source manifest and fingerprint, plus actual tag provenance, so a consumer can detect changed declaration-determining inputs.
- Track implementation and evidence in a separately approved change in the owning repository. This local record does not authorize external edits and completes only after the owning implementation and every acceptance item have verified evidence.

## Capabilities

No capability deltas. The parent owns all language, context, catalog, server, and diagnostic requirements; duplicating them here would create a second, weaker contract. The requirements this delivery must satisfy are `Inert versioned host and library declarations`, `Library availability is explicit`, and `Reproducible declared-source verification` in the parent's `lispico-project-context` delta, together with the shared contracts for declared-source fingerprints and declaration semantics.

## Dependencies

- [add-lispico-development-support](../../../../changes/add-lispico-development-support/proposal.md)

The parent dependency means its completed corpus and schema resources and explicit upstream approval task 3.1, not its final archive. Parent final readiness depends on this record; no circular completion requirement is intended.

Sibling records that consume these catalogs — the analyzer/checker parity work, both host profiles, host-aware context, and editor acceptance — depend on this one, not the reverse.

## Impact

Local planning record only until the owning implementation is separately approved. Preserve source formats, host execution behavior, and the selected runtime versions. No local archive updates canonical capability specs. The parent cannot complete or archive until this record has real upstream implementation evidence.
