use crate::expr::{Expr, ListOf};
use std::iter;

#[derive(Debug)]
pub struct Sum<T: Expr<T> + iter::Sum + 'static> {
    pub items: ListOf<T>,
}

impl<T: Expr<T> + iter::Sum> Expr<T> for Sum<T> {
    fn eval(&self) -> T {
        self.items.iter().map(|x| (*x).eval()).sum()
    }
}

#[derive(Debug)]
pub struct Product<T: Expr<T> + iter::Product + 'static> {
    pub items: ListOf<T>,
}

impl<T: Expr<T> + iter::Product> Expr<T> for Product<T> {
    fn eval(&self) -> T {
        self.items.iter().map(|x| (*x).eval()).product()
    }
}
