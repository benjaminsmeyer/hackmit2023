#lang racket

(require json)

#|
                                    STAGE 0

  Welcome, youngling! Before you can become an adventurer, we must test to
  see that you are quick-minded. Complete this function that sums two numbers
  together.

|#

(define/contract (add a b)
  (-> number? number? number?)
  ;; TODO: Add two numbers
  ;; Hint: what operator is used for addition?

  ;; WRITE YOUR CODE HERE ;;;

  0

  ;;; END OF YOUR CODE HERE ;;;

  ;; Delete the `(void)` when you are done.
  )

(module+ main
  (define input-json (read-json (current-input-port)))
  (define a (hash-ref input-json 'a))
  (define b (hash-ref input-json 'b))
  (define result (add a b))
  (display result))
