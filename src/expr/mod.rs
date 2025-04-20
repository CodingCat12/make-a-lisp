use std::{any::Any, collections::HashMap, rc::Rc};

pub mod builtins;
#[cfg(test)]
mod tests;

pub trait Expr<T> {
    fn eval(&self, env: &Env) -> T;
}

macro expr_impl {
    // stolen from std::ops::Add (https://doc.rust-lang.org/src/core/ops/arith.rs.html#96)
    ($($t:ty)*) => ($(
        impl Expr<Self> for $t {
            fn eval(&self, _env: &Env) -> Self {
                self.clone()
            }
        }

        impl Expr<Box<dyn Any>> for $t {
            fn eval(&self, _env: &Env) -> Box<dyn Any> {
                Box::new(self.clone())
            }
        }
    )*)
}

expr_impl! { i32 bool f64 String }

pub struct Env {
    vars: HashMap<String, Rc<dyn Expr<Box<dyn Any>>>>,
}

impl Default for Env {
    fn default() -> Self {
        let mut vars: HashMap<String, Rc<dyn Expr<Box<dyn Any>>>> = HashMap::new();
        vars.insert(String::from("PI"), Rc::new(std::f64::consts::PI));
        vars.insert(String::from("E"), Rc::new(std::f64::consts::E));
        vars.insert(
            String::from("HELLO"),
            Rc::new(String::from("Hello, world!")),
        );
        Self { vars }
    }
}

pub struct Variable {
    name: String,
}

impl<T: Any> Expr<T> for Variable {
    fn eval(&self, env: &Env) -> T {
        let val = env
            .vars
            .get(&self.name)
            .unwrap_or_else(|| panic!("error: variable {} not found", self.name))
            .eval(env);
        *val.downcast::<T>()
            .unwrap_or_else(|_| panic!("error: type mismatch"))
    }
}

impl Variable {
    pub fn new(name: &str) -> Box<Variable> {
        Box::new(Self {
            name: String::from(name),
        })
    }
}

pub struct Let<V, T> {
    name: String,
    value: Box<dyn Expr<V>>,
    body: Box<dyn Expr<T>>,
}

impl<V, T: 'static> Let<V, T> {
    pub fn new(name: &str, value: Box<dyn Expr<V>>, body: Box<dyn Expr<T>>) -> Box<Self> {
        Box::new(Self {
            name: name.to_string(),
            value,
            body,
        })
    }
}

impl<V: Expr<Box<dyn Any>> + 'static, T> Expr<T> for Let<V, T> {
    fn eval(&self, env: &Env) -> T {
        let mut vars = env.vars.clone();
        vars.insert(self.name.clone(), Rc::new(self.value.eval(env)));
        self.body.eval(&Env { vars })
    }
}
