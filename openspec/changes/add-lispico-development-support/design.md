# Design

## Context

See [review](review.md) for source evidence. The extension is a small Rust/WASM adapter with Common Lisp queries. Both hosts use Lispico Clojure, but their loading and binding rules differ. No existing Lispico command supplies a complete static editor service.

Design depth: Standard. Verification mode proposed for approval: **existing-service-strict**. This is an additive editor feature with upstream tooling prerequisites, not an evaluator rewrite.

## Goals and boundaries

Provide trustworthy completion, signatures, documentation, navigation, and diagnostics using the selected runtime and host context. Preserve ordinary Common Lisp support and keep host behavior out of the WASM adapter.

Initial scope excludes formatting, rename, debugger/REPL execution, arbitrary Go-string injection, full Clojure/JVM compatibility, ASDF/Quicklisp support for Lispico, arbitrary macro expansion, host rule execution, and automatic dependency fetching. Server acquisition is no longer excluded: the shared `llsp` server publishes verified release archives, and its checksum-verified download, cache integrity, and offline reuse are specified and gated by [migrate-to-llsp](../migrate-to-llsp/proposal.md). `.edn` pack files remain data; executable-source contexts exclude them. Pack-manifest semantic validation remains with Yagel's existing pack tooling.

## Decisions

### 1. Separate language modes, explicit associations

Add `Lispico Clojure` and `Lispico CL`, served by the shared `llsp` language server, which also serves ordinary `Common Lisp` under one registration. Map their LSP language IDs explicitly. Neither mode claims common suffixes globally. Users select a mode or add workspace `file_types` entries; ordinary `Common Lisp` keeps its existing defaults. Do not infer dialect from `.lisp`, a slash-qualified symbol, or the presence of a Go module.

Portable Zed examples will associate zhk's `**/workflows/**/*.lisp` with `Lispico Clojure`, and Yagel's `**/rules/**/*.clj` and `**/packs/**/*.clj` with that mode. go-lispico's mixed test corpus needs narrowly selected fixture paths and explicit contexts, not a repository-wide dialect guess. Examples must be smoke-tested against Zed's matching behavior.

Use the Clojure Tree-sitter grammar as the first candidate for both new modes, pinned only after a real corpus/query test. Write separate queries rather than copying Clojure semantic assumptions: CL `#(...)` is a vector, not an anonymous function. A permissive structural parse does not prove runtime validity. The static analyzer owns dialect errors. If corpus coverage needs a grammar fork or different distribution, stop and revise the design before adding that maintenance obligation.

### 2. Analysis model, with the server role supplied upstream

Use the published `llsp` server for the language-server role across all three modes; it already implements the `common-lisp`, `lispico-clojure`, and `lispico-cl` dialects and is not a go-lispico deliverable. The batch-checker role is retargeted to llsp's `check` subcommand where its semantics fit, and remains a separately approved upstream prerequisite for the host-aware analysis llsp does not yet provide — the catalog, host-profile, layer, and phase model below is not supplied by any current server. Server registration, language identifiers, resolution, cache, and settings forwarding are owned by the `llsp-language-server-integration` capability in [migrate-to-llsp](../migrate-to-llsp/proposal.md). The editor and batch paths must draw on the same analysis implementation, whichever repository provides it; the extension only registers languages, resolves the binary, and forwards settings.

Create a tooling syntax tree retaining complete source spans and comments, with recovery for incomplete buffers. Keep runtime values and evaluator hot paths unchanged. Compare complete-file syntax with the selected dialect's reader using bounded fixtures. Do not equate successful compilation with name correctness or unsupported bytecode compilation with invalid Lisp.

The resolver models Lisp-1/Lisp-2 cells, local bindings, parameters/rest arguments, `let`/`let*`, `loop`/`recur`, quoted data, quasiquote/unquote depth, `try`/`catch`, definitions, and recognized bootstrap macros. Both `let` and `let*` bind sequentially in the reviewed runtime: later initializers see earlier sibling bindings (`core/eval.go:1308-1309,1914-1953`, `core/eval_test.go:259-270`). Use version-matched behavior rather than importing ANSI CL rules. Vocabulary aliases retain actual runtime exposure; CL adapter signatures override corresponding kernel signatures. Analyze known macro forms structurally. For arbitrary macros, state uncertainty and avoid definitive findings about expansion-dependent bindings.

References cover statically resolved references only. No exhaustive promise for computed names, runtime `eval`, or custom macro expansion. Source definitions lead to real Lisp files; Go-hosted declarations lead to verified Go source when available, otherwise a readable local declaration catalog.

