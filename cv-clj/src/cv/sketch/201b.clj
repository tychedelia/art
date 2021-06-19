(ns cv.sketch.201b
  (:require
   [cv.core :as core]
   [clojure.core.async :as async]
   [quil.core :as q]
   [quil.middleware :as m]))

(defn setup []
  (q/frame-rate 30)
  0)

(defn update-state [state]
  (q/map-range (first (async/alts!! [core/c0] :default (:val state)))))

(defn draw-state [state]
  (q/stroke-cap :square)
  (q/smooth)
  (q/no-fill)
  (q/background 255)
  (q/translate (/ (q/width) 2) (/ (q/height) 2))
  (q/stroke-weight (q/abs (/ (q/mouse-y) 20)))
  (let [circle-resolution (q/map-range state 0 (q/height) 2 80)
        radius (+ (- state (/ (q/width) 2)) 0.5)
        angle  (/ q/TWO-PI circle-resolution)]
    (q/begin-shape)
    (doseq [i (range circle-resolution)
            :let [x (* (q/cos (* angle i)) radius)
                  y (* (q/sin (* angle i)) radius)]]
      (q/line 0 0 x y))
    (q/end-shape)))

(q/defsketch gen-art
  :title "Hello, shape"
  :size [600 600]
  :setup setup
  :update update-state
  :draw draw-state
  :features [:no-bind-output]
  :middleware [m/fun-mode])




