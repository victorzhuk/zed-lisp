;; Lispico Clojure highlights — go-lispico's Clojure dialect (Lisp-1, bracket
;; literals enabled, shared `#(...)`, `#'`, `~`, `~@` reader forms).

(comment) @comment

(str_lit) @string
(num_lit) @number
(char_lit) @character
(regex_lit) @string.special

(kwd_lit) @keyword

(nil_lit) @constant.builtin
(bool_lit) @constant.builtin

;; Definitions: head and name.
((list_lit
  .
  (sym_lit) @keyword.function
  .
  (sym_lit) @function)
  (#match? @keyword.function "^(defn|defmacro|def)$"))

((list_lit
  .
  (sym_lit) @keyword.function)
  (#eq? @keyword.function "fn"))

;; Binding and iteration forms.
((list_lit
  .
  (sym_lit) @keyword.function)
  (#match? @keyword.function "^(let|let\\*|loop)$"))

;; Control forms.
((list_lit
  .
  (sym_lit) @keyword.control)
  (#match? @keyword.control "^(if|when|cond|and|or|not|do|recur|throw|try|catch|finally|quote|quasiquote)$"))

;; Mutation.
((list_lit
  .
  (sym_lit) @operator)
  (#eq? @operator "set!"))

;; Thread-first / thread-last / conditional-binding bootstrap macros.
((list_lit
  .
  (sym_lit) @keyword.operator)
  (#match? @keyword.operator "^(->|->>|as->|if-let|when-let)$"))

;; Rest marker.
((sym_lit) @operator
  (#eq? @operator "&"))

;; Parameters of fn/defn/defmacro: vector and list parameter lists, named
;; and anonymous fn.
((list_lit
  .
  (sym_lit) @_fn
  .
  (sym_lit)
  .
  [(list_lit (sym_lit) @variable.parameter)
   (vec_lit (sym_lit) @variable.parameter)])
  (#match? @_fn "^(fn|defn|defmacro)$"))

((list_lit
  .
  (sym_lit) @_fn
  .
  [(list_lit (sym_lit) @variable.parameter)
   (vec_lit (sym_lit) @variable.parameter)])
  (#eq? @_fn "fn"))

;; Call-position heads other than the special forms above.
((list_lit
  .
  (sym_lit) @function)
  (#not-match? @function "^(def|defn|defmacro|fn|let|let\\*|loop|if|when|cond|and|or|not|do|recur|throw|try|catch|finally|quote|quasiquote|set!|->|->>|as->|if-let|when-let)$"))

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
"[" @punctuation.bracket
"]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket
