# Tasks

Original task identifiers remain stable for parent references. All tasks are pending; existing-service-strict evidence is required.

- [ ] 3.2 Derive names and verified arity and type facts from the primitive registry. Verify every entry, optional and zero arguments, source labels, documentation, references, and zhk/sh without changing execution behavior.
- [ ] 3.4 (zhk) Publish portable source-layout fixtures for ordered preludes, route isolation, and replacement workflow roots. Verify source-checkout versus installed-runtime context; declare semantics for the analyzer without executing workflows.

## Required evidence

- Every primitive-registry entry is exported with its own guard-derived arity; type-code letters are not treated as parameter labels and unknown metadata is omitted rather than invented.
- Schema-2 metadata carries a matching owner revision and bounded source manifest; runtime core/stdlib/JSON catalogs are kept separate and pinned to v0.14.0.
- Fixtures prove that the analyzer resolves a route helper to its real prelude definition, that another route redefining the same name does not satisfy a reference inside the selected route, that ordered redefinition preserves both origins, that auxiliary files do not contribute executed definitions, and that an explicitly selected replacement workflow root replaces the default tree wholesale.
- No fixture instructs the analyzer to read `ZHK_WORKFLOWS`.
- Bounded engine package tests, relevant workflow tests, the project test and lint targets, and an inert export smoke are recorded with commands and observed output.

## Not covered by this record

Analyzer implementation, host-aware context loading, and editor behavior belong to their own records. This record introduces no capability deltas and no editable surface in the extension repository.