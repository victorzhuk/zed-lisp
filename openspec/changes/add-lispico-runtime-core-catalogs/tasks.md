# Tasks

Original task identifiers remain stable for parent references. All tasks are pending; existing-service-strict evidence is required.

- [ ] 2.4 Publish inert core, dialect, stdlib, and JSON catalogs with verified signatures and provenance. Verify names, aliases, adapter arities, macros, disabled-library isolation, both verified CL cells, and explicit snapshot refresh against authoritative runtime sources.

## Required evidence

- Catalog artifacts validate against schema version 2, with the emitted library identities, dialect, and cell layout matching the runtime's actual registration surface.
- Deterministic parity tests compare published names, aliases, adapter arities, and macros against the authoritative inventories for the pinned source tree.
- A test mutates a real declaration-determining source input and observes the catalog invalidated through the consumer contract, including after a previously successful load.
- A catalog emitted with only one of `source_files`/`source_fingerprint`, with an out-of-order manifest, or over the bounded envelope is rejected.
- Bounded project test, lint, and build runs plus an inert producer smoke are recorded with commands and observed output.

## Not covered by this record

Execution effects, editor behavior, host loading order, and installed-pack snapshots belong to the analyzer/checker, host profile, host-context, and acceptance records. This record introduces no capability deltas and no editable surface in the extension repository.
