(ns advent.2022.day03
  (:require [advent.core :refer [get-input-lines]]
            [clojure.set :refer [intersection]])
  (:gen-class))

(def input (get-input-lines "2022/day03.txt"))

(defn split-at-half [s]
  (split-at (/ (count s) 2) s))

(defn find-intersection [entries]
  (->> entries
       (map set)
       (apply intersection)
       first))

(defn priority [c]
  (let [i (int c)]
    (if (> i 96) (+ i -96) (+ i -64 26))))

(def part1 (->>
            input
            (map split-at-half)
            (map find-intersection)
            (map priority)
            (reduce +)))

(def part2 (->>
            input
            (partition 3)
            (map find-intersection)
            (map priority)
            (reduce +)))

(defn -main []
  (println "2022-03 part 1:" part1)
  (println "2022-03 part 2:" part2))