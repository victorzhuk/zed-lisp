# Design

## Ownership and contract

**zed-lisp** owns the implementation. This record carries original tasks 6.1 through 6.4 without duplicating the parent capability deltas. The parent design and its delta specs remain the authoritative semantic, catalog, context, phase, and evidence contract; changes to that contract require review before implementation.

## Deliverable

Integrated acceptance evidence from real Zed sessions for both runtime dialects, zhk, Yagel, and ordinary Common Lisp. Each session records exact extension, server, runtime, host, catalog, and snapshot revisions plus fixture selection, and observes completion, signatures, navigation, diagnostics, phase and layer behavior, and missing-tooling degradation without executing host code.

## Delivery

Confirm approval and actual dependency versions in this repository. Establish regression contracts for uncovered behavior before implementation; reuse existing coverage for behavior already implemented. Deliver coherent edits, then verify through bounded project wrappers. No evaluator execution, fake metadata, unverified signatures, or source stubs may satisfy a requirement.

Testing depth: **existing-service-strict**. Deterministic contract evidence plus actual Zed observations are required; protocol smoke alone is insufficient. Exact commands and symbol names belong to the reviewed implementation plan in this repository, not invented here.

## Reviewed acceptance sequence

The approved [shared contracts](../archive/2026-10-01-add-lispico-development-support/contracts.md) and the completed local schema/resource prerequisite are available. The independent server migration (`migrate-to-llsp`) completed and was archived on 2026-10-04; the remaining wait is every owner implementation. Do not treat protocol smoke as actual Zed proof.

1. Record exact extension, `llsp`, runtime, host, catalog, and snapshot revisions plus fixture selection. Re-check opt-in associations and explicit server language IDs without changing ordinary Common Lisp defaults.
2. Exercise both runtime dialects in Zed: real completion, signatures, references and definitions, known diagnostics, and valid snippets. Use unsaved Unicode edits and observe error replacement at the displayed source ranges.
3. Exercise zhk ordered-prelude navigation, redefinitions, route isolation, shared-file uncertainty, replacement roots, and unsaved-prelude invalidation without executing a workflow.
4. Exercise Yagel host values and signatures, workflow opt-in, proven phase boundaries, source overlays, installed snapshot keys, unreadable winners, and independent rule scopes without starting sessions or applying rules.
5. Re-check ordinary Common Lisp, restart, reload, and context changes, missing-server behavior, catalog and source navigation with and without owner checkouts, and failed metadata replacement. Run bounded local wrappers once for the coherent implementation set and record observations, not inferred UI success.

## Boundaries

No permanent test should assert a copied registration table or incidental template default merely to raise counts. Unrelated maintenance findings are recorded separately; they are not prerequisites for this acceptance.

## Completion and archival

Keep tasks open until the owning change, revision, verification commands, and observed outcomes are recorded. Parent final readiness waits for all six ownership records, the shared-contract migration, and the separate shared-server migration. Introduce no canonical capability specs and do not archive prematurely or infer upstream approval from local decomposition approval.