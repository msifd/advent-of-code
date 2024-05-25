(ns advent.2022.day01
  (:require [advent.core :refer [get-input-lines]])
  (:gen-class))

(defn sum-list [coll]
  (if (= '("") coll)
    0
    (apply + (map #(Integer/parseInt %) coll))))

(def sorted-inventories
  (->> "2022/day01.txt"
       get-input-lines
       (partition-by empty?)
       (map sum-list)
       (filter #(not= 0 %))
       sort reverse))

(def part1 (first sorted-inventories))
(def part2 (apply + (take 3 sorted-inventories)))

(defn -main []
  (println "2022-01 part 1:" part1)
  (println "2022-01 part 2:" part2))