(ns cv.sketch.lissa
  (:require
   [quil.core :as q]
   [quil.middleware :as m]))

;; CONSTANTS
(def state {:point-count 200
            :freq-x 4
            :freq-y 7
            :mod-freq-x 3
            :mod-freq-y 2
            :line-weight 1
            :line-color 0
            :line-alpha 50
            :phi 15
            :connection-radius 70
            :connection-ramp 6})


;; LIFECYCLE
(defn update-points [{:keys [point-count freq-x freq-y mod-freq-x mod-freq-y phi points] :as state}]
  (doseq [i (range point-count)
          :let [angle         (q/map-range i 0 point-count 0 q/TWO-PI)
                x             (* (q/sin (+ (q/radians phi) (* angle freq-x))) (q/cos (* angle mod-freq-x)))
                y             (* (q/sin (* angle freq-y)) (q/cos (* angle mod-freq-y)))
                ^"[[F" points points
                ^floats point (aget points i)]]
    (aset point 0 (float (* x (- (/ (q/width) 2 ) 30))))
    (aset point 1 (float (* y (- (/ (q/height) 2) 30)))))
  state)

(defn setup []
  (q/frame-rate 60)
  (let [state (assoc state :points (make-array Float/TYPE (:point-count state) 2))]
    (-> state
        (update-points)
        (assoc :channels (cv.core/es8)))))

(defn update-vars [{:keys [channels] :as state}]
  (assoc state
         :phi (or ((nth channels 0)) (:phi state))
         :mod-freq-x (+ 0.001 (:mod-freq-x state))
         :mod-freq-y (+ 0.001 (:mod-freq-y state))))

(defn update [state]
  ;; (println (map #(%) (:channels state)))
  (-> state
      (update-vars)
      (update-points)))

(defn lissa [{:keys [line-weight point-count connection-radius line-color line-alpha points]}]
  (q/color-mode :rgb)
  (q/background 255)
  (q/stroke-weight line-weight)
  (q/stroke-cap :round)
  (q/no-fill)

  (q/push-matrix)
  (q/translate (/ (q/width) 2) (/ (q/height) 2))

  ;; LOOP
  (doseq [i1 (range point-count)
          i2 (range i1)
          :let [^"[[F" points points
                ^floats p1 (aget points i1)
                ^floats p2 (aget points i2)
                x1 (aget p1 0)
                y1 (aget p1 1)
                x2 (aget p2 0)
                y2 (aget p2 1)
                d       (q/dist x1 y1 x2 y2)
                a       (q/pow (/ 1 (/ d (+ 1 connection-radius))) 6)]]

    (if (< d connection-radius)
      (do (q/stroke line-color (min Float/MAX_VALUE (* a line-alpha)))
          (q/line x1 y1 x2 y2))))
  (q/pop-matrix))

(defn draw [state]
  (lissa state))

(q/defsketch gen-art
  :title "Lissajous"
  :size [800 800]
  :setup setup
  :update update
  :draw draw
  :features [:keep-on-top :no-bind-output]
  :middleware [m/fun-mode])



