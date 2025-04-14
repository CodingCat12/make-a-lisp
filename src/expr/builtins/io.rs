use std::fmt::Debug;

use crate::expr::Expr;

#[derive(Debug)]
pub struct PrintLine<T>(pub T);

impl<T: Debug> Expr<()> for PrintLine<T> {
    fn eval(&self) {
        println!("{:#?}", self.0);
    }
}

#[derive(Debug)]
pub struct Print<T>(pub T);

impl<T: Debug> Expr<()> for Print<T> {
    fn eval(&self) {
        print!("{:#?}", self.0);
    }
}
