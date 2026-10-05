# Proposal

## Why

The parent contract promises Yagel declarations that distinguish root values, pure/staged/live functions, and workflow-only helpers with verified signatures and phases, plus explicit installed-pack snapshots that an editor can consume without opening a live packstore. The current Yagel source has the names and the active-set model, but no inert host export, no signature or phase enrichment, and no inert installed-pack snapshot. The existing `rules check` is v1-only and not a complete oracle for the production engine's v2 branch. Narrowing this to generic dialect support would let names-only results substitute for verified declarations.

## What Changes

- Owner: **Yagel**. Original task coverage: 3.3 and the Yagel portion of 3.4.
- Publish source-backed declarations distinguishing root values, pure/staged/live functions, and workflow-only helpers, covering names, signatures, phases, and unknown-signature behavior without booting a live session.
- Export schema version 2 source provenance alongside the host and workflow catalogs.
- Implement an explicit installed-pack capture under the approved lease and self-contained-copy rules: caller-selected store, current selection semantics, complete verified snapshot and payload bundle, no ambient store discovery, no host boot.
- Deliver source-profile and installed-store fixture evidence for layer precedence, per-file isolation, unreadable winners, and failed snapshot replacement.
- Track implementation and evidence in a separately approved change in the owning repository. This local record does not authorize external edits and completes only after the owning implementation and every acceptance item have verified evidence.

## Capabilities

No capability deltas. The parent owns all language, context, catalog, server, and diagnostic requirements; duplicating them here would create a second, weaker contract. The requirements this delivery must satisfy are `Inert versioned host and library declarations`, `Yagel layers and file isolation`, `Yagel phase and workflow context`, `Reproducible declared-source verification`, and `Explicit installed-pack snapshot selection` in the parent's `lispico-project-context` delta, together with the shared contracts for declared-source fingerprints and installed-pack snapshots.

## Dependencies

- [add-lispico-development-support](../archive/2026-10-01-add-lispico-development-support/proposal.md)
- [add-lispico-runtime-core-catalogs](../add-lispico-runtime-core-catalogs/proposal.md)
- [add-zhk-host-profile](../add-zhk-host-profile/proposal.md)

The parent dependency means its completed corpus and schema resources and explicit upstream approval task 3.1, not its final archive. The zhk host profile lands first: both host profiles produce declared-source fingerprints under the same shared contract, and Yagel follows the zhk producer so the two hosts cannot diverge on canonical framing. The host-context record is a downstream consumer; this record does not own its visibility boundary or the consumer's verification of the snapshot.

## Impact

Local planning record only until the owning implementation is separately approved. Preserve source formats, host execution behavior, and the selected runtime versions. No local archive updates canonical capability specs.