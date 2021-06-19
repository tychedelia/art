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
  ;; (core/ES8)
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
  (let [val (miss! (async/alts!! [core/c0] :default (:val state)))
        [gate] (async/alts!! [core/c1] :default {:gate false})
        [r g b] (map-sound val)]
    {:r r :g (if (:gate gate) 0 255) :b b :val val}))


(defn draw-state [{:keys [r g b a]}]
  (q/background r g b)
  (fps))

(q/defsketch debug
  :title "debug"
  :size [1000 400]
  :setup setup
  :update update-state
  :draw draw-state
  :features [:no-bind-output]
  :middleware [m/fun-mode])
