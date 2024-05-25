(ns advent.core
  (:require [clojure.java.io :as io]
            [clojure.string :as str])
  (:gen-class))

(defn into-int [s] (Integer/parseInt s))
  
(defn get-input [path] (slurp (io/resource path)))
(defn get-input-lines [path] (str/split-lines (get-input path)))
