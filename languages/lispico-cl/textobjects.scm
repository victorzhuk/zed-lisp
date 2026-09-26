;; Lispico CL text objects.

;; Definitions: the whole form and its body (parameters plus body forms).
((list_lit
  .
  (sym_lit) @_def
  .
  (sym_lit)
  .
  (_)+ @function.inside)
  (#match? @_def "^(defun|defn|defmacro)$"))

((list_lit
  .
  (sym_lit) @_def
  .
  (sym_lit))
  (#match? @_def "^(defun|defn|defmacro|def)$")) @function.around

;; List interiors: every child, without the surrounding delimiters.
(list_lit
  .
  (_)+ @class.inside)

(list_lit) @class.around

;; `#(...)` reader vectors are collections.
(anon_fn_lit
  .
  (_)+ @class.inside)

(anon_fn_lit) @class.around
