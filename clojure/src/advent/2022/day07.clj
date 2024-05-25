(ns advent.2022.day07
  (:require [advent.core :refer [get-input-lines, into-int]]
            [clojure.string :refer [split, starts-with?]])
  (:gen-class))

(def input (get-input-lines "2022/day07.txt"))

(defn get-name [line] (last (split line #" ")))
(defn get-size [line] (into-int (first (split line #" "))))
(defn get-path [state line] (concat [:tree] (:path state) [(get-name line)]))

(defn walk-log [state line]
  (cond
    (= line "$ ls") state
    (= line "$ cd /") (assoc state :path ())
    (= line "$ cd ..") (update state :path drop-last)
    (starts-with? line "$ cd ") (update state :path #(concat % [(get-name line)]))
    (starts-with? line "dir") (assoc-in state (get-path state line) {})
    :else (assoc-in state (get-path state line) (get-size line))))

(def file-tree (->> input
                    (reduce walk-log {:path () :tree {}})
                    :tree))

(defn sum-small-dirs [coll]
  (reduce-kv
   (fn [acc _ v]
     (if (map? v)
       (let [res (sum-small-dirs v)
             sm (:smol res)
             sz (:size res)]
         (-> acc
             (update :smol + sm (if (< sz 100000) sz 0))
             (update :size + sz)))
       (update acc :size + v)))
   {:smol 0 :size 0} coll))

(def part1 (:smol (sum-small-dirs file-tree)))

(defn -main []
  (println "2022-07 part 1:" part1))