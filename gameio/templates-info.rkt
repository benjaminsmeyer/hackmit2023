#lang racket

(require struct-plus-plus)

(provide (all-defined-out))

(struct++ language
          ([name symbol?]
           [extension string?]
           [shell-command string?])
          #:transparent)

(struct++ stage
          ([name symbol?]
           [number string?]
           [(templates (hash)) hash?])
          #:transparent)

(define supported-languages
  (list (language++ #:name          'python
                    #:extension     "py"
                    #:shell-command "python")
        (language++ #:name          'racket
                    #:extension     "rkt"
                    #:shell-command "racket")))

(define stages-info
  (list (stage++ #:name 'add
                 #:number "0")
        (stage++ #:name 'killenemies
                 #:number "1")
        (stage++ #:name 'maze
                 #:number "2")))

(define stages
  (for/list ([stage stages-info])
    (define templates
      (for/hash ([language supported-languages])
        (define lang-name (language-name language))
        (define language-filename (~a (stage-name stage)
                                      "."
                                      (language-extension language)))
        (define json-filename (~a (stage-name stage) ".json"))
        (define templates-directory "templates")
        (define filepath (string-join
                          `(,templates-directory
                            ,(stage-number stage)
                            ,language-filename)
                          "/"))
        (define json-filepath (string-join
                               `(,templates-directory
                                 ,(stage-number stage)
                                 ,json-filename)
                               "/"))

        (values lang-name (list language-filename filepath json-filepath))))

    (set-stage-templates stage templates)))
