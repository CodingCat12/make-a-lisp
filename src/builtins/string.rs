use crate::expr::{Expr, ListOf};

#[derive(Debug)]
pub struct Joined {
    items: ListOf<String>,
}

impl Expr<String> for Joined {
    fn eval(&self) -> String {
        let evalled: Vec<String> = self.items.iter().map(|s| (*s).eval()).collect();
        evalled.join("")
    }
}

impl Joined {
    pub fn new(items: ListOf<String>) -> Box<Self> {
        Box::new(Self { items })
    }
}
