use crate::expr::{Expr, ListOf};

#[derive(Debug)]
pub struct And {
    pub items: ListOf<bool>,
}

impl Expr<bool> for And {
    fn eval(&self) -> bool {
        self.items.iter().all(|x| (*x).eval())
    }
}

impl And {
    pub fn new(items: ListOf<bool>) -> Box<Self> {
        Box::new(Self { items })
    }
}

#[derive(Debug)]
pub struct Or {
    pub items: ListOf<bool>,
}

impl Expr<bool> for Or {
    fn eval(&self) -> bool {
        self.items.iter().any(|x| (*x).eval())
    }
}

impl Or {
    pub fn new(items: ListOf<bool>) -> Box<Self> {
        Box::new(Self { items })
    }
}
