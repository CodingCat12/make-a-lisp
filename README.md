# Lisp

## About

This is Lisp:
```lisp
(let x 5)
(+ x 6)
```

For the fun of it, I want to try to Make a Lisp. I'm implementing `Expr<T>` traits on structs in Rust that form expressions that mimic the capabilities of Lisp.
My goal is to be able to convert a string to an expression built on this type system, which I can evaluate to run some code.
