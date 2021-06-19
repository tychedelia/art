(ns cv.sketch.core
  (:require
   [clojure.spec.alpha :as s]
   [expound.alpha :as expound]
   [cv.core :as core]
   [clojure.core.async :as async]
   [quil.core :as q]
   [quil.middleware :as m]))

;; mutable vars
(def !current-sketch (atom :default))
(defn default-setup []
  (q/frame-rate 30)
  (q/background 0)
  {})
(defn default-update [state] state)
(defn default-draw [state])
(def default-sketch {:setup default-setup
                     :update default-update
                     :draw default-draw})
(def !sketches (atom {:default default-sketch}))

;; ctor
(defn sketch [key sketch-fns]
  (expound/expound ::sketch sketch-fns)
  (swap! !sketches assoc key sketch-fns))

(s/def ::sketch
  (s/keys :req-un [::setup ::update ::draw]))

;; run
(defn setup-wrapper []
  (apply (:setup (@!current-sketch @!sketches)) nil))
(defn update-wrapper [state]
  (println state)
  (apply (:update (@!current-sketch @!sketches)) state))
(defn draw-wrapper [state]
  (apply (:draw (@!current-sketch @!sketches)) state))

(defn init []
  (q/defsketch gen-art
    :title "cv"
    :size [600 600]
    :setup setup-wrapper
    :update update-wrapper
    :draw draw-wrapper
    :features [:no-bind-output]
    :middleware [m/fun-mode]))

(init)
