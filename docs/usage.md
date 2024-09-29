# Document of Statia Programming Language

## Calculation
```
(+ 1 (- 2 (* 3 (/ 4 5))))
```

### With type annotation
```
(+:function 1:number (-:function 2:number (*:function 3:number (/:function 4:number 5:number):number):number):number):number

```

## Define variable
```
(define x 5)
```

### With type annotation
```
(define:function x:symbol 5:number):number
```

## Define function
```
(define '(x2 n) '(* n 2))
```

### With type annotation
```
(define:function '(x2:symbol n:symbol):list '(*:function n:number 2:number):list):function
```

## Lambda function
```
(lambda '(n) '(* n 2))
```

### With type annotation
```
(lambda:function '(n:symbol):list '(*:function n:nunber 2:number):list):function
```
