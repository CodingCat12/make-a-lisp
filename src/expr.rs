use std::fmt::Debug;

pub trait Expr<T: Expr<T>>: Debug {
    fn eval(&self) -> T;
}

pub type EvalTo<T> = Box<dyn Expr<T>>;
pub type ListOf<T> = Vec<EvalTo<T>>;

impl<T: Clone + Debug> Expr<T> for T {
    fn eval(&self) -> Self {
        self.clone()
    }
}

#[derive(Debug)]
pub struct Overwrite<D: Expr<D> + 'static, T: Expr<T>> {
    pub function: EvalTo<D>,
    pub value: EvalTo<T>,
}

impl<D: Expr<D>, T: Expr<T> + Clone> Expr<T> for Overwrite<D, T> {
    fn eval(&self) -> T {
        self.function.eval();
        self.value.eval()
    }
}

pub struct Null;
pub type Ignore<T> = Overwrite<T, Null>;
