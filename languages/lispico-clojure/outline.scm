;; Lispico Clojure outline. Only named definitions are items; anonymous
;; functions are not top-level definitions.

((list_lit
  .
  (sym_lit) @context
  .
  (sym_lit) @name)
  (#match? @context "^(defn|defmacro|def)$")) @item
