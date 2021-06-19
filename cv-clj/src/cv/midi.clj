(ns cv.midi
  (:require [overtone.midi :as midi]
            [cv.core :as core]
            [clojure.core.async :as async]))
(defn midi-bus [name]
  (println (str "open midi bus " name))
  (if-let [device (midi/midi-in name)]
    (do
      (println (str "openend " name))
      (midi/midi-handle-events device #(println (:data2 %))))))


(defn vir-midi [] (midi-bus {:channels [] :name "VirMIDI [hw:1,0,0]"}))

(map :name )(midi/midi-devices)

(def MidiInDeviceInfo (nth (.getDeclaredClasses com.sun.media.sound.MidiInDeviceProvider) 0))

(defn- is-device-instance?
  [device] (.equals MidiInDeviceInfo (.getClass device)))

(defn get-devices [] (seq (javax.sound.midi.MidiSystem/getMidiDeviceInfo)))

(defn- get-device-info [name]
  (first (filter #( and (= (.getName %) name) (is-device-instance? %)) (get-devices))))

(defn get-device [name]
  (let [device-info (get-device-info name)]
    (javax.sound.midi.MidiSystem/getMidiDevice device-info)))

(defn- open-device [device]
  (if (not (.isOpen device))
    (.open device)))

(defn ->Reciever []
  (reify javax.sound.midi.Receiver
    (close [this] (println "Reciever was closed early."))
    (send [this msg time] (println (.getChannel msg)))))

(defn- set-reciever [device]
  (.setReceiver (.getTransmitter device) (->Reciever)))


(def dev (get-device "IAC"))
(open-device dev)
(set-reciever dev)
