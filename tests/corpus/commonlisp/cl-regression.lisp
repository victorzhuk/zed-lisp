;; Common Lisp regression fixture: ANSI Common Lisp source exercising the
;; existing commonlisp queries (defun, defvar, defpackage, defstruct, loop,
;; quoting, vectors, block comments, character literals, packages).
;; Provenance: composed for regression coverage of the pre-existing grammar
;; (tree-sitter-grammars/tree-sitter-commonlisp @ 3232350) and queries.

(defpackage #:example
  (:use #:cl)
  (:export #:greet))

(in-package #:example)

(defparameter *greeting* "hello")

(defvar *counter* 0)

(defstruct point
  x
  y)

(defun greet (name)
  "Return a greeting for NAME."
  (format t "hello, ~a!~%" name))

(defun bump ()
  (incf *counter*))

(defun classify (n)
  (cond ((= n 0) :zero)
        ((minusp n) :negative)
        (t :positive)))

(defun accumulate (limit)
  (loop for i from 0 below limit
        sum i))

(defun quoting ()
  (let ((quoted 'a)
        (quasi `(a ,(+ 1 2) ,@(list 3 4)))
        (fn-ref #'greet))
    (values quoted quasi fn-ref)))

(defun vector-literal ()
  #(1 2 3))

#| a block comment
   spanning lines (1 2 3) |#

(char-code #\a)
