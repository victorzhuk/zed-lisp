# Design

## Ownership and contract

**Yagel** owns the implementation. This record carries original tasks 3.3 and the Yagel portion of 3.4 without duplicating the parent capability deltas. The parent design and its delta specs remain the authoritative semantic, catalog, context, phase, and evidence contract; changes to that contract require review before implementation.

## Deliverable

Source-backed declarations with verified signatures and phases for root values, pure/staged/live functions, and workflow-only helpers; schema version 2 source provenance; an explicit inert installed-pack export producing a complete verified snapshot and payload bundle with logical keys, shadow handling, and selected-generation semantics; and source-profile plus installed-store fixtures covering layer precedence, per-file isolation, unreadable winners, and failed snapshot replacement.

## Delivery

Confirm approval and actual dependency versions in the owning repository. Establish regression contracts for uncovered behavior before implementation; reuse existing coverage for behavior already implemented. Deliver coherent edits, then verify through bounded project wrappers. No evaluator execution, fake metadata, unverified signatures, or source stubs may satisfy a requirement.

Testing depth: **existing-service-strict**. Deterministic contract evidence plus a real inert exporter smoke are required. Editor acceptance happens in the acceptance record; protocol smoke alone is insufficient. Exact commands and symbol names belong to the reviewed implementation plan in that repository, not invented here.

## Reviewed owner execution plan

Consume the approved [shared contracts](../../../../changes/add-lispico-development-support/contracts.md) and the completed local schema/resource prerequisite. Implementation remains separately authorized upstream. Start after the [zhk host profile](../add-zhk-host-profile/proposal.md) has landed its declared-source fingerprint producer: Yagel reuses that framing rather than defining a second one, so the two host catalogs verify under one canonical form.

Planning baseline: Yagel `a984f4eea2615e2744521321f586f77f9ee698d1`, with unrelated Makefile and OpenSpec edits and active worktrees. Earlier `332ac33e` reconnaissance is stale; use the current primitive-name, staging, workflow-prelude, and packstore seams. Runtime stays go-lispico v0.14.0.

Verified seams: `internal/core/primitive_names.go`, `internal/core/staging.go`, `internal/primitives/workflow_prelude.go`, `internal/cli/rules_surface.go`, `internal/rules/rules.go` and `playbooks.go`, `internal/core/engine.go` (`packLayerFS`), `internal/packstore/layer.go`, and `internal/packstore/v2.go` (`CaptureActiveV2`). The current engine v1 preference and the existing checker's v1-only acquisition are distinct; do not silently repair either here.

1. Establish consumer contracts for actual host values and functions, verified signatures and phases, unknown metadata, and workflow-only helpers. Use current source-backed name inventories, not stale registration counts or guessed generic varargs.
2. Implement inert host and workflow catalog export with schema version 2 source provenance. Do not boot a Core, start a session, expand macros, or execute rules. Parameter and phase enrichment requires actual source evidence.
3. Implement explicit installed-pack capture under the approved lease and self-contained-copy rules. Preserve current selected generations, lexical pack order, shadows keys, collection errors, and unreadable winners. Publish only a complete verified snapshot and payload bundle; no ambient store discovery.
4. Deliver source-profile and installed-store fixture evidence for relative-key precedence, per-file isolation, unreadable winners, higher-layer replacement, and failed snapshot replacement. Installed-store parity requires deterministic store fixtures, not only source-directory packs.
5. Run the bounded fast target for the affected CLI, core, rules, and packstore packages, the existing relevant rule checks, and a real inert exporter smoke. Preserve the dirty Makefile; adding a wrapper is not mandatory if it would collide with another session. Record actual byte and provenance evidence plus the no-execution evidence.

## Boundaries

Real installed-store access remains a separately approved implementation and verification action. New exporter interfaces and snapshot structs must be sealed in the owning appendix; no new second checker is introduced here. The export's contract is fixed by the parent's `lispico-project-context` and `shared contracts` documents; changes require review.

## Completion and archival

Keep tasks open until the owning change, revision, verification commands, and observed outcomes are recorded. Parent final readiness waits for all six ownership records, the shared-contract migration, and the separate shared-server migration. Introduce no canonical capability specs and do not archive prematurely or infer upstream approval from local decomposition approval.