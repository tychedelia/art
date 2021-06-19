(ns cv.gate
  (:require [cv.cv]))

;; magic numbers
(def default-ratio 5/10)

(def default-max 15000) ;; TODO: find avg max across channels

(def default-length 8)

;; fn
(defn gate
  "
  if the signal is above ratio of the channel's max for n frames
  we consider the gate high, otherwise low

  returns a map of whether the gate was high, and the total gate length
  "

  ;; TODO: handle case where gate is evently split on either side of a
  ;; buffer such that it doesn't register.
  ([buffer]
   (gate {:max default-max} buffer default-length default-ratio))

  ([channel buffer]
   (gate channel buffer default-length default-ratio))

  ([channel buffer n]
   (gate channel buffer n default-ratio))

  ([channel buffer n ratio]
   (let [threshold (* ratio (:max channel))
         avg     (cv.cv/cv buffer)
         length  (reduce (fn [l x]
                           (if (or (> l n) (> x threshold))
                             (inc l)
                             0))
                         0 buffer)]
     {:avg avg :gate (> length n) :length length})))

(defn trigger [buffer]
  "
  triggers are simpler, as we only need to know whether a trigger
  happened during an animation frame, and can merge multiple
  triggers together since we can only impact the effects system
  once per render
  "
  (:gate (gate buffer)))
