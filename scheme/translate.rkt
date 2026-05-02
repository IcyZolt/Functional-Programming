#lang racket

;basic list of utility functions
(define first car)
(define second cadr)
(define third caddr)
(define rest cdr)

;number mapping lists (required task symbols)
(define chinese '(ling yi er san si wu liu qi ba jiu shi))
(define english '(zero one two three four five six seven eight nine ten))

;combined mapping table
(define mappings
  '((ling 0) (yi 1) (er 2) (san 3) (si 4) (wu 5) (liu 6) (qi 7) (ba 8) (jiu 9) (shi 10)
    (zero 0) (one 1) (two 2) (three 3) (four 4) (five 5) (six 6) (seven 7) (eight 8) (nine 9) (ten 10)))

;translate single word to number, returns #f if invalid
(define (translate-word w)
  (let ((pair (assq w mappings)))
    (if pair (cadr pair) #f)))  ;look up word, get number from pair

;check if word is valid (translatable)
(define (valid-word? w)
  (not (eq? (translate-word w) #f)))

;convert number list to space-separated string
(define (numbers->string nums)
  (if (null? nums)
      ""
      (let loop ((rest (cdr nums)) (result (number->string (car nums))))
        (if (null? rest)
            result  ;done, return accumulated string
            (loop (cdr rest) (string-append result " " (number->string (car rest))))))))  ;append next number

;generate operation expression string
(define (expression-str nums op)
  (if (null? nums)
      ""
      (let loop ((rest (cdr nums)) (result (number->string (car nums))))
        (if (null? rest)
            result
            (loop (cdr rest) (string-append result op (number->string (car rest))))))))  ;insert operation symbol

;main function: process input list, output translation + math results
(define (go input)
  (let* ((valid-words (filter valid-word? input))  ;remove unrecognized words
         (nums (map translate-word valid-words)))  ;convert to numbers
    (display "Translation: ") (display (numbers->string nums)) (newline)
    (cond
      ((null? nums)
       (display "Addition: 0") (newline)
       (display "Multiplication: 1") (newline))
      (else
       (display "Addition: ")
       (display (expression-str nums " + "))
       (display " = ")
       (display (apply + nums))  ;sum all translated numbers
       (newline)
       (display "Multiplication: ")
       (display (expression-str nums " * "))
       (display " = ")
       (display (apply * nums))  ;product of all translated numbers
       (newline)))))