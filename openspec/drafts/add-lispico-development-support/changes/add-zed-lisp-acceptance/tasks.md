# Tasks

Original task identifiers remain stable for parent references. All tasks are pending; existing-service-strict evidence is required.

- [ ] 6.1 Smoke-test runtime contexts in actual Zed: both dialects, aliases, completion and signatures, navigation, diagnostics, and valid snippets. Record versions, fixtures, actions, and observed results.
- [ ] 6.2 Smoke-test zhk in Zed: explicit Clojure mode, host completion, prelude navigation, route isolation, and clearing unsaved errors. Do not execute workflows.
- [ ] 6.3 Smoke-test Yagel in Zed: host context, rule isolation, layers, proven phase restrictions, workflow opt-in, and project-local catalog navigation. Do not start sessions or apply rules.
- [ ] 6.4 Re-check plain Common Lisp recognition, highlighting, outline and text objects, and server settings in Zed. Verify missing Lispico tooling does not disrupt structural editing; record unrelated maintenance findings separately.

## Required evidence

- Each session records the exact extension version, server binary version and revision, runtime version and revision, host version and revision, catalog file path and fingerprint, and any installed-pack snapshot path and fingerprint used.
- For 6.1: a real Zed session observes completion and signature help for `zhk/step` in a zhk route through the matching host catalog, and observes the same-dialect / other-dialect-adapter difference in CL contexts where the runtime registers both cells.
- For 6.2: a real Zed session observes a zhk route resolving its ordered prelude helper to the real definition, another route redefining the same name not satisfying a reference inside the selected route, an unsaved prelude edit clearing stale errors, and a wholesale replacement root replacing the default tree.
- For 6.3: a real Zed session observes a Yagel project layer overriding an embedded rule and identifying the embedded file as shadowed, an unreadable winner blocking the lower layer, workflow-only helpers not appearing as global rules, and installed-pack snapshot keys matching their logical rule identity.
- For 6.4: a real Zed session observes that plain Common Lisp recognition, highlighting, outline, text objects, and server settings are unaffected by missing Lispico tooling; missing-server behavior leaves structural editing usable; unrelated maintenance findings are recorded separately and do not block this record.
- Bounded local wrappers are run once for the coherent implementation set; their commands and observed results are recorded alongside the session observations.

## Not covered by this record

Catalog production, host profile export, analyzer parity, and host-aware context loading belong to their own records. The separate shared-server migration in `migrate-to-llsp` is a prerequisite of the editor path; this record waits on it but does not own it. This record introduces no capability deltas.