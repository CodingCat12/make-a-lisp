use crate::expr::Expr;

#[derive(Debug)]
pub struct PrintLine<T: Expr<T>>(pub T);

impl<T: Expr<T>> Expr<()> for PrintLine<T> {
    fn eval(&self) {
        println!("{:#?}", self.0);
    }
}

#[derive(Debug)]
pub struct Print<T: Expr<T>>(pub T);

impl<T: Expr<T>> Expr<()> for Print<T> {
    fn eval(&self) {
        print!("{:#?}", self.0);
    }
}
