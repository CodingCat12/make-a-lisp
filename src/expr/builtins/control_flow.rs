use crate::expr::Expr;

pub struct If<T> {
    check: Box<dyn Expr<bool>>,
    case: Box<dyn Expr<T>>,
}

impl<T> Expr<Option<T>> for If<T> {
    fn eval(&self) -> Option<T> {
        if self.check.eval() {
            Option::Some(self.case.eval())
        } else {
            Option::None
        }
    }
}

impl<T> If<T> {
    pub fn new(check: Box<dyn Expr<bool>>, case: Box<dyn Expr<T>>) -> Box<Self> {
        Box::new(Self { check, case })
    }
}

pub struct IfElse<T> {
    check: Box<dyn Expr<bool>>,
    case_one: Box<dyn Expr<T>>,
    case_two: Box<dyn Expr<T>>,
}

impl<T> Expr<T> for IfElse<T> {
    fn eval(&self) -> T {
        if self.check.eval() {
            self.case_one.eval()
        } else {
            self.case_two.eval()
        }
    }
}

impl<T> IfElse<T> {
    pub fn new(
        check: Box<dyn Expr<bool>>,
        case_true: Box<dyn Expr<T>>,
        case_false: Box<dyn Expr<T>>,
    ) -> Box<Self> {
        Box::new(Self {
            check,
            case_one: case_true,
            case_two: case_false,
        })
    }
}
