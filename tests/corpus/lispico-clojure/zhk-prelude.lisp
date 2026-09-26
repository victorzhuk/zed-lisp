;; Source: zhk f9ce4a1 workflows/lib/00-core.lisp lines 1-42 (excerpt, whitespace-trimmed).
;; Provenance: Lispico Clojure dialect, stdlib + json + zhk libraries.
;; helpers the routes compose their policy from. It is evaluated before
;; each workflow's own script.

;; ---- evidence -----------------------------------------------------------

;; The latest submission against a step, or nil while none stands.
(defn latest (step)
  (last (:evidence step)))

;; The body of the latest submission, or an empty map.
(defn latest-body (step)
  (or (:body (latest step)) {}))

;; The outcome word of the latest submission: ok, failed, unknown,
;; refused, partial, or nil.
(defn latest-outcome (step)
  (:outcome (latest step)))

(defn settled? (step)
  (= (:status step) "settled"))

(defn accepted? (step)
  (= (:verdict step) "accepted"))

;; ---- kernel commands ----------------------------------------------------

;; The only variables a kernel command inherits, so a run does not depend
;; on whatever else the caller's shell exported.
(def command-env
  ["PATH" "HOME" "USER" "LANG" "LC_ALL" "TERM" "TMPDIR" "XDG_CACHE_HOME" "XDG_CONFIG_HOME"
   "GOPATH" "GOCACHE" "GOMODCACHE" "GOFLAGS" "GOTMPDIR" "CGO_ENABLED"])

;; Runs argv once per run: a later evaluation answers from the record.
(defn run! (id argv opts)
  (zhk/sh id argv (assoc opts :env command-env)))

;; Runs argv on every evaluation and records nothing, for reads whose
;; answer may change between evaluations.
(defn probe (id argv opts)
  (run! id argv (assoc opts :fresh true)))

