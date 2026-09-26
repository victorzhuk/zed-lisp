;; Source: yagel 1f0a5757 rules/defaults/doctor.clj lines 20-37 (excerpt).
;; Provenance: Yagel rule scope, embedded layer, vector bindings, staged/live host calls.

(def roles-snap nil)
(def lineup-chosen nil)
(def models-pin nil)

(sub "roles-registered"
  (fn [ev] (set! roles-snap ev)))
(sub "lineup-chosen"
  (fn [ev] (set! lineup-chosen (get ev :lineup ""))))
(sub "models-registered"
  (fn [m] (set! models-pin m)))

(pub "command-registered" {:name "doctor"
  :description "Report the current Session state: fixed configuration and live budgets."
  :handler "doctor/command"}
  {:retain-key :name})

