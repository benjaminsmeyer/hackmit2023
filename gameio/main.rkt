#lang racket

(require "./templates-info.rkt")

(module+ main
  (require racket/cmdline)

  (define language (make-parameter #f))
  (define stage (make-parameter #f))
  (define filename (make-parameter #f))
  (define reset? (make-parameter #f))

  (command-line
   #:program "gameio"
   #:once-each
   [("-l" "--language") cli-language
                        "Your language of choice"
                        (language cli-language)]
   [("-s" "--stage")    cli-stage
                        "The stage you want to tackle"
                        (stage cli-stage)]
   [("-f" "--filename") cli-filename
                        "A file to run the stage against (optional)"
                        (filename cli-filename)]
   [("-r" "--reset")    "Reset the file template (if not -f)"
                        (reset? #t)]
   #:args ()
   (void))

  (cond [(or (not (language))
             (not (stage)))
         (displayln "--language and --stage flags are required, please use --help for more info")]
        [(string? (filename))
         "string"]
        [(false? (filename))
         (generate-templates #:language (language)
                             #:stage    (stage)
                             #:reset?   (reset?))]))

(define (generate-templates #:language language
                            #:stage    stage
                            #:reset?   reset?)
  (define supported-language-names
    (for/list ([lang supported-languages])
      (symbol->string (language-name lang))))

  (unless (member language supported-language-names)
    (error 'generate-templates
           "~a is not a supported language"
           language))

  (define language^ (string->symbol language))

  (define stage^
    (findf (lambda (s)
            (or (equal? (symbol->string (stage-name s))
                        stage)
                (equal? (stage-number s)
                        stage)))
          stages))

  (unless stage^
    (error 'generate-templates
           "~a is an unknown stage"
           stage^))

  (define templates
    (stage-templates stage^))

  (define template-path
    (cdr (hash-ref templates language^)))
  (define template-filename
    (car (hash-ref templates language^)))

  (displayln (~a "creating " template-filename))

  (copy-file template-path
             template-filename
             #:exists-ok? reset?)

  (void))

(define (run-code #:language language
                  #:stage    stage
                  #:filename filename)
  (void))
