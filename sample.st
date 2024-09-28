print (reduce (range 1 101):list ""
(lambda '(i output)
    '(if-else (= 0 (% i:number 15)):bool
        '(concat output "FizzBuzz ")
        '(if-else (= 0 (% i:number 3)):bool
            '(concat output "Fizz ")
            '(if-else (= 0 (% i:number 5)):bool
                '(concat output "Buzz ")
                '(concat output (concat i " ")))))))
