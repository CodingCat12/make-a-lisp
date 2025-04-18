use std::fmt::Debug;

use crate::expr::{Env, Expr};

pub struct PrintLine<T>(T);

impl<T: Debug> Expr<()> for PrintLine<T> {
    fn eval(&self, _env: &Env) {
        println!("{:#?}", self.0);
    }
}

impl<T> PrintLine<T> {
    pub fn new(v: T) -> Box<Self> {
        Box::new(Self(v))
    }
}

pub struct Print<T>(T);

impl<T: Debug> Expr<()> for Print<T> {
    fn eval(&self, _env: &Env) {
        print!("{:#?}", self.0);
    }
}

impl<T> Print<T> {
    pub fn new(v: T) -> Box<Self> {
        Box::new(Self(v))
    }
}
