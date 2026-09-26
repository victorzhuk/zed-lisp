;; Lispico CL highlights — go-lispico's Common Lisp profile (Lisp-2, bracket
;; literals disabled, `#'` function references, `#(...)` reader vectors).

(comment) @comment

(str_lit) @string
(num_lit) @number
(char_lit) @character
(regex_lit) @string.special

(kwd_lit) @keyword

(nil_lit) @constant.builtin
(bool_lit) @constant.builtin

;; Definitions: defun is the CL alias for the kernel defn form.
((list_lit
  .
  (sym_lit) @keyword.function
  .
  (sym_lit) @function)
  (#match? @keyword.function "^(defun|defn|defmacro|def)$"))

;; Binding and iteration forms.
((list_lit
  .
  (sym_lit) @keyword.function)
  (#match? @keyword.function "^(let|let\\*|loop)$"))

;; Control forms.
((list_lit
  .
  (sym_lit) @keyword.control)
  (#match? @keyword.control "^(if|when|cond|and|or|not|do|progn|recur|throw|try|catch|finally|quote|quasiquote|function|funcall)$"))

;; Mutation: setq is the CL rename of set!.
((list_lit
  .
  (sym_lit) @operator)
  (#match? @operator "^(set!|setq)$"))

;; Rest marker.
((sym_lit) @operator
  (#eq? @operator "&"))

;; Parameters of defun/defn/defmacro: list parameter lists (the Lispico
;; convention) and vector parameter lists.
((list_lit
  .
  (sym_lit) @_def
  .
  (sym_lit)
  .
  [(list_lit (sym_lit) @variable.parameter)
   (vec_lit (sym_lit) @variable.parameter)])
  (#match? @_def "^(defun|defn|defmacro)$"))

;; Function references: `#'name` presents the referenced symbol as a function.
(var_quoting_lit
  (sym_lit) @function)

;; Call-position heads other than the special forms above.
((list_lit
  .
  (sym_lit) @function)
  (#not-match? @function "^(def|defn|defun|defmacro|let|let\\*|loop|if|when|cond|and|or|not|do|progn|recur|throw|try|catch|finally|quote|quasiquote|function|funcall|set!|setq)$"))

;; Qualified symbols: namespace and separator stay distinguishable from the name.
(sym_lit
  namespace: (sym_ns) @type
  "/"
  name: (sym_name))

;; Reader structure.
"'" @punctuation.special
"`" @punctuation.special
"~" @punctuation.special
"~@" @punctuation.special
"@" @punctuation.special
"#'" @punctuation.special
"#" @punctuation.special

"(" @punctuation.bracket
")" @punctuation.bracket