### 3. One explicit project file

Propose `.lispico.json` at each worktree root. A server-specific `project_file` initialization option can select a different file, resolved relative to that worktree. No upward scan into another project. Configuration is declarative JSON; all relative paths use its containing directory. A checked-in JSON schema documents and validates it.

Approved version-2 structure, delivered by [revise-lispico-shared-contracts](../archive/2026-10-01-revise-lispico-shared-contracts/proposal.md):

- `schema_version`: supported integer version.
- `runtime_version`: expected go-lispico version or recorded source revision.
- `catalogs`: records containing a local `path`, `owner`, and independently configured `expected_version` or `expected_revision`; `expected_fingerprint` additionally pins a development snapshot. An optional `source_root` enables checking the catalog's declared source-file fingerprints against a checkout. Conflicting identity fields are invalid.
- `contexts`: entries with a unique `name`, `files` globs, `exclude` globs, `dialect` (`cl` or `clojure`), `profile` (`runtime`, `zhk`, `yagel-rule`, or `yagel-workflow`), `libraries`, and source-loading configuration.
- `libraries`: selected catalog library IDs; locating a library on disk does not enable it.
- Source-loading configuration: explicit `source_roots` for indexing, ordered `prelude` paths/globs for sequential programs, and ordered `layers` for Yagel overlays. A layer records its kind and root; a packs layer alternatively selects a pinned inert snapshot under the [approved shared contracts](contracts.md). Relative logical rule identity determines shadowing. Profile validation rejects fields whose semantics do not apply.

File glob semantics: `/` separator, `*` within a segment, `**` across segments; excludes win. Exactly one context must match an executable source file. Overlap is a configuration error, not a first-match priority rule. Explicit paths are normalized; symlink traversal is disabled. External source roots/catalogs require explicit paths, never ambient home-directory scanning. Reject missing required roots/catalogs and bound all scans. Unmatched files retain structural editing and syntax diagnostics, with a visible missing-context message and no guessed host globals.

Language ID and context dialect must agree. Invalid edits to configuration immediately mark semantic context unavailable and clear results derived from it; do not silently continue using an older valid host profile. Valid reloads rebuild only affected contexts. Independent worktrees never share mutable document or symbol state.

### 4. Match host visibility, including order and phase

**Runtime:** a source root permits indexing, not implicit imports. Each file is independent unless an explicit ordered loading context declares otherwise. Do not invent Lisp `require`/`load` semantics from directory layout. Custom restricted runtime dialects are out of scope until represented by an explicit verified profile.

**zhk:** expand `lib/*.lisp` in lexical filename order; append one route's `main.lisp`. Each route is a separate sequential program. Preserve binding origins and redefinition order across prelude and route; model top-level initializer visibility at its source position and function-body references against the appropriate program environment. A library edit invalidates dependent routes. A selected external workflow tree replaces the default tree, matching `ZHK_WORKFLOWS`; the server does not read that environment variable implicitly. Auxiliary route files do not become executable sources merely because the runtime hashes them.

A prelude file matches one source context but contributes to multiple route analysis snapshots. Requests made inside that shared file are route-independent by default: resolve lexical bindings and invariant prelude references, and mark route-dependent references as uncertain. If routes redefine a referenced name differently, do not pick the first route or combine their globals. Provide no unique definition target or definitive route-dependent diagnostic until analyzed through a specific route. Known call/order information can refine a route snapshot; unknown dynamic invocation order cannot justify a guessed target.

**Yagel:** select effective rule files by relative path using embedded → packs → global → project precedence. Source-repository profiles explicitly map `rules/defaults` to embedded sources and `packs` to pack sources; they do not read an installed binary's embedded snapshot. Overlay selection governs which file is active, not a shared Lisp namespace. Every rule has a separate scope. An unreadable or invalid winning file must not expose a lower layer silently.

Catalogs distinguish root values, pure helpers, staging declarations, and runtime effects. Known declaration/callback shapes provide phase boundaries; direct forbidden top-level calls receive diagnostics. Unknown higher-order or macro-controlled execution remains uncertain. Dynamic workflow helpers appear only in explicit `yagel-workflow` contexts. Opening a shadowed file permits local analysis but identifies it as inactive in the selected layer set.

### 5. Inert declarations, real libraries

Use JSON declaration catalogs, not executable fake `defn` libraries. The analyzer uses original Lisp source for library functions/macros and catalogs for Go-defined symbols. Catalogs contain schema version, owner/module, exact source version or revision, supported dialect/profile, library ID, and entries with:

