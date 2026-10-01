# lispico-language-support Specification

## Purpose
Provide structural editing for go-lispico dialects in Zed while preserving ordinary Common Lisp and other Clojure projects.

## Requirements

### Requirement: Explicit Lispico language selection

The extension SHALL expose separate `Lispico Clojure` and `Lispico CL` modes. It MUST NOT globally take ownership of `.lisp`, `.lsp`, `.cl`, `.asd`, `.clj`, or `.edn`. Workspace associations or manual selection SHALL select Lispico mode independently of the file suffix.

#### Scenario: zhk workflow uses the Clojure dialect
- **WHEN** the documented zhk workspace associations are enabled and a workflow `.lisp` file opens
- **THEN** Zed selects `Lispico Clojure`, not ordinary Common Lisp

#### Scenario: Existing projects remain unchanged
- **WHEN** no Lispico association is configured
- **THEN** this extension continues selecting Common Lisp for its existing suffixes and does not change other extensions' Clojure or EDN associations

### Requirement: Dialect-correct syntax presentation

Each Lispico mode SHALL highlight strings, numbers, keywords, symbols, comments, definitions, bindings, control forms, and `nil`/`true`/`false` according to its supported source syntax. Quotes, quasiquotes, `~`, and `~@` SHALL remain structurally distinguishable. CL mode SHALL present `#'` as a function reference and `#(...)` as a vector. Highlighting MUST NOT imply that all constructs accepted by a permissive structural grammar are valid Lispico programs.

#### Scenario: CL reader vector
- **WHEN** a valid Lispico CL reader vector is opened
- **THEN** it is highlighted and selected as a collection without an anonymous-function outline entry

#### Scenario: Lispico Clojure source
- **WHEN** a zhk or Yagel source uses maps, vectors, list/vector parameters, qualified symbols, and flat `cond` clauses
- **THEN** those forms retain their boundaries and expected syntax categories without ANSI Common Lisp assumptions

### Requirement: Structural editing and symbol boundaries

Lispico modes SHALL provide matching delimiters, two-space indentation defaults, comment commands, and string/comment scope exclusions for auto-closing. Clojure mode SHALL support `()`, `[]`, and `{}`. CL mode SHALL support its list and reader-vector structure without offering unsupported bracket literals or Common Lisp escaped-symbol/block-comment syntax. Qualified names including `/` and keywords beginning with `:` SHALL remain intact during selection and completion. Parentheses in strings/comments MUST NOT alter structural navigation.

#### Scenario: Qualified host completion
- **WHEN** the user selects or requests completion within `zhk/step` or `command/claim`
- **THEN** the complete qualified name is the symbol range

#### Scenario: String and comment delimiters
- **WHEN** collection delimiters appear inside a string or line comment
- **THEN** bracket matching and auto-closing use the enclosing lexical scope rather than treating them as code delimiters

### Requirement: Definitions and collection text objects

Each mode SHALL show supported function, macro, and value definitions in the outline. Text objects SHALL select entire definitions or bodies and entire collection interiors, including every child. Anonymous functions SHALL NOT masquerade as named top-level definitions.

#### Scenario: List-parameter function
- **WHEN** zhk's `(defn latest (step) ...)` is opened
- **THEN** `latest` appears in the outline and its body can be selected independently of its header

#### Scenario: Multiple collection children
- **WHEN** a collection has several children and the user selects its interior
- **THEN** the selection covers all interior children without the surrounding delimiters

### Requirement: Valid scoped authoring resources

The extension SHALL provide dialect-scoped snippets for supported definitions, bindings, iteration, and error handling, plus portable project configuration examples. Snippets MUST use valid Lispico parameter/rest syntax and MUST NOT introduce full-Clojure-only namespace or docstring forms. Host-specific snippets SHALL require that host's context or explicit project installation.

#### Scenario: Definition snippet expands
- **WHEN** a definition snippet is completed with valid names/body in its intended dialect
- **THEN** the resulting source passes that dialect's syntax/shape validation

#### Scenario: Host isolation in suggestions
- **WHEN** a zhk context is active
- **THEN** automatic host-specific suggestions do not offer Yagel claim declarations

### Requirement: Structural features survive absent semantic tooling

Highlighting, bracket matching, indentation, outline, and text objects SHALL remain usable when the language server or host catalogs are absent.

#### Scenario: Server not installed
- **WHEN** a Lispico source opens without the language server available
- **THEN** structural editing works and the missing semantic dependency is reported separately
