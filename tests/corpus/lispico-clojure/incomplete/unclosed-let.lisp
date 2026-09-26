;; Incomplete-source fixture (composed): the parser must recover without
;; hanging; the static analyzer owns the diagnostic.
(defn later (x)
  (let [x 1
