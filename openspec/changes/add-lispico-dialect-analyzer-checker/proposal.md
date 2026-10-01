# Proposal

## Why

The parent contract promises dialect-correct parsing, resolution, and diagnostics with no runtime effects, and requires the editor and batch checker to report the same findings on the same snapshot. The shared `llsp` server already implements the `common-lisp`, `lispico-clojure`, and `lispico-cl` dialects, but it does not yet match the reviewed runtime's binding semantics in every covered case, and its batch surface is only partly characterized. Narrowing this to generic dialect support would let a generic analyzer's conventions substitute for the runtime's actual behavior.

## What Changes

- Owner: **llsp, with go-lispico parity fixtures**. Original task coverage: 2.1, 2.2, 2.3, 2.5.
- Verify and complete source-preserving parsing, recovery, and position conversion without changing runtime values or the evaluator hot path.
- Verify and complete lexical and namespace resolution and form checks against the reviewed runtime's binding semantics, CL adapters, both verified CL cells, and known macros, with dynamic-analysis limits documented and no speculative errors for unsupported macro expansion.
- Verify and complete shared diagnostic records and a bounded checker interface over `llsp`'s existing `check` subcommand: human and JSON output, exact ranges and stable codes, exit 0/1/2, configuration-failure handling, and no source writes or side effects.
- Track implementation and evidence in a separately approved change in the owning repository. This local record does not authorize external edits and completes only after the owning implementation and every acceptance item have verified evidence.

## Capabilities

No capability deltas. The parent owns all language, context, catalog, server, and diagnostic requirements; duplicating them here would create a second, weaker contract. The requirements this delivery must satisfy are `Dialect and form validation`, `Scope and known-call diagnostics`, `Host availability diagnostics`, `Ranged diagnostics and replacement`, `Analysis has no runtime effects`, and `Shared editor and batch results` in the parent's `lispico-static-diagnostics` delta, together with the analyzer-related clauses of the `lispico-language-server` delta.

## Dependencies

- [add-lispico-development-support](../../../../changes/add-lispico-development-support/proposal.md)
- [add-lispico-runtime-core-catalogs](../add-lispico-runtime-core-catalogs/proposal.md)

The parent dependency means its completed corpus and schema resources and explicit upstream approval task 3.1, not its final archive. Catalog and host-context records are downstream consumers; the analyzer does not own their visibility.

## Impact

Local planning record only until the owning implementation is separately approved. Preserve source formats, host execution behavior, and the selected runtime versions. No local archive updates canonical capability specs.