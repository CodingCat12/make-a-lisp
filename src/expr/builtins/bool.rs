use crate::expr::Expr;

pub struct And {
    items: Vec<Box<dyn Expr<bool>>>,
}

impl Expr<bool> for And {
    fn eval(&self) -> bool {
        self.items.iter().all(|x| x.eval())
    }
}

impl And {
    pub fn new(items: Vec<Box<dyn Expr<bool>>>) -> Box<Self> {
        Box::new(Self { items })
    }
}

pub struct Or {
    items: Vec<Box<dyn Expr<bool>>>,
}

impl Expr<bool> for Or {
    fn eval(&self) -> bool {
        self.items.iter().any(|x| x.eval())
    }
}

impl Or {
    pub fn new(items: Vec<Box<dyn Expr<bool>>>) -> Box<Self> {
        Box::new(Self { items })
    }
}
