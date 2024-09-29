(define '(fizzbuzz i)
    '(if-else (= 0 (% i:number 15)):bool
        "FizzBuzz":string
        '(if-else (= 0 (% i:number 3)):bool
            "Fizz":string
            '(if-else (= 0 (% i:number 5)):bool
                "Buzz":string
                '(cast i "string")
            ):list
        ):list
    ):list
):function
(print (join (map (range 1 101):list fizzbuzz:function):list new-line:string):string)
