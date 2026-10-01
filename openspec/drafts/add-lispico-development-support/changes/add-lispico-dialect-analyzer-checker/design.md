# Design

## Ownership and contract

**llsp, with go-lispico parity fixtures** owns the implementation. This record carries original tasks 2.1, 2.2, 2.3, 2.5 without duplicating the parent capability deltas. The parent design and its delta specs remain the authoritative semantic, catalog, context, phase, and evidence contract; changes to that contract require review before implementation.

## Deliverable

A bounded analyzer and checker surface in the shared `llsp` server, expressed through the existing recovering parser, cells, checker records, and settings-reload retention. It does not introduce a second parser or checker, and it does not reinterpret the existing 32/32 dialect smoke as host-aware completion.

## Delivery

Confirm approval and actual dependency versions in the owning repository. Establish regression contracts for uncovered behavior before implementation; reuse existing coverage for behavior already implemented. Deliver coherent edits, then verify through bounded project wrappers. No evaluator execution, fake metadata, unverified signatures, or source stubs may satisfy a requirement.

Testing depth: **existing-service-strict**. Deterministic contract evidence plus real checker and LSP smoke are required. Editor acceptance happens in the acceptance record; protocol smoke alone is insufficient. Exact commands and symbol names belong to the reviewed implementation plan in that repository, not invented here.

## Reviewed owner execution plan

Planning baseline: llsp v0.2.1 at `436bc84c0f520c424d6b7a1c086f38e1ce0448e8`. Reuse the existing recovering parser, cells, checker records, and settings-reload retention. Runtime parity uses the host-pinned v0.14.0 source and its namespace and binding tests, not unrelated Lisp conventions.

Verified seams: `dialects/lispico-cl.toml`, `dialects/lispico-clojure.toml`, `src/analysis.rs`, `src/analysis/tests.rs`, `src/diagnostics.rs`, `src/main.rs`, `tests/lispico.rs`, `tests/cli.rs`, and `tests/support`.

1. Characterize uncovered dialect behavior before edits: sequential `let` and `let*` sibling visibility, distinct `loop` initializer semantics, catch parameters, CL adapters and both verified cells, quotation depth, and known-macro uncertainty. Reuse existing recovery and cell tests; unsupported macro expansion must not become speculative errors.
2. Add a meaningful mixed-dialect visibility contract with unresolved-call lint enabled: a same-dialect helper resolves while a helper defined only in the other dialect does not satisfy the call and produces the expected unresolved result. A clean exit under disabled lint is not isolation evidence.
3. Compare checker and server diagnostic locations through one saved source-byte snapshot and normalized negotiated encoding. Do not globally switch CLI columns to UTF-16 merely to make raw numbers equal. Preserve documented CLI behavior unless a separate API change is approved.
4. Cover actual invalid-config exit status, machine and human output, unknown signature handling, and bounded operation. Document only verified existing `check` options and explicit dynamic-analysis limits; no invented host-project option is advertised before implementation.
5. Run `task test` and `task lint` with their existing limits, plus real checker and LSP smoke. Record gaps versus the full host contract. Catalog, context, and phase completion belong to the separate host-context record.

## Boundaries

Every new regression-test identifier is new until sealed against the live upstream harness. Existing behavior is not marked complete from this local planning record. The analyzer remains a configuration-consumer; its visibility model is centrally context-scoped and integrated with the host-context record rather than reinjected here.

## Completion and archival

Keep tasks open until the owning change, revision, verification commands, and observed outcomes are recorded. Parent final readiness waits for all six ownership records, the shared-contract migration, and the separate shared-server migration. Introduce no canonical capability specs and do not archive prematurely or infer upstream approval from local decomposition approval.