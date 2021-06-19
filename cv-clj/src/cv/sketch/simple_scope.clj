(ns cv.sketch.simple-scope
  (:require
   [quil.middleware :as m]
   [quil.core :as q]
   [taoensso.carmine :as car :refer (wcar)]))

(def con {:pool {} :spec {:host "127.0.0.1" :port 6379}})

(defmacro wcar* [& body] `(car/wcar con ~@body))

(defn setup [] 0)


(defn parse-number
  "Reads a number from a string. Returns nil if not a number."
  [s]
  (if (re-find #"^-?\d+\.?\d*$" s)
    (read-string s)))

(defn update-state [val]
  (parse-number (wcar* (car/get "1"))))

(defn draw-state [val]
  (println val)
  (q/background 0)
  (q/ellipse 500 200 val val))

(q/defsketch scope
  :title "scope"
  :size [1000 400]
  :setup setup
  :update update-state
  :draw draw-state
  :features [:no-bind-output]
  :middleware [m/fun-mode])


