print (reduce (range 1 101) ""
(lambda '(i output)
    '(if-else (= 0 (% i:number 15))
        '(concat output "FizzBuzz ")
        '(if-else (= 0 (% i:number 3))
            '(concat output "Fizz ")
            '(if-else (= 0 (% i:number 5))
                '(concat output "Buzz ")
                '(concat output (concat i " ")))))))
