(ns advent.2022.day06
  (:require [advent.core :refer [get-input]])
  (:gen-class))

(def input (get-input "2022/day06.txt"))

(defn find-marker-pos [len coll]
  (->> (partition len 1 coll)
       (take-while #(< (count (distinct %)) len))
       count
       (+ len)))

(def part1 (find-marker-pos 4 input))
(def part2 (find-marker-pos 14 input))

(defn -main []
  (println "2022-06 part 1:" part1)
  (println "2022-06 part 2:" part2))