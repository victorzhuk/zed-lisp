;; Lispico CL corpus entry for the go-lispico source repository. The
;; goldset fixtures are Clojure-dialect sources; this directory holds the
;; explicitly selected Common Lisp profile sources so both modes have a
;; home without a repository-wide dialect guess.

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

#'greet
#(greet "world")
