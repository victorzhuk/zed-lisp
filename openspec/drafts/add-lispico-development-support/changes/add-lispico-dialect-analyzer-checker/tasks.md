# Tasks

Original task identifiers remain stable for parent references. All tasks are pending; existing-service-strict evidence is required.

- [ ] 2.1 Create the separately approved upstream analysis and checker change. Verify actual CLI and configuration behavior and document ownership before implementation.
- [ ] 2.2 Verify and complete source-preserving parsing, recovery, and position conversion without changing runtime values. Cover dialect parity, incomplete input, Unicode ranges, and depth and size limits.
- [ ] 2.3 Verify and complete lexical and namespace resolution and form checks. Cover CL adapters, parameter and rest bindings, sequential sibling visibility for both `let` and `let*`, quotation, catch bindings, and known macros; document dynamic-analysis limits.
- [ ] 2.5 Verify and complete shared diagnostic records and the bounded checker interface. Cover human and JSON output, exact ranges and codes, exit 0/1/2, configuration failures, and no source writes or effects; document only verified options.

## Required evidence

- Characterization fixtures exercise sequential `let` and `let*` sibling visibility, `loop` initializers, catch parameters, CL adapters, both verified CL cells, quotation depth, and a known macro whose expansion is left uncertain.
- A mixed-dialect visibility fixture resolves a same-dialect helper and reports the expected diagnostic for a helper defined only in the other dialect when the unresolved-call lint is enabled.
- One saved source-byte snapshot is checked through both checker and LSP after encoding normalization; codes, severities, and ranges match.
- Invalid configuration exits with the documented status; human and JSON outputs retain diagnostic codes, severities, paths, and ranges.
- Effectful top-level sources, macro bodies, and plugin registrations are not exercised by indexing, hover, or the batch checker.
- `task test` and `task lint` plus a real checker and LSP smoke are recorded with commands and observed output.

## Not covered by this record

Catalog production, host profile export, host-aware context loading, and editor behavior belong to their own records. This record introduces no capability deltas and no editable surface in the extension repository.