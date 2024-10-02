# Document of Gradia Programming Language

This documentation provides an overview of the Gradia language's syntax and its powerful type annotation features.

---

## 1. Calculation

Gradia allows for basic mathematical expressions similar to Lisp.

```lisp
(+ 1 (- 2 (* 3 (/ 4 5))))
```

### With Type Annotation

Gradia supports annotating every part of the expression with types.

```lisp
(+:function 1:number (-:function 2:number (*:function 3:number (/:function 4:number 5:number):number):number):number):number
```

---

## 2. Defining a Variable

To define a variable in Gradia:

```lisp
(define x 5)
```

### With Type Annotation

The variable can also be annotated to specify its type:

```lisp
(define:function x:symbol 5:number):number
```

- **`x:symbol`**: `x` is a symbol representing a variable.
- **`5:number`**: The value assigned to `x` is of type `number`.

---

## 3. Defining a Function

Functions are defined with the `define` keyword. Here's an example:

```lisp
(define '(x2 n) '(* n 2))
```

### With Type Annotation

You can also define functions with type annotations for the parameters and return types:

```lisp
(define:function '(x2:symbol n:number):list '(*:function n:number 2:number):list):function
```

- **`x2:symbol`**: The function name is represented as a symbol.
- **`n:number`**: The parameter `n` is of type `number`.
- **`(* n 2)`**: The multiplication operation is annotated with type `number`.
- **Return Type**: The function returns a list representing the evaluated expression.

---

## 4. Lambda Functions

Gradia supports anonymous functions (lambdas) as first-class citizens:

```lisp
(lambda '(n) '(* n 2))
```

### With Type Annotation

You can annotate lambda functions as well:

```lisp
(lambda:function '(n:number):list '(*:function n:number 2:number):list):function
```

- **`lambda:function`**: Indicates this is a lambda function.
- **`n:number`**: The parameter `n` is of type `number`.
- **`(* n 2)`**: Annotated as a function that operates on numbers and returns a list.

---

## 5. Lists

Gradia uses single quotes for lists, and they can contain mixed types:

```lisp
'(1 2 3 "abc")
```

### With Type Annotation

Annotate the elements of the list with their types:

```lisp
'(1:number 2:number 3:number "abc":string):list
```

- **List Elements**: Each element in the list is annotated with its type (`number` or `string`).
- **Return Type**: The entire expression is a `list`.

---

## 6. Plural Expressions

Gradia supports multiple expressions within a single evaluation context:

```lisp
(eval '(print "hello") '(+ 1 2))
```

### With Type Annotation

Both expressions within `eval` can be annotated:

```lisp
(eval:function '(print:function "hello":string):list '(+:function 1:number 2:number):list):number
```

- **`eval:function`**: This is the function call for `eval`.
- **`print:function`**: The `print` function takes a string (`"hello"`) as an argument.
- **`(+ 1 2)`**: The addition operation is annotated with the type `number`.
- **Return Type**: The result is a `number`.
