;; Common Lisp Outline/Symbol Panel

(defun
  (defun_header
    keyword: (defun_keyword) @context
    function_name: (_) @name)) @item

((list_lit
  .
  (sym_lit) @context
  .
  [(sym_lit) (kwd_lit)] @name)
  (#match? @context "^(defclass|defvar|defparameter|defconstant|defpackage|defstruct)$")) @item
