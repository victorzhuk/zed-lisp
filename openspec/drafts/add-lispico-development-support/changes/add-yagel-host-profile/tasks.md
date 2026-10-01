# Tasks

Original task identifiers remain stable for parent references. All tasks are pending; existing-service-strict evidence is required.

- [ ] 3.3 Publish source-backed declarations distinguishing root values, pure/staged/live functions, and workflow-only helpers. Verify names, signatures, phases, and unknown-signature behavior without booting a live session.
- [ ] 3.4 (Yagel) Publish portable source-layout fixtures for layer winners, per-file isolation, and unreadable overrides. Verify source-checkout versus installed-runtime context without running rules or applying effects.

## Required evidence

- Exports carry verified names, signatures, and phases; unknown metadata is omitted rather than invented, and no Core boot, session start, macro expansion, or rule execution is observable.
- Schema-2 metadata carries an exact owner revision and bounded source manifest framed exactly as the already-landed zhk host profile frames its declared-source fingerprints; runtime core/stdlib/JSON catalogs are kept separate and pinned to v0.14.0.
- Installed-pack capture reuses production selection semantics, holds the v1 active-set lease through verification and copying, releases it on success and failure, and uses self-contained v2 records; no ambient store discovery and no host boot are observable.
- A snapshot with verified payloads is published only after every payload verifies; a snapshot whose payload digest fails verification is rejected and any earlier valid snapshot is not retained as a fallback.
- Source-profile fixtures prove relative-key precedence, per-file isolation, unreadable winners blocking lower layers, and higher-layer replacement.
- Bounded package tests for the affected CLI, core, rules, and packstore packages, the existing relevant rule checks, and a real inert exporter smoke are recorded with commands and observed output.

## Not covered by this record

Analyzer implementation, host-aware context loading, and editor behavior belong to their own records. The existing `rules check` is v1-only and is not repaired or relied on as a complete oracle here. This record introduces no capability deltas and no editable surface in the extension repository.