#lang racket

(require struct-plus-plus)

(provide (all-defined-out))

(struct++ language
          ([name symbol?]
           [extension string?])
          #:transparent)

(struct++ stage
          ([name symbol?]
           [number string?]
           [(templates (hash)) hash?])
          #:transparent)

(define supported-languages
  (list (language++ #:name 'python
                    #:extension "py")
        (language++ #:name 'racket
                    #:extension "rkt")))

(define stages-info
  (list (stage++ #:name 'add
                 #:number "00")
        (stage++ #:name 'killenemies
                 #:number "01")
        (stage++ #:name 'findway
                 #:number "02")))

(define stages
  (for/list ([stage stages-info])
    (define templates
      (for/hash ([language supported-languages])
        (define lang-name (language-name language))
        (define filename (~a (stage-name stage)
                             "."
                             (language-extension language)))
        (define templates-directory "templates")
        (define filepath (string-join
                          `(,templates-directory
                            ,(stage-number stage)
                            ,filename)
                          "/"))
        
        (values lang-name (cons filename filepath))))

    (set-stage-templates stage templates)))
