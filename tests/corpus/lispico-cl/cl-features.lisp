;; Composed feature fixture for the Lispico CL profile.
;; Provenance: forms verified against go-lispico cl/cl.go (Lisp-2, bracket
;; literals disabled, `#'` function references, `#(...)` reader vectors) and
;; the shapes exercised by cl/cl_test.go: nested-list let bindings, let*
;; sequential bindings, list loop bindings with recur, parenthesized cond
;; pairs, progn, setq, and the CL vocabulary (car, cdr, concat, append).

(defun greet (name)
  (let ((greeting (concat "hello, " name)))
    (setq greeting greeting)))

(defun route (event)
  (cond
    ((= event 'tool-call) :dispatch)
    (:else :ignore)))

(defun count-down (n)
  (loop ((i n))
    (if (< i 0)
        i
        (recur (- i 1)))))

(defun sequential ()
  (let* ((a 1)
         (b (+ a 2)))
    (progn
      (setq b (+ b a))
      b)))

(defun head-and-rest (items)
  (cons (car items) (cdr items)))

(defun guarded (value)
  (if (null value)
      :empty
      value))

(def add #'count-down)

#(count-down 3)

(try
  (greet "мир")
  (catch error (append "failed: " error)))
