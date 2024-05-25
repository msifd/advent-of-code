(ns advent.2022.day04
  (:require [advent.core :refer [get-input-lines]]
            [clojure.string :as str])
  (:gen-class))

(def input (get-input-lines "2022/day04.txt"))

(defn parse-ranges [line]
  (let [[a b c d] (map #(Integer/parseInt %) (str/split line #"[,-]"))]
    (list (list a b) (list c d))))

(defn includes? [[a b] [c d]] (and (<= a c) (>= b d)))
(defn includes-any? [[l r]] (or (includes? l r) (includes? r l)))

(defn in-range? [[a b] x] (and (<= a x) (>= b x)))
(defn overlaps? [pair [c d]] (or (in-range? pair c) (in-range? pair d)))
(defn overlaps-any? [[l r]] (or (overlaps? l r) (overlaps? r l)))

(def part1 (->> input
                (map parse-ranges)
                (filter includes-any?)
                count))

(def part2 (->> input
                (map parse-ranges)
                (filter overlaps-any?)
                count))

(defn -main []
  (println "2022-04 part 1:" part1)
  (println "2022-04 part 2:" part2))