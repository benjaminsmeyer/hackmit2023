#lang racket

(require json)

#|
                                    STAGE 1

   Welcome to the fist problem, young adventurer! Before you embark, you must
   learn how to survive in this world. In this problem, you'll receive a list
   of enemies with their health. You must kill all of them by setting their
   health to 0 in one move. Good luck!

|#

(struct enemy (health) #:transparent)

(define/contract (solve enemies)
  (-> (listof enemy?) (listof enemy?))
  ;; TODO: Kill all enemies
  ;; Hint: an enemy is dead when their health is 0.

  ;;; WRITE YOUR CODE HERE ;;;



  ;;; END OF YOUR CODE HERE ;;;

  enemies)

(module+ main
  (define input-json (read-json (current-input-port)))
  (define num-enemies (hash-ref input-json 'numEnemies))
  (define starting-health (hash-ref input-json 'startingHealth))
  (define enemies
    (for/list ([_ (in-range num-enemies)])
      (enemy starting-health)))
  (define result (solve enemies))

  (define result-strs
    (for/list ([enemy result])
      (number->string (enemy-health enemy))))
  (define result-str (string-join result-strs ","))
  (display result-str))