- exact symbol name, kind, and namespace cell;
- verified argument count/range, rest behavior, and parameter labels when known;
- documentation, verified value/result information when known, and phase availability;
- source path/range when available, with a catalog-location fallback.

Unknown signature/type information remains absent. Never replace unknown signatures with permissive invented varargs. Missing or mismatched catalogs produce one actionable context diagnostic; suppress cascading unknown-host errors while retaining independent syntax/local-scope results. Do not silently select a newer catalog.

go-lispico owns core/dialect/plugin declarations. zhk and Yagel own their host declarations. Initial snapshots can be maintained from source-backed facts, with deterministic parity tests against authoritative names, signatures, aliases, and phase tables. zhk's central registry can directly drive its metadata. Yagel requires explicit verified signature enrichment; its names-only surface cannot supply it. No hot-path `GoFunc` layout change, general reflection system, or plugin package manager is needed.

The [approved shared contracts](contracts.md) define paired source manifests/fingerprints, exact digest framing, bounded and race-safe verification, both-cell catalog identity, and the separate installed-pack snapshot. They supersede unspecified fingerprint details and add explicit static installed-pack capture without changing source-repository profiles. Each producer must use the actual pinned runtime; both hosts currently depend on v0.14.0 rather than the sibling runtime checkout's HEAD.

Snapshot provenance includes source revision/content fingerprint, so dirty development trees are not falsely labeled as an unchanged release. Compare catalog identity with the independently selected expected owner/version/revision/fingerprint in project configuration. When `source_root` is provided, verify the declared source-file fingerprint set against that tree and report changed inputs. Without a host checkout, verify the explicitly pinned catalog identity and label provenance as catalog-only; do not claim to have verified an installed host binary. Bundle matched core declarations with the server; select host catalogs explicitly. Package readable catalogs and source references so navigation works without a sibling checkout. Updating snapshots is an explicit maintainer action, never an editor-side runtime evaluation.

### 6. Server and diagnostic behavior

Resolution, language identifiers, cache handling, and settings forwarding are owned by the `llsp-language-server-integration` capability in [migrate-to-llsp](../migrate-to-llsp/proposal.md): one shared server entry for all three modes, configured binary → `PATH` → checksum-verified release download, and an actionable error otherwise. No Roswell build, no source build, and no installer script. This change keeps only the buffer-facing contract below. The project-file initialization option remains a reviewed migration gate: do not assume llsp reads `.lispico.json`, and do not document the option before it is verified against the real configuration surface or a rejected option is reported to the user. A missing or failing server leaves Tree-sitter features available.

LSP baseline: initialize, shutdown/exit, document sync, completion, hover, signature help, definition, references, document/workspace symbols, and published diagnostics. Use document versions, cancellation, debouncing, bounded indexing, and negotiated position encoding (UTF-16 fallback). Unsaved buffers supersede disk. Clear diagnostics on fixes, deletion, closure where appropriate, and context changes. Do not advertise unsupported capabilities.

The batch checker uses the same config, catalogs, resolver, and diagnostic codes. Upstream `llsp` ships a `check` subcommand; any option that addresses the project file or selects machine output must be read from that command's real interface and specified in the owning upstream change before being documented here. JSON records include path/URI, range, severity, code, and message. Exit 0 means no error diagnostics, 1 means source errors, 2 means configuration/tool failure. LSP and CLI results must match for identical saved snapshots. Note that llsp's available lint set is narrower than the list below. That is the current state of the gap between this contract and the available upstream server, and it blocks host-context parity; it is not an approved or accepted reduction, and nothing here authorizes shipping this capability with the missing checks treated as satisfied.

Initial checks: malformed syntax, dialect-disabled reader forms/special forms, malformed bindings/parameters, statically unresolved names, known arity errors, missing libraries, context/catalog failures, and provable host phase violations. Report only proven facts. Keep runtime-dependent value/type/effect claims out of definitive lint results.

### 7. Configuration and authoring resources

Ship documented context templates for the three repositories, with relative paths and a schema. zhk/Yagel templates select only stdlib, JSON, and their own host catalog. They must not enable IO/process/network Lispico plugins merely because those plugins exist in the runtime repository.

Ship language-scoped snippets for supported definitions, bindings, loops, and error handling, with valid list/vector conventions and `&` rest syntax. Host-specific forms belong in host context-aware completion or opt-in project snippet examples; Yagel declarations must not appear as defaults in zhk. Do not offer unsupported Clojure namespace/docstring conventions. Ship no task that invokes a workflow as a lint operation.

## Alternatives considered

