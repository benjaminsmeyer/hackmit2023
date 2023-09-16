#lang racket

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


