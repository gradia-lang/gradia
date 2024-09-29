print (
    reduce (range 1 101):list "" (lambda '(i output)
        '(if-else (= 0 (% i:number 15)):bool
        '(concat output (concat "FizzBuzz" new-line):string)
            '(if-else (= 0 (% i:number 3)):bool
                '(concat output (concat "Fizz" new-line):string)
                '(if-else (= 0 (% i:number 5)):bool
                    '(concat output (concat "Buzz" new-line):string)
                    '(concat output (concat i new-line):string)
                )
            )
        )
    )
)
