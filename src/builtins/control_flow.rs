use crate::expr::{EvalTo, Expr};

#[derive(Debug)]
pub struct If<T: Expr<T> + 'static> {
    pub check: EvalTo<bool>,
    pub case: EvalTo<T>,
}

impl<T: Expr<T> + Copy> Expr<Option<T>> for If<T> {
    fn eval(&self) -> Option<T> {
        if self.check.eval() {
            Option::Some(self.case.eval())
        } else {
            Option::None
        }
    }
}

#[derive(Debug)]
pub struct IfElse<T: Expr<T> + 'static> {
    pub check: EvalTo<bool>,
    pub case_one: EvalTo<T>,
    pub case_two: EvalTo<T>,
}

impl<T: Expr<T> + Copy> Expr<T> for IfElse<T> {
    fn eval(&self) -> T {
        if self.check.eval() {
            self.case_one.eval()
        } else {
            self.case_two.eval()
        }
    }
}
