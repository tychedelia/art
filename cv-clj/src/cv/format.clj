(ns cv.format)

(defn audio-format [{:keys [sample-rate sample-size channels signed big-endian]}]
  (javax.sound.sampled.AudioFormat. sample-rate sample-size channels signed big-endian))

(def x1-41000-16bit
  (audio-format
   { :sample-rate 44100
     :sample-size 16
     :signed true
     :channels 1
     :big-endian false }))

(def x2-41000-16bit
  (let [sample-rate 44100
        sample-size 16
        channels 2
        signed true
        big-endian false]
    (javax.sound.sampled.AudioFormat. sample-rate sample-size channels signed big-endian)))

(def x4-44100-16bit
  (let [sample-rate 44100
        sample-size 16
        channels 4
        signed true
        big-endian false]
    (javax.sound.sampled.AudioFormat. sample-rate sample-size channels signed big-endian)))

(def x12-44100-16bit
  (let [sample-rate 44100
        sample-size 16
        channels 12
        signed true
        big-endian false]
    (javax.sound.sampled.AudioFormat. sample-rate sample-size channels signed big-endian)))

(def x4-88200-16bit
  (let [sample-rate 88200
        sample-size 16
        channels 4
        signed true
        big-endian false]
    (javax.sound.sampled.AudioFormat. sample-rate sample-size channels signed big-endian)))

(def x4-96000-16bit
  (let [sample-rate 96000
        sample-size 16
        channels 4
        signed true
        big-endian false]
    (javax.sound.sampled.AudioFormat. sample-rate sample-size channels signed big-endian)))

(def x4-96000-24bit
  (let [sample-rate 96000
        sample-size 24
        channels 4
        signed true
        big-endian false]
    (javax.sound.sampled.AudioFormat. sample-rate sample-size channels signed big-endian)))
(def x12-96000-16bit
  (let [sample-rate 96000
        sample-size 16
        channels 12
        signed true
        big-endian false]
    (javax.sound.sampled.AudioFormat. sample-rate sample-size channels signed big-endian)))

(def x12-96000-24bit
  (let [sample-rate 96000
        sample-size 24
        channels 12
        signed true
        big-endian false]
    (javax.sound.sampled.AudioFormat. sample-rate sample-size channels signed big-endian)))


