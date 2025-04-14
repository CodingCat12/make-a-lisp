use crate::expr::Expr;
use num::FromPrimitive;
use std::{
    iter,
    ops::{self, Sub},
};

#[derive(Debug)]
pub struct Sum<T: Expr<T> + iter::Sum + 'static> {
    pub items: Vec<Box<dyn Expr<T>>>,
}

impl<T: Expr<T> + iter::Sum> Expr<T> for Sum<T> {
    fn eval(&self) -> T {
        self.items.iter().map(|x| (*x).eval()).sum()
    }
}

impl<T: Expr<T> + iter::Sum> Sum<T> {
    pub fn new(items: Vec<Box<dyn Expr<T>>>) -> Box<Self> {
        Box::new(Self { items })
    }
}

#[derive(Debug)]
pub struct Product<T: Expr<T> + iter::Product + 'static> {
    pub items: Vec<Box<dyn Expr<T>>>,
}

impl<T: Expr<T> + iter::Product> Expr<T> for Product<T> {
    fn eval(&self) -> T {
        self.items.iter().map(|x| (*x).eval()).product()
    }
}

impl<T: Expr<T> + iter::Product> Product<T> {
    pub fn new(items: Vec<Box<dyn Expr<T>>>) -> Box<Self> {
        Box::new(Self { items })
    }
}

#[derive(Debug)]
pub struct Average<T: Expr<T> + iter::Sum> {
    pub items: Vec<Box<dyn Expr<T>>>,
}

impl<T: Expr<T> + iter::Sum + FromPrimitive + ops::Div<Output = T>> Expr<T> for Average<T> {
    fn eval(&self) -> T {
        let len = T::from_usize(self.items.len()).unwrap();
        let total: T = self.items.iter().map(|x| (*x).eval()).sum();
        total / len
    }
}

impl<T: Expr<T> + iter::Sum> Average<T> {
    pub fn new(items: Vec<Box<dyn Expr<T>>>) -> Box<Self> {
        Box::new(Self { items })
    }
}

#[derive(Debug)]
pub struct Median<T> {
    pub items: Vec<Box<dyn Expr<T>>>,
}

impl<T: Expr<T> + PartialOrd + Clone> Expr<T> for Median<T> {
    fn eval(&self) -> T {
        let mut sorted: Vec<T> = self.items.iter().map(|x| (*x).eval()).collect();
        sorted.sort_unstable_by(|x, y| x.partial_cmp(y).unwrap());
        sorted[self.items.len() / 2].clone()
    }
}

impl<T: Expr<T> + PartialOrd + Clone> Median<T> {
    pub fn new(items: Vec<Box<dyn Expr<T>>>) -> Box<Self> {
        Box::new(Self { items })
    }
}

#[derive(Debug)]
pub struct Subtraction<T> {
    a: Box<dyn Expr<T>>,
    b: Box<dyn Expr<T>>,
}

impl<T: Expr<T> + Sub<Output = T>> Expr<T> for Subtraction<T> {
    fn eval(&self) -> T {
        self.a.eval() - self.b.eval()
    }
}

impl<T: Expr<T> + Sub<Output = T>> Subtraction<T> {
    pub fn new(a: Box<dyn Expr<T>>, b: Box<dyn Expr<T>>) -> Box<Self> {
        Box::new(Self { a, b })
    }
}
