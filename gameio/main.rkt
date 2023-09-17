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
         (run-code #:language     (language)
                   #:stage        (stage)
                   #:filename     (filename))]
        [(false? (filename))
         (generate-templates #:language (language)
                             #:stage    (stage)
                             #:reset?   (reset?))]))

(define (supported-language? language)
  (define supported-language-names
    (for/list ([lang supported-languages])
      (symbol->string (language-name lang))))

  (findf (curry equal? language)
         supported-language-names))

(define (check-supported-language language)
  (unless (supported-language? language)
    (error 'unsupported-language
           "~a is not a supported language"
           language)))

(define (get-stage stage)
  (findf (lambda (s)
             (or (equal? (symbol->string (stage-name s))
                         stage)
                 (equal? (stage-number s)
                         stage)))
           stages))

(define (check-valid-stage stage)
  (unless stage
    (error 'unknown-stage
           "~a is an unknown stage"
           stage)))

(define (generate-templates #:language language
                            #:stage    stage
                            #:reset?   reset?)
  (check-supported-language language)
  (define language^ (string->symbol language))

  (define stage^ (get-stage stage))

  (check-valid-stage stage^)

  (define templates (stage-templates stage^))

  (define template-filename
    (first (hash-ref templates language^)))
  (define template-path
    (second (hash-ref templates language^)))  

  (displayln (~a "creating " template-filename))

  (copy-file template-path
             template-filename
             #:exists-ok? reset?)

  (void))

(define (run-code #:language language
                  #:stage    stage
                  #:filename filename)
  (check-supported-language language)
  (define language^ (string->symbol language))
  (define language+
    (findf (lambda (l) (symbol=? (language-name l) language^))
           supported-languages))
  (define executing-program (~a "/usr/bin/" (language-shell-command language+)))
  (println executing-program)

  (define stage^ (get-stage stage))
  (check-valid-stage stage^)

  (define executing-filename (~a "./" filename))
  (println executing-filename)
  (println (file-exists? executing-filename))

  (define templates (stage-templates stage^))
  (define stdin-json-input-filepath
    (third (hash-ref templates language^)))

  (define stdin (open-input-file stdin-json-input-filepath))

  (define-values (user-program stdout-port _ stderr-port)
    (subprocess #f
                stdin
                #f
                executing-program
                executing-filename))

  (subprocess-wait user-program)

  (unless (zero? (subprocess-status user-program))
    (error 'program-error
           "Encountered an error running ~a:\n~a"
           filename
           (port->string stderr-port)))

  (define user-output (port->string stdout-port))
  (println user-output)

  (define binary-name "./game")

  (define-values (game-program game-stdout __ game-stderr)
    (subprocess #f #f #f
                "./game"
                "--input"
                user-output
                "--stage"
                stage))

  (subprocess-wait game-program)

  (println (subprocess-status game-program))
  (println (port->string game-stderr))

  (void))
