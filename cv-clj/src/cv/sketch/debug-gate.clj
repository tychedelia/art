(ns cv.sketch.1
  (:require [cv.core :as core]
            [clojure.core.async :as async]
            [quil.core :as q]
            [quil.middleware :as m]))

(defn map-sound [val]
  [(q/map-range val -30000 30000 0 255) 0 (q/map-range val 30000 -30000 0 255) ])

(def !n (atom 0))
(def !m (atom 0))

(defn fps []
  (q/fill 0)
  (q/text-size 32)
  (q/text (str @!m "/" @!n) 20 80)
  (q/text (str (int (q/current-frame-rate))) 20 40))

(defn setup []
  (q/frame-rate 30)
  {:r 0 :g 0 :b 0 :val 0})

(defn miss! [[val chan]]
  (swap! !n inc)
  (if (= chan :default)
    (do
      (swap! !m inc)
      (q/fill 0)
      (q/text-size 32)
      (q/text (str @!m "/" @!n) 20 80)))
  val)

(defn update-state [state]
  (let [val (miss! (async/alts!! [core/c1] :default (:val state)))]
    (if (:gate val) 255 0)))

(defn draw-state [b]
  (q/background b)
  (fps))

(q/defsketch debug-gate
  :title "debug-gate"
  :size [1000 400]
  :setup setup
  :update update-state
  :draw draw-state
  :features [:no-bind-output]
  :middleware [m/fun-mode])
