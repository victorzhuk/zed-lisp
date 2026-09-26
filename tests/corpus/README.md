# Syntax and query corpus

Bounded fixtures verifying that both shipped Lispico modes and the existing
Common Lisp mode parse and query their real source shapes. The Rust test
harness (`tests/corpus.rs`, `tests/queries.rs`) parses every complete
fixture with its pinned grammar — zero `ERROR` and zero `MISSING` nodes —
and compiles every shipped query file against the same grammars.
`incomplete/` fixtures must recover without hanging; the native analyzer
owns the diagnostic.

## Provenance

Reviewed upstream working trees on 2026-09-26 (change task 1.1):

| Repository | Recorded revision | Notes |
| --- | --- | --- |
| go-lispico | `8b7c299` (moved past reviewed `1d36a9b`/`894a9be` through docs-only openspec commits; reader, dialect, and evaluator sources verified unchanged in the reviewed areas) | Runtime facts re-verified against the working tree: sequential `let`/`let*` (`core/eval.go`), reader forms and dialect flags (`core/reader.go`, `cl/cl.go`, `clojure/clojure.go`), CL test shapes (`cl/cl_test.go`). |
| zhk | `f9ce4a1` | `go.mod` pins go-lispico `v0.14.0`; `workflows/lib/*.lisp` prelude and route `main.lisp` layout unchanged. |
| yagel | `1f0a5757` | `go.mod` pins go-lispico `v0.14.0`; embedded → packs → global → project rule layers and per-rule scopes unchanged. |

Fixture provenance is repeated in each fixture's leading comments. When a
fixture is refreshed from an upstream tree, update its header and this
table together.

## Fixtures

| File | Source | Exercised behavior |
| --- | --- | --- |
| `lispico-clojure/zhk-prelude.lisp` | zhk `f9ce4a1` `workflows/lib/00-core.lisp` excerpt | List parameters, list bindings, maps, vectors, `zhk/*` host calls, flat `cond`, `throw`. |
| `lispico-clojure/zhk-route.lisp` | zhk `f9ce4a1` `workflows/plan/main.lisp` excerpt | Top-level route definitions evaluated after the ordered prelude. |
| `lispico-clojure/yagel-rule.clj` | yagel `1f0a5757` `rules/defaults/doctor.clj` excerpt | Vector bindings, `sub`/`pub`/`command/claim` callbacks, `try`/typed `catch`. |
| `lispico-clojure/goldset-pipeline.lisp` | go-lispico `8b7c299` goldset fixture (verbatim) | Higher-order stdlib calls. |
| `lispico-clojure/goldset-route-decision.lisp` | go-lispico `8b7c299` goldset fixture (verbatim) | Flat `cond`, keyword dispatch. |
| `lispico-clojure/clojure-features.lisp` | composed; verified against `core/reader.go` | Quoting, quasiquote with `~`/`~@`, `#'`, `#(...)`, `@`, list and vector bindings, qualified symbols, sequential sibling visibility, Unicode strings, loop/recur, `try`/`catch`. |
| `lispico-cl/cl-features.lisp` | composed; verified against `cl/cl.go` and `cl/cl_test.go` | `defun`, `progn`, `setq`, `car`/`cdr`/`null`, parenthesized `cond`, nested-list and `let*` bindings, list `loop` bindings, `#'`, `#(...)`, no bracket literals, Unicode. |
| `commonlisp/cl-regression.lisp` | composed for existing-mode coverage | `defpackage`, `defun`, `defvar`/`defparameter`, `defstruct`, `loop`, quoting, vectors, block comments, character literals. |
| `*/incomplete/*.lisp` | composed | Recovery of unclosed forms and strings without hanging. |