- **Retarget sextant:** its Common Lisp semantic model does not describe either host or Lispico's exact CL profile, and it is superseded for all three modes by the shared `llsp` server.
- **Use generic Clojure LSP/lint as authority:** reader overlap does not establish Lispico binding, alias, macro, or phase semantics. A second independent analyzer would need reconciliation and still lack host load scopes.
- **Put analysis in the WASM extension:** duplicates runtime language knowledge, ties reusable checking to one editor, and still needs metadata.
- **Execute the host to discover symbols:** can run effects and cannot represent incomplete unsaved source safely. Controlled parity tests in owning repositories are separate from indexing.

## Delivery and verification

1. Approve scope, upstream ownership, existing-service-strict depth, and seams: syntax, catalog, context loader, LSP transport, Zed adapter.
2. Land bounded corpus/query gates and matched declaration/config schemas. Verify candidate grammar against both profiles before registering it.
3. In separately approved upstream changes, land the host declarations and catalogs, then the host-aware analysis with bounded tests. The language-server role is already supplied by the published `llsp` server, subject to the gates in [migrate-to-llsp](../migrate-to-llsp/proposal.md). Host declarations and source-root profiles must be available before claiming host support.
4. Add Zed modes, queries/snippets, shared server launch, and tested configuration examples. Server registration, identifiers, resolution, cache, and settings forwarding are delivered by [migrate-to-llsp](../migrate-to-llsp/proposal.md). Preserve extension ID and existing CL resources.
5. Exercise all three project profiles and a plain Common Lisp project in Zed. Only then claim complete support and release.

Rust wrapper tests need a wall timeout and capped build/test workers; propose five minutes, four build jobs, four test threads. Upstream Go wrappers must bound both total command duration and test/package parallelism. Query validation and protocol tests need wall limits too. Add limits once to wrappers/CI rather than relying on repeated ad hoc invocations.

Required evidence: syntax/query corpus, catalog parity, host visibility/phase fixtures, unsaved edits and Unicode ranges, cancellation and limit handling, zero execution during analysis, adapter dispatch/config tests, packaged-resource checks, and Zed smoke results. Every new behavior lands with its relevant tests and docs; a successful WASM build alone is insufficient.

## Risks and rollback

- Grammar mismatch → corpus gate before registration; revise scope if a fork is required.
- Runtime/host drift → versioned catalogs and parity tests; visible degraded state on mismatch.
- Macro/dynamic uncertainty → conservative diagnostics with stated limits.
- Large rule sets → bounded scans, cancellation, and source-version isolation.
- Cross-repository delivery → explicit upstream dependency tasks; no placeholder server advertised as working.

Rollback removes Lispico workspace associations and disables its server; Common Lisp behavior remains available. No source migration or host execution is required. Proposed interfaces may be refined through reviewed upstream changes, but changes to visibility, safety, public configuration, or verification floor require revisiting this proposal.

## Ownership split and verified server prerequisite

The six dependency records linked from the proposal own the remaining implementation and acceptance tasks. The requirements in this change remain authoritative; each dependency preserves its original task identifiers so existing references resolve through the task-ownership map. Parent tasks 3.1 and 6.5 remain explicit approval and readiness gates. Dependencies on the parent mean completed corpus/schema resources and upstream approval, not parent completion; final integration depends on the children, avoiding a completion cycle.

The llsp v0.2.0 settings-reload defect is historical. The checksum-verified official v0.2.1 binary at `436bc84` passed 32/32 protocol checks: client-selected Lispico CL, Lispico Clojure, and ordinary Common Lisp retained diagnostic and hover dialect behavior after plain and wrapped settings updates; an incremental unsaved edit cleared CL reader errors; shutdown exited successfully. The same driver failed 11 checks against the official v0.2.0 release. This closes only the upstream language-ID retention prerequisite, not host contexts, catalogs, phase rules, actual Zed acceptance, or the verified-download feasibility gate.

Each ownership record is planning-only here and must link separately approved implementation in its owning repository before its tasks can complete. No duplicate capability deltas are introduced. Archive this parent only when its children, shared-server migration, and final acceptance have completed; preserve its completed task evidence throughout.

## Approved contract migration prerequisite

[revise-lispico-shared-contracts](../archive/2026-10-01-revise-lispico-shared-contracts/proposal.md) completed the bounded local schema/resource prerequisite added by the approved shared-contract decision. The six ownership records remain responsible for their original tasks. Their reviewed execution sequences consume [contracts.md](contracts.md); none authorizes upstream code changes or replaces an executable, baseline-reviewed implementation appendix in the owning repository.
