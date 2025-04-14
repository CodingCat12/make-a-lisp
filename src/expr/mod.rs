pub mod builtins;
#[cfg(test)]
mod tests;

use std::fmt::Debug;

pub trait Expr<T>: Debug {
    fn eval(&self) -> T;
}

macro_rules! expr_impl {
    // stolen from std::ops::Add (https://doc.rust-lang.org/src/core/ops/arith.rs.html#96)
    ($($t:ty)*) => ($(
        impl Expr<Self> for $t {
            fn eval(&self) -> Self {
                self.clone()
            }
        }
    )*)
}

expr_impl! { i32 bool f64 String }
