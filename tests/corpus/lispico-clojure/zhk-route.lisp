;; Source: zhk f9ce4a1 workflows/plan/main.lisp lines 1-22 (excerpt).
;; Provenance: a zhk route program evaluated after the ordered prelude.
;; a reviewer judges it, and the plan lands as the appendix of the change's
;; design.md and, where the repository has a board, as its plan record.
;; This policy owns the inputs, the review rounds and their budget, the
;; appendix contract and the completion; the plan's shape is the prelude's.
;; The kernel runs every check itself, so the agents only plan and review.

(def change (zhk/fact "focus"))
(def change-dir (str "openspec/changes/" change))
(def design-path (str change-dir "/design.md"))

;; A submission missing the schema is asked again on the same step; past
;; this many submissions the planning stops.
(def review-rounds 3)
(def plan-submissions 5)
(def max-input-bytes 4194304)

(def planner-blocked "the planner reported blockers")

;; ---- design.md ------------------------------------------------------------------

;; design.md without its appendix section, so writing the appendix changes
