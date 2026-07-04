## ADDED Requirements

### Requirement: Extension manifest and build configuration

The extension SHALL declare an `extension.toml` manifest with `id`, `name`, `description`, `version`, `schema_version`, `authors`, and `repository` fields. A `[grammars.commonlisp]` section SHALL reference the `tree-sitter-grammars/tree-sitter-commonlisp` repository at a pinned commit. A `Cargo.toml` SHALL declare a `cdylib` crate depending on `zed_extension_api`.

#### Scenario: Extension is loadable by Zed

- **WHEN** Zed discovers the extension package
- **THEN** the manifest is parsed successfully and the grammar source is resolved

### Requirement: Common Lisp file recognition

The extension SHALL register a `Common Lisp` language in Zed via `languages/commonlisp/config.toml`. The config MUST set `name = "Common Lisp"`, `grammar = "commonlisp"`, and `path_suffixes` including `lisp`, `lsp`, `cl`, and `asd`.

#### Scenario: Opening a Common Lisp source file

- **WHEN** a user opens a file with extension `.lisp`, `.lsp`, `.cl`, or `.asd`
- **THEN** Zed selects the `Common Lisp` language mode for that buffer

### Requirement: Comment syntax configuration

The language config SHALL define `line_comments` with `"; "` as the first entry (used by toggle-comment) plus `";; "` and `";;; "` continuation prefixes, and `block_comment = ["#|", "|#"]`, so that Zed's comment commands produce idiomatic Common Lisp comments.

#### Scenario: Toggling a line comment

- **WHEN** a user invokes toggle-line-comment on a Common Lisp buffer
- **THEN** the line is prefixed with `; `

#### Scenario: Inserting a block comment

- **WHEN** a user invokes toggle-block-comment on a selection
- **THEN** the selection is wrapped with `#|` and `|#`

### Requirement: Bracket pairs and autoclose

The language config SHALL define bracket pairs for `()`, `""`, and `|` (symbol escape). Parentheses MUST be configured with `close = true` and `newline = true`. The `"` and `|` pairs MUST NOT auto-close inside `comment` and `string` scopes (via `not_in` and an `overrides.scm` defining those scopes). The `autoclose_before` string SHALL include `;:.,=}])>"`.

#### Scenario: Typing an opening parenthesis

- **WHEN** a user types `(` in a Common Lisp buffer
- **THEN** a closing `)` is inserted automatically

### Requirement: Tree-sitter grammar based highlighting

The extension SHALL provide a `highlights.scm` query file that highlights at minimum: comments, string literals, number literals, character literals, keyword symbols (`:foo`), boolean constants (`t`, `nil`), definition keywords (`defun`, `defmacro`, `defclass`, `defvar`, `defparameter`, etc.), control flow forms (`if`, `when`, `unless`, `cond`, `case`), binding forms (`let`, `let*`, `lambda`, `flet`, `labels`), iteration forms (`loop`, `do`, `dotimes`, `dolist`), operators (`setf`, `setq`, `and`, `or`, `not`), built-in functions, lambda-list keywords (`&optional`, `&rest`, `&key`, `&body`), quote/backquote/unquote markers, and parenthesis/bracket punctuation.

#### Scenario: Rendering Common Lisp syntax

- **WHEN** a Common Lisp file is opened in Zed
- **THEN** syntax tokens are highlighted according to the query set with visually distinct categories for keywords, strings, comments, numbers, and built-in forms

### Requirement: Bracket query file

The extension SHALL provide a `brackets.scm` that matches `(` / `)` pairs for structural navigation and rainbow bracket support.

#### Scenario: Navigating matching parentheses

- **WHEN** the cursor is on a `(` or `)`
- **THEN** Zed highlights the matching bracket

### Requirement: Indentation query file

The extension SHALL provide an `indents.scm` that marks list forms as indent containers so that pressing Enter inside a form indents the new line by the configured tab size (default 2 spaces).

#### Scenario: Auto-indenting inside a form

- **WHEN** the user presses Enter after `(defun foo (x)` on a new line inside the body
- **THEN** the cursor is indented by the configured amount relative to the parent form

### Requirement: Outline query file

The extension SHALL provide an `outline.scm` that captures top-level definition forms (`defun`, `defmacro`, `defclass`, `defgeneric`, `defmethod`, `defvar`, `defparameter`, `defconstant`, `defpackage`) as outline entries, enabling the symbol outline panel.

#### Scenario: Viewing the outline panel

- **WHEN** a user opens the outline/symbol panel for a Common Lisp buffer
- **THEN** top-level definitions are listed with their names and kinds

### Requirement: Text objects query file

The extension SHALL provide a `textobjects.scm` that captures list forms and function/macro definitions as text objects for structural selection commands. The query SHALL use the following capture names per current Zed conventions:

- `@function.around` for entire function/macro definitions
- `@function.inside` for function bodies (the body forms of the definition, excluding the header)
- `@class.around` for entire list forms
- `@class.inside` for list form contents

#### Scenario: Selecting a function definition as a text object

- **WHEN** a user invokes "select enclosing text object" inside a `defun` form
- **THEN** the entire s-expression including parentheses is selected via `@function.around`

#### Scenario: Selecting a function body as a text object

- **WHEN** a user invokes "select inside text object" inside a `defun` form
- **THEN** the body forms are selected via `@function.inside`

### Requirement: Comment injection query file

The extension SHALL provide an `injections.scm` that injects the `comment` language into comment nodes, enabling TODO/FIXME/HACK highlighting within comments.

#### Scenario: TODO comment highlighting

- **WHEN** a comment contains `TODO:` or `FIXME:`
- **THEN** those markers are highlighted distinctly from surrounding comment text

### Requirement: Indentation defaults

The language config SHALL set `tab_size = 2` and `hard_tabs = false` to match Common Lisp community conventions.

#### Scenario: Default indentation

- **WHEN** a user opens a Common Lisp file without workspace-specific settings
- **THEN** indentation uses 2 spaces per level

### Requirement: Symbol word characters

The language config SHALL declare `word_characters` and `completion_query_characters` including `-`, `*`, `+`, `!`, `?`, `<`, `>`, and `=` so that Common Lisp symbols behave as single words.

#### Scenario: Selecting a hyphenated symbol

- **WHEN** a user double-clicks on `foo-bar` or `*special*`
- **THEN** the whole symbol is selected as one word
