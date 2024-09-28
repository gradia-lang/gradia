print (reduce (range 1 101) ""
(lambda '(n output)
    '(if-else (= 0 (% n 15))
        '(concat output "FizzBuzz ")
        '(if-else (= 0 (% n 3))
            '(concat output "Fizz ")
            '(if-else (= 0 (% n 5))
                '(concat output "Buzz ")
                '(concat output (concat n " ")))))))
