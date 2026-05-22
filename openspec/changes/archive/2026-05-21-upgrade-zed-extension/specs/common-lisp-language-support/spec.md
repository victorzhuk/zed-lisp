## CHANGED Requirements

### Requirement: Text objects query file

The extension SHALL provide a `textobjects.scm` that captures list forms and function/macro definitions as text objects for structural selection commands. The query SHALL use the following capture names per current Zed conventions:

- `@function.around` for entire function/macro definitions
- `@function.inside` for function bodies (the lambda list and body within the definition)
- `@class.around` for entire list forms
- `@class.inside` for list form contents

#### Scenario: Selecting a function definition as a text object

- **WHEN** a user invokes "select enclosing text object" inside a `defun` form
- **THEN** the entire s-expression including parentheses is selected via `@function.around`

#### Scenario: Selecting a function body as a text object

- **WHEN** a user invokes "select inside text object" inside a `defun` form
- **THEN** the lambda list and body are selected via `@function.inside`
