;; Incomplete-source fixture (composed): the parser must recover without
;; hanging; the static analyzer owns the diagnostic.
(defun later (x)
  (progn (+ 1
