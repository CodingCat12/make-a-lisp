use crate::expr::Expr;
use num::FromPrimitive;
use std::fmt::Debug;
use std::{iter, ops};

#[derive(Debug)]
pub struct Sum<T> {
    items: Vec<Box<dyn Expr<T>>>,
}

impl<T: iter::Sum + Debug> Expr<T> for Sum<T> {
    fn eval(&self) -> T {
        self.items.iter().map(|x| x.eval()).sum()
    }
}

impl<T> Sum<T> {
    pub fn new(items: Vec<Box<dyn Expr<T>>>) -> Box<Self> {
        Box::new(Self { items })
    }
}

#[derive(Debug)]
pub struct Product<T> {
    items: Vec<Box<dyn Expr<T>>>,
}

impl<T: iter::Product + Debug> Expr<T> for Product<T> {
    fn eval(&self) -> T {
        self.items.iter().map(|x| x.eval()).product()
    }
}

impl<T> Product<T> {
    pub fn new(items: Vec<Box<dyn Expr<T>>>) -> Box<Self> {
        Box::new(Self { items })
    }
}

#[derive(Debug)]
pub struct Average<T> {
    items: Vec<Box<dyn Expr<T>>>,
}

impl<T: iter::Sum + FromPrimitive + ops::Div<Output = T> + Debug> Expr<T> for Average<T> {
    fn eval(&self) -> T {
        let len = T::from_usize(self.items.len()).unwrap();
        let total: T = self.items.iter().map(|x| x.eval()).sum();
        total / len
    }
}

impl<T: iter::Sum + Debug> Average<T> {
    pub fn new(items: Vec<Box<dyn Expr<T>>>) -> Box<Self> {
        Box::new(Self { items })
    }
}

#[derive(Debug)]
pub struct Median<T> {
    items: Vec<Box<dyn Expr<T>>>,
}

impl<T: PartialOrd + Debug + Clone> Expr<T> for Median<T> {
    fn eval(&self) -> T {
        let mut sorted: Vec<T> = self.items.iter().map(|x| x.eval()).collect();
        sorted.sort_unstable_by(|x, y| x.partial_cmp(y).unwrap());
        sorted[self.items.len() / 2].clone()
    }
}

impl<T: PartialOrd + Clone> Median<T> {
    pub fn new(items: Vec<Box<dyn Expr<T>>>) -> Box<Self> {
        Box::new(Self { items })
    }
}

#[derive(Debug)]
pub struct Sub<T> {
    a: Box<dyn Expr<T>>,
    b: Box<dyn Expr<T>>,
}

impl<T: ops::Sub<Output = T> + Debug> Expr<T> for Sub<T> {
    fn eval(&self) -> T {
        self.a.eval() - self.b.eval()
    }
}

impl<T: ops::Sub<Output = T> + Debug> Sub<T> {
    pub fn new(a: Box<dyn Expr<T>>, b: Box<dyn Expr<T>>) -> Box<Self> {
        Box::new(Self { a, b })
    }
}
