# Proposal

## Why

The parent contract promises editor-level support for Lispico Clojure, Lispico CL, zhk, Yagel, and ordinary Common Lisp in Zed, all observable in real Zed sessions rather than via protocol smoke. Without an integrated acceptance record, an extension build could pass while editor behavior is silently degraded. This record assigns that final acceptance work so the parent's capability contract is verified end to end.

## What Changes

- Owner: **zed-lisp**. Original task coverage: 6.1, 6.2, 6.3, 6.4.
- Smoke-test both runtime dialects in actual Zed: completion, signatures, navigation, diagnostics, and valid snippets, with unsaved Unicode edits whose errors are replaced at the displayed source ranges.
- Smoke-test zhk in Zed: explicit Clojure mode, host completion, prelude navigation, route isolation, shared-file uncertainty, replacement roots, and unsaved-prelude invalidation, without executing a workflow.
- Smoke-test Yagel in Zed: host context, rule isolation, layers, proven phase restrictions, workflow opt-in, project-local catalog navigation, and installed-pack snapshot keys, without starting sessions or applying rules.
- Re-check plain Common Lisp recognition, highlighting, outline and text objects, and server settings, showing that missing Lispico tooling does not disrupt structural editing; record unrelated maintenance findings separately.
- Track implementation and evidence in this repository. Acceptance completes only after every acceptance item has verified evidence and every upstream dependency has separate implementation evidence; protocol smoke does not satisfy acceptance.

## Capabilities

No capability deltas. The parent owns all language, context, catalog, server, and diagnostic requirements; duplicating them here would create a second, weaker contract. The requirements this delivery must satisfy are the bundled cross-capability scenarios in the parent's design that an editor must observe: opt-in associations, explicit server language IDs, real completion and signatures, ranged diagnostics, isolated overlays, and present-but-not-executing host behavior.

## Dependencies

- [add-lispico-development-support](../archive/2026-10-01-add-lispico-development-support/proposal.md)
- [add-lispico-runtime-core-catalogs](../add-lispico-runtime-core-catalogs/proposal.md)
- [add-lispico-dialect-analyzer-checker](../add-lispico-dialect-analyzer-checker/proposal.md)
- [add-zhk-host-profile](../add-zhk-host-profile/proposal.md)
- [add-yagel-host-profile](../add-yagel-host-profile/proposal.md)
- [add-llsp-host-context](../add-llsp-host-context/proposal.md)
- [migrate-to-llsp](../archive/2026-10-04-migrate-to-llsp/proposal.md) (separate sibling change, completed and archived 2026-10-04)

The parent dependency means its completed corpus and schema resources and explicit upstream approval task 3.1, not its final archive. The shared-server migration in `migrate-to-llsp` was a prerequisite of the editor path and has completed with its gates met (G1, G2, G4, G5; G3 remains open upstream); this record no longer waits on it but does not own its content.

## Impact

Local planning record only until the owning implementation in this repository is complete. Preserve source formats, host execution behavior, and the selected runtime versions. No local archive updates canonical capability specs.