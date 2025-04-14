pub mod builtins;
#[cfg(test)]
mod tests;

use std::fmt::Debug;

pub trait Expr<T: Expr<T>>: Debug {
    fn eval(&self) -> T;
}

impl<T: Clone + Debug> Expr<T> for T {
    fn eval(&self) -> Self {
        self.clone()
    }
}
