;; Common Lisp Outline/Symbol Panel

((list_lit
  .
  (sym_lit) @context
  .
  (sym_lit) @name)
  (#match? @context "^(defun|defmacro|defgeneric|defmethod|defclass|defvar|defparameter|defconstant|defpackage|defstruct)$")) @item
