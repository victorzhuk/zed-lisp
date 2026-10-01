## MODIFIED Requirements

### Requirement: Common Lisp file recognition

The extension SHALL register a `Common Lisp` language in Zed via `languages/commonlisp/config.toml`. The config MUST set `name = "Common Lisp"`, `grammar = "commonlisp"`, and `path_suffixes` including `lisp`, `lsp`, `cl`, and `asd`. These SHALL remain the default associations when no workspace or manual language override applies. Explicit Lispico associations SHALL select the requested dialect without changing these global defaults.

#### Scenario: Opening a Common Lisp source file

- **WHEN** a user opens a file with extension `.lisp`, `.lsp`, `.cl`, or `.asd` without an explicit language override
- **THEN** Zed selects the `Common Lisp` language mode for that buffer

#### Scenario: Explicit Lispico workflow association

- **WHEN** a workspace explicitly maps a zhk `.lisp` workflow to `Lispico Clojure`
- **THEN** the workflow uses that mode while ordinary Common Lisp files outside the override retain their existing language behavior
