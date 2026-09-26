;; Source: go-lispico 8b7c299 internal/goldset/testdata/route-decision.lisp (verbatim fixture).
;; Provenance: Lispico Clojure dialect gold set; flat cond and keyword dispatch.
; Example fixture pending the YAGEL-exported gold set: a rule-shaped
; handler dispatching on a keyword-called event map through flat cond.
(defn route [event]
  (cond
    (= (:type event) :tool-call) :dispatch
    (= (:type event) :message) :reply
    :else :ignore))
(route {:type :message :from "user"})
