(comment) @comment
(block_comment) @comment

(str_lit) @string
(format_specifier) @string.escape

(num_lit) @number
(complex_num_lit) @number

(char_lit) @character

(kwd_lit) @keyword

(nil_lit) @constant.builtin
((sym_lit) @constant.builtin
  (#eq? @constant.builtin "t"))

((list_lit
  .
  (sym_lit) @function)
  (#not-match? @function "^(t|defclass|defvar|defparameter|defconstant|defpackage|defstruct|if|when|unless|cond|case|typecase|etypecase|ecase|progn|prog1|prog2|unwind-protect|handler-case|handler-bind|restart-case|block|return-from|catch|throw|tagbody|go|let|let\*|flet|labels|macrolet|symbol-macrolet|multiple-value-bind|destructuring-bind|do|dotimes|dolist|setf|setq|and|or|not)$"))

(defun_keyword) @keyword.function

(defun_header
  function_name: (sym_lit) @function)

((list_lit
  .
  (sym_lit) @keyword.function)
  (#match? @keyword.function "^(defclass|defvar|defparameter|defconstant|defpackage|defstruct)$"))

((list_lit
  .
  (sym_lit) @keyword.control)
  (#match? @keyword.control "^(if|when|unless|cond|case|typecase|etypecase|ecase|progn|prog1|prog2|unwind-protect|handler-case|handler-bind|restart-case|block|return-from|catch|throw|tagbody|go)$"))

((list_lit
  .
  (sym_lit) @keyword.function)
  (#match? @keyword.function "^(let|let\*|flet|labels|macrolet|symbol-macrolet|multiple-value-bind|destructuring-bind)$"))

(loop_macro "loop" @keyword.control)

((list_lit
  .
  (sym_lit) @keyword.control)
  (#match? @keyword.control "^(do|dotimes|dolist)$"))

((list_lit
  .
  (sym_lit) @operator)
  (#match? @operator "^(setf|setq|and|or|not)$"))

((sym_lit) @keyword
  (#match? @keyword "^&(optional|rest|key|body|allow-other-keys)$"))

(package_lit) @type

(path_lit) @string.special
(fancy_literal) @string

(var_quoting_lit) @function

(include_reader_macro marker: _ @keyword.control)

(self_referential_reader_macro) @constant.builtin

"'" @punctuation.special
"`" @punctuation.special
"," @punctuation.special
",@" @punctuation.special

("#'" @punctuation.special)

("(" @punctuation.bracket)
(")" @punctuation.bracket)
