(define '(fizzbuzz i:number)
    '(if-else (= 0 (% i 15)):bool
        "FizzBuzz":string
        '(if-else (= 0 (% i 3)):bool
            "Fizz":string
            '(if-else (= 0 (% i 5)):bool
                "Buzz":string
                '(cast i "string")
            ):list
        ):list
    ):list
):function
(print (join (map (range 1 101):list fizzbuzz:function):list new-line:string):string)
