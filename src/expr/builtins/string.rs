use crate::expr::{Env, Expr};

pub struct Joined {
    items: Vec<Box<dyn Expr<String>>>,
}

impl Expr<String> for Joined {
    fn eval(&self, env: &Env) -> String {
        let evalled: Vec<String> = self.items.iter().map(|s| s.eval(env)).collect();
        evalled.join("")
    }
}

impl Joined {
    pub fn new(items: Vec<Box<dyn Expr<String>>>) -> Box<Self> {
        Box::new(Self { items })
    }
}
