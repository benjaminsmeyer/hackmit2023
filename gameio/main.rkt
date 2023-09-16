#lang racket

(require "./templates-info.rkt")

(module+ main
  (require racket/cmdline)

  (define language (make-parameter #f))
  (define stage (make-parameter #f))
  (define filename (make-parameter #f))

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
   #:args ()
   (void))

  (cond [(or (not (language))
             (not (stage)))
         (displayln "--language and --stage flags are required, please use --help for more info")]
        [(string? (filename))
         "string"]
        [(false? (filename))
         "false"])

  (match (filename)
    [(? string?)
     (run-code #:language (language)
               #:stage    (stage)
               #:filename (filename))]
    [#f
     (generate-templates #:language (language)
                         #:stage    (stage))]))

(define (generate-templates #:language language
                            #:stage    stage)
  (void))

(define (run-code #:language language
                  #:stage    stage
                  #:filename filename)
  (void))
