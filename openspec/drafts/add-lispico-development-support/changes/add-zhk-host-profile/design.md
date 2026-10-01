# Design

## Ownership and contract

**zhk** owns the implementation. This record carries original tasks 3.2 and the zhk portion of 3.4 without duplicating the parent capability deltas. The parent design and its delta specs remain the authoritative semantic, catalog, context, phase, and evidence contract; changes to that contract require review before implementation.

## Deliverable

An inert host-declaration export from the actual primitive registry, schema version 2 host metadata with exact owner revision and bounded source manifest, and portable order and isolation fixtures. The raw primitive's signature follows its own guard, not an assumed registry shape; unknown metadata is omitted rather than invented.

## Delivery

Confirm approval and actual dependency versions in the owning repository. Establish regression contracts for uncovered behavior before implementation; reuse existing coverage for behavior already implemented. Deliver coherent edits, then verify through bounded project wrappers. No evaluator execution, fake metadata, unverified signatures, or source stubs may satisfy a requirement.

Testing depth: **existing-service-strict**. Deterministic contract evidence plus an inert export smoke are required. Editor acceptance happens in the acceptance record; protocol smoke alone is insufficient. Exact commands and symbol names belong to the reviewed implementation plan in that repository, not invented here.

## Reviewed owner execution plan

Consume the approved [shared contracts](../../../../changes/add-lispico-development-support/contracts.md) and the completed local schema/resource prerequisite. Implementation remains separately authorized upstream.

Planning baseline advanced to zhk `5052b8e1720814daa61ccc92a7e498c5e027ddfa` during reconnaissance. Refresh anchors before dispatch and preserve other active worktrees. Runtime stays go-lispico v0.14.0.

Verified seams: `internal/engine/engine.go` contains `primitives`, argument decoding and arity checks, `load`, and ordered evaluation; `workflows/embed.go` owns the embedded tree. Existing `internal/engine/lifecycle_test.go`, `norun_test.go`, and workflow tests already protect replacement roots and refusal behavior; reuse them rather than redesigning runtime gates.

1. Establish consumer contracts for verified zero, optional, and fixed arities, qualified names, omitted unknown metadata, and source navigation.
2. Implement inert declaration export from the actual registry without initializing an evaluation. A `recording` field is not purity or execution-phase metadata; type-code letters are not invented parameter labels.
3. Export schema-2 host metadata with an exact owner revision and bounded source manifest. Keep runtime core/stdlib/JSON catalogs separate and pinned to v0.14.0.
4. Deliver portable order and isolation fixtures: lexical `lib/*.lisp`, selected route `main.lisp`, auxiliary exclusion, explicit wholesale replacement roots, redefinition order, and shared-prelude route uncertainty. Do not read `ZHK_WORKFLOWS` implicitly in the analyzer.
5. Verify through the bounded engine package test, relevant workflow tests, the project test and lint targets, and an inert export smoke. Record actual results and refresh stale source excerpts or templates only in the separately owned zed-lisp resource work.

## Boundaries

Exporter interfaces and new tests remain new until the owner appendix fixes exact sites. This plan authorizes neither workflow execution nor unrelated kernel compatibility repair. The analyzer-facing surface remains a portable description of ordering, replacement roots, and exclusion, not a contract that exposes runtime internals.

## Completion and archival

Keep tasks open until the owning change, revision, verification commands, and observed outcomes are recorded. Parent final readiness waits for all six ownership records, the shared-contract migration, and the separate shared-server migration. Introduce no canonical capability specs and do not archive prematurely or infer upstream approval from local decomposition approval.