;; Composed feature fixture for the Lispico Clojure dialect.
;; Provenance: composed for this corpus; every reader form below is verified
;; against go-lispico core/reader.go and cl/clojure.go dialect flags
;; (quoting, quasiquote, `~`/`~@`, `#'`, `#(...)`, bracket literals, `;` comments).
;; It exercises list and vector bindings, qualified symbols, flat cond, and
;; Unicode text.

(def config {:name "запуск" :retries 3
             :labels ["α" "β" "γ"]})

(defn note (cfg)
  ;; Unicode in strings and comments: 🚀 "quotes" stay in strings.
  (str "заметка: " (:name cfg) " → done"))

(defn pick (cfg key)
  (get cfg key :else))

(let ((x 1)
      (y (+ x 1))) ; later initializers see earlier siblings under the reviewed runtime
  (str "x=" x " y=" y))

(let [a "α" b "β"]
  [a b])

(def sum-reducer
  (fn [acc n] (+ acc n)))

(def add
  #'sum-reducer)

#(:a :b :c)

(def wrapped
  `(outer ~(first [1 2]) ~@(rest [1 2]) @config))

(def qualified-call
  (example/qualified 1 2 3))

(defn flat-cond (event)
  (cond
    (= (:type event) :tool-call) :dispatch
    (= (:type event) :message) :reply
    :else :ignore))

(defn loop-sum (limit)
  (loop ((i 0)
         (acc 0))
    (if (>= i limit)
        acc
        (recur (+ i 1) (+ acc i)))))

(try
  (note config)
  (catch error (str "failed: " error)))
