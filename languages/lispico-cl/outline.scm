;; Lispico CL outline. defun/defn/defmacro/def are named definitions;
;; `#(...)` reader vectors never appear as items.

((list_lit
  .
  (sym_lit) @context
  .
  (sym_lit) @name)
  (#match? @context "^(defun|defn|defmacro|def)$")) @item
