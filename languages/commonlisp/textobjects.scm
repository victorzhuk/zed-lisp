(defun) @function.outer

(defun
  (defun_header
    (defun_keyword)
    (_)
    (_) @function.inner))

(list_lit) @class.outer

(list_lit
  .
  (sym_lit)
  .
  (_) @class.inner)
