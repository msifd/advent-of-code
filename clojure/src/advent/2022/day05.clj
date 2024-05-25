(ns advent.2022.day05
  (:require [advent.core :refer [get-input-lines into-int]])
  (:gen-class))

(def input (->> (get-input-lines "2022/day05.txt")
                (split-with (complement empty?))))

(def state-regex (re-pattern #"(?:\[(\w)\]|\s{3})\s?"))
(def init-stacks
  (->> (first input)
       (drop-last 1)
       (map (partial re-seq state-regex))
       (map (partial map second))
       (apply map list)
       (map reverse)
       (map (partial filter identity))
       (into [])))

(def commands-regex (re-pattern #"move (\d+) from (\d) to (\d)"))
(def commands
  (->> (second input)
       (drop 1)
       (map (partial re-seq commands-regex))
       (map flatten)
       (map (partial drop 1))
       (map (partial map into-int))
       (map (fn [[n f t]] {:amount n :from (- f 1) :to (- t 1)}))))

(defn pick-up [s a r] (take-last a (nth s r)))
(def pick-up-and-flip (comp reverse pick-up))

(defn rearrange-crates [state cmd picker]
  (-> state
      (update (:to cmd) #(concat % (picker state (:amount cmd) (:from cmd))))
      (update (:from cmd) #(drop-last (:amount cmd) %))))

(defn rearrange-with-crate-mover-9000 [state cmd]
  (rearrange-crates state cmd pick-up-and-flip))
(defn rearrange-with-crate-mover-9001 [state cmd]
  (rearrange-crates state cmd pick-up))

(defn read-tops [stacks]
  (->> stacks
       (map last)
       (reduce str)))

(def part1 (read-tops (reduce rearrange-with-crate-mover-9000 init-stacks commands)))
(def part2 (read-tops (reduce rearrange-with-crate-mover-9001 init-stacks commands)))

(defn -main []
  (println "2022-05 part 1:" part1)
  (println "2022-05 part 2:" part2))