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

#[derive(Debug)]
pub struct Overwrite<D: Expr<D> + 'static, T: Expr<T>> {
    pub function: Box<dyn Expr<D>>,
    pub value: Box<dyn Expr<T>>,
}

impl<D: Expr<D>, T: Expr<T> + Clone> Expr<T> for Overwrite<D, T> {
    fn eval(&self) -> T {
        self.function.eval();
        self.value.eval()
    }
}

pub struct Null;
pub type Ignore<T> = Overwrite<T, Null>;
