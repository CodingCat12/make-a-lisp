# Make a Lisp

## Overview

**Make a Lisp** is a lightweight Lisp interpreter implemented in Rust, focusing on functional purity and extensibility.

## Current Status

As of now, Make a Lisp is in the early stages of development. The core features implemented thus far include:
- Support for basic arithmetic operations.
- Logical operations using `And` and `Or` constructs.
- Control flow through conditional constructs (`If` and `IfElse`).
- Input and output operations using `Print` and `PrintLine`.
- Basic parsing of `i32` and `f64` expressions. Such as:
  ```
  (avg (2.0 (+ (1.0 2.0))))
  ```
  (should return `2.5`)

## Technical Details

### Architecture

Make a Lisp is built around the concept of **expression evaluation** using Rust's trait system. The core components of the interpreter are organized as follows:

1. **Expr Trait**: The `Expr<T>` trait defines the interface for all expressions and provides an `eval` method that each expression type must implement. This trait allows for polymorphic evaluation of various types of expressions.

   ```rust
   pub trait Expr<T: Expr<T>>: Debug {
       fn eval(&self) -> T;
   }
   ```

2. **Built-in Functions**: The interpreter includes built-in functions for boolean operations, mathematical operations (like `Sum` and `Product`), control flow (like `If` and `IfElse`), and IO operations (like `PrintLine`). Each of these constructs is represented by a struct implementing the `Expr` trait.

3. **Dynamic Evaluation**: The expressions are dynamically built using Rust's `Box` type for heap allocation, allowing for complex expressions to be formed at runtime. This provides a flexible approach to combining different expression types.

  ```rust
  let expr = Box::new(IfElse {
      check: Box::new(RandomBool),
      case_one: Box::new(Sum {
          items: vec![Box::new(1.0), Box::new(2.0)],
      }),
      case_two: Box::new(5.0),
  });
  ```

### Differences from Lisp

Lisp is great, but it can be greater. So I want to make a language that is very much like the traditional Lisp, but not quite it. These are the main differences

1. **Syntax**: In Lisp, a function call is made by sequencing the function name and parameters. So to add a list of numbers, you do:

  ```lisp
  (+ 5 6 7)
  ```

  I do that too, but if it's a single parameter internally, it's a single parameter for the end-user. And since `+` sums up a list, and not a bunch of parameters (e.g.: `param1 + param2 + param3`,) the syntax in Make a Lisp would look like this:

  ```
  (+ (5 6 7))
  ```

  But functions with non-list parameters (like `-`,) still use syntax like

  ```
  (- 21 11)
  ```

2. **Everything is an expression**: In Lisp, everything is an expression. But in Make a Lisp *everything* is an expression. Even keywords like `if` and `let` are expressions

## Building

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Nix](https://nixos.org/download/#download-nix) (optional) 

### Installation

- Using Cargo:
  ```bash
  git clone https://github.com/CodingCat12/make-a-lisp.git
  cd make-a-lisp
  cargo run
  ```

- Using Nix:
  ```bash
  git clone https://github.com/CodingCat12/make-a-lisp.git
  cd make-a-lisp
  nix run .#lisp
  ```
