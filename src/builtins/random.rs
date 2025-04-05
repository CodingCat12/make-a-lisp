use crate::expr::Expr;
use rand;

#[derive(Debug)]
pub struct RandomBool;

impl Expr<bool> for RandomBool {
    fn eval(&self) -> bool {
        rand::random::<bool>()
    }
}

impl RandomBool {
    pub fn new() -> Box<Self> {
        Box::new(Self)
    }
}
