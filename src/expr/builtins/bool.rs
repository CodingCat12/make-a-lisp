use crate::expr::{Env, Expr};

pub struct And {
    items: Vec<Box<dyn Expr<bool>>>,
}

impl Expr<bool> for And {
    fn eval(&self, env: &Env) -> bool {
        self.items.iter().all(|x| x.eval(env))
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
    fn eval(&self, env: &Env) -> bool {
        self.items.iter().any(|x| x.eval(env))
    }
}

impl Or {
    pub fn new(items: Vec<Box<dyn Expr<bool>>>) -> Box<Self> {
        Box::new(Self { items })
    }
}
