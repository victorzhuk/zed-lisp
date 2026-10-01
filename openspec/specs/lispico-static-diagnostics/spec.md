# lispico-static-diagnostics Specification

## Purpose
Detect provable Lispico source and context errors consistently in Zed and batch checks without executing project code.

## Requirements

### Requirement: Analysis has no runtime effects

Indexing and checking SHALL parse source and declarations without evaluating project forms, expanding arbitrary macros, initializing project plugins, starting a host session, or invoking shell/network/file-write primitives. Discovery MUST NOT run zhk workflows, Yagel rules, or the Lispico REPL. Reading explicitly configured source/catalog files is permitted.

#### Scenario: Effectful top-level source
- **WHEN** a buffer contains a top-level shell, filesystem-write, or network call
- **THEN** analysis produces relevant static results without performing the call

#### Scenario: Effectful macro body
- **WHEN** a macro body contains a host effect
- **THEN** indexing its definition or invocation does not execute the body

### Requirement: Dialect and form validation

The analyzer SHALL diagnose malformed reader syntax, unsupported dialect reader forms, malformed known bindings/parameters, and invalid known special-form shapes. It SHALL accept actual Lispico conventions, including list parameters, supported vector parameters, `&` rest arguments, applicable list/vector bindings, and the selected `cond` convention. ANSI CL or full-Clojure-only syntax MUST NOT be assumed valid from file suffix or structural highlighting.

#### Scenario: Bracket literal in CL profile
- **WHEN** a Lispico CL context contains a bracket literal disabled by its reader
- **THEN** the analyzer reports a ranged dialect error

#### Scenario: zhk list binding
- **WHEN** a zhk source uses a valid nested-list `let` binding
- **THEN** the analyzer accepts it without requiring full Clojure's vector-only convention

### Requirement: Scope and known-call diagnostics

The analyzer SHALL diagnose statically unresolved references and verified arity errors using actual dialect cells, lexical scope, loading context, and selected catalogs. It SHALL follow the selected runtime's binding semantics, including sequential binding for both `let` and `let*` in the reviewed runtime, track parameters/rest bindings and mutation, and handle quote/quasiquote/unquote depth. Quoted data and identifiers introduced by unsupported macro expansion MUST NOT receive speculative undefined-name errors. A known symbol with unknown signature MUST NOT receive a guessed arity error.

#### Scenario: Misspelled host symbol
- **WHEN** a source references an absent host symbol in an otherwise valid complete context
- **THEN** a diagnostic identifies the exact reference range

#### Scenario: Proven host arity error
- **WHEN** `zhk/step` is called with one argument in a matching zhk context
- **THEN** a known-arity diagnostic reports the call against its verified two-argument declaration

#### Scenario: Quoted symbol data
- **WHEN** an otherwise unknown name occurs as quoted data
- **THEN** the analyzer does not report it as an unresolved evaluated variable

#### Scenario: Later initializer uses an earlier sibling
- **WHEN** a `let` or `let*` initializer refers to an earlier binding in the same form under the reviewed runtime profile
- **THEN** it resolves to that sibling binding instead of an outer binding or an undefined-name error

#### Scenario: Custom macro binding uncertainty
- **WHEN** correctness of a reference depends on expanding an arbitrary macro
- **THEN** the analyzer records the limitation without evaluating that macro or issuing a definitive expansion-dependent error

### Requirement: Host availability diagnostics

Diagnostics SHALL respect enabled libraries, host profiles, file/route isolation, and proven execution-phase restrictions. Invalid catalogs or contexts SHALL produce actionable context failures while suppressing cascaded errors caused solely by unavailable metadata. Independent syntax and local-scope errors SHALL remain visible.

#### Scenario: Missing host metadata
- **WHEN** a selected host catalog is missing or incompatible
- **THEN** the user receives a context diagnostic rather than one unknown-host error per call

#### Scenario: Forbidden staging call
- **WHEN** a call is provably executed at staging time and its verified declaration forbids that phase
- **THEN** the diagnostic names the phase restriction at the call without executing it

### Requirement: Ranged diagnostics and replacement

Every source diagnostic SHALL contain a stable code, severity, message, source URI/path, and accurate range. Related source locations SHALL be included when needed to explain conflicting bindings or context. After a source/configuration fix, obsolete diagnostics MUST be removed. Partial analysis or missing context MUST be distinguishable from a complete successful check.

#### Scenario: Error is fixed in an unsaved buffer
- **WHEN** a newer buffer version fixes an error
- **THEN** the previous diagnostic disappears without requiring file save or server restart

### Requirement: Shared editor and batch results

A batch checker command SHALL use the same analysis semantics, project configuration, and catalog versions as the language server. The upstream `llsp` server's `check` subcommand is the batch entry point where its semantics fit; the host-aware analysis it does not yet provide is an open upstream prerequisite and an unmet part of this capability, not a reduced scope that satisfies it. The checker SHALL provide human-readable and machine-readable results. Machine-readable records SHALL retain diagnostic codes, severities, paths, and ranges. Exit status SHALL be 0 for no error diagnostics, 1 for source errors, and 2 for configuration/tool failure. Batch execution SHALL be bounded and SHALL NOT modify source.

#### Scenario: Editor and CLI parity
- **WHEN** the checker and server analyze the same saved source snapshot and context
- **THEN** they report equivalent diagnostic codes, severities, and ranges

#### Scenario: Checker coverage narrower than the editor contract
- **WHEN** the available checker does not implement one of the diagnostics this capability requires
- **THEN** that gap is reported as unmet work rather than presented as parity with the editor, and this capability is not treated as satisfied

#### Scenario: Existing host checks remain available
- **WHEN** a Yagel user also runs the existing `rules check` workflow
- **THEN** that host check remains separate and is not presented as unsaved-buffer or ranged LSP analysis
