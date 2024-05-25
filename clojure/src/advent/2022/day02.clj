(ns advent.2022.day02
  (:require [advent.core :refer [get-input-lines]]
            [clojure.string :as str])
  (:gen-class))

(def strategies
  (->> "2022/day02.txt"
       get-input-lines
       (map #(str/split % #" "))))

(defn what-beats? [a]
  (case a
    "A" "B"
    "B" "C"
    "C" "A"))

(defn what-looses-to? [a]
  (case a
    "A" "C"
    "B" "A"
    "C" "B"))

(defn beats? [a b] (= a (what-beats? b)))

(defn play-rps [[they us]]
  (+
   (case us "A" 1 "B" 2 "C" 3)
   (cond
     (beats? they us) 0
     (= they us) 3
     :else 6)))

(defn replace-with-translation [[they us]]
  (list
   they
   (case us
     "X" "A"
     "Y" "B"
     "Z" "C")))

(defn replace-with-tactical-move [[they us]]
  (list
   they
   (case us
     "X" (what-looses-to? they)
     "Y" they
     "Z" (what-beats? they))))

(def part1 (->>
            strategies
            (map replace-with-translation)
            (map play-rps)
            (reduce +)))
(def part2 (->>
            strategies
            (map replace-with-tactical-move)
            (map play-rps)
            (reduce +)))

(defn -main []
  (println "2022-02 part 1:" part1)
  (println "2022-02 part 2:" part2))