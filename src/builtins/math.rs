use crate::expr::{Expr, ListOf};
use num::FromPrimitive;
use std::{iter, ops};

#[derive(Debug)]
pub struct Sum<T: Expr<T> + iter::Sum + 'static> {
    pub items: ListOf<T>,
}

impl<T: Expr<T> + iter::Sum> Expr<T> for Sum<T> {
    fn eval(&self) -> T {
        self.items.iter().map(|x| (*x).eval()).sum()
    }
}

impl<T: Expr<T> + iter::Sum> Sum<T> {
    pub fn new(items: ListOf<T>) -> Box<Self> {
        Box::new(Self { items })
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

impl<T: Expr<T> + iter::Product> Product<T> {
    pub fn new(items: ListOf<T>) -> Box<Self> {
        Box::new(Self { items })
    }
}

#[derive(Debug)]
pub struct Average<T: Expr<T> + iter::Sum> {
    pub items: ListOf<T>,
}

impl<T: Expr<T> + iter::Sum + FromPrimitive + ops::Div<Output = T>> Expr<T> for Average<T> {
    fn eval(&self) -> T {
        let len = T::from_usize(self.items.len()).unwrap();
        let total: T = self.items.iter().map(|x| (*x).eval()).sum();
        total / len
    }
}

impl<T: Expr<T> + iter::Sum> Average<T> {
    pub fn new(items: ListOf<T>) -> Box<Self> {
        Box::new(Self { items })
    }
}
