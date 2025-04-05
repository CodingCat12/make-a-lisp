# Make a Lisp

## Overview

**Make a Lisp** is a lightweight Lisp interpreter implemented in Rust, designed to explore the principles of language design and evaluation. This project endeavors to bring the expressive power of Lisp into the Rust ecosystem, leveraging Rust's type safety, performance, and concurrency capabilities.

## Project Goals

The primary goal of Make a Lisp is to create a functional and extensible Lisp-like language that:
- Allows users to write and evaluate Lisp expressions.
- Supports basic control flow and data manipulation constructs.
- Encourages exploration of language design concepts, such as parsing and evaluation mechanisms.
- Provides a foundation for further language extensions, such as macro systems or advanced data structures.

## Current Status

As of now, Make a Lisp is in the early stages of development. The core features implemented thus far include:
- Support for basic arithmetic operations.
- Logical operations using `And` and `Or` constructs.
- Control flow through conditional constructs (`If` and `IfElse`).
- Input and output operations using `Print` and `PrintLine`.

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

Lisp is great, but it can be greater. So I want to make a language that is very much like the traditional Lisp, but not quite it.

1. **Syntax**: 

## Building

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Nix](https://nixos.org/download/#download-nix) (optional) 

### Installation

- With Cargo:
  ```bash
  git clone github.com/CodingCat12/make-a-lisp.git
  cd make-a-lisp
  cargo run
  ```

- With Nix:
  ```bash
  git clone github.com/CodingCat12/make-a-lisp.git
  cd make-a-lisp
  nix run .#lisp
  ```
