#lang info
(define collection "gameio")
(define deps '("base"))
(define build-deps '("scribble-lib" "racket-doc" "rackunit-lib" "threading" "struct-plus-plus" "hash-star"))
(define scribblings '(("scribblings/gameio.scrbl" ())))
(define pkg-desc "Handling IO for the algorithmic game")
(define version "0.1")
(define pkg-authors '(priime))
(define license 'MIT)
