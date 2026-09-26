;; Lispico Clojure text objects.

;; Definitions: the whole form and its body (parameters plus body forms).
((list_lit
  .
  (sym_lit) @_def
  .
  (sym_lit)
  .
  (_)+ @function.inside)
  (#match? @_def "^(defn|defmacro)$"))

((list_lit
  .
  (sym_lit) @_def
  .
  (sym_lit))
  (#match? @_def "^(defn|defmacro|def)$")) @function.around

;; Collection interiors: every child, without the surrounding delimiters.
(list_lit
  .
  (_)+ @class.inside)

(list_lit) @class.around

(vec_lit
  .
  (_)+ @class.inside)

(vec_lit) @class.around

(map_lit
  .
  (_)+ @class.inside)

(map_lit) @class.around

;; `#(...)` reader vectors are collections.
(anon_fn_lit
  .
  (_)+ @class.inside)

(anon_fn_lit) @class.around
