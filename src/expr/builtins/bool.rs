use crate::expr::Expr;

#[derive(Debug)]
pub struct And {
    pub items: Vec<Box<dyn Expr<bool>>>,
}

impl Expr<bool> for And {
    fn eval(&self) -> bool {
        self.items.iter().all(|x| (*x).eval())
    }
}

impl And {
    pub fn new(items: Vec<Box<dyn Expr<bool>>>) -> Box<Self> {
        Box::new(Self { items })
    }
}

#[derive(Debug)]
pub struct Or {
    pub items: Vec<Box<dyn Expr<bool>>>,
}

impl Expr<bool> for Or {
    fn eval(&self) -> bool {
        self.items.iter().any(|x| (*x).eval())
    }
}

impl Or {
    pub fn new(items: Vec<Box<dyn Expr<bool>>>) -> Box<Self> {
        Box::new(Self { items })
    }
}
