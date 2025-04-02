use basicexpr_derive::BasicExpr;
use rand::random;

fn main() {
    let expr = Sum {
        items: vec![Box::new(Int(5)), Box::new(Int(6)), Box::new(Int(10))],
    };

    println!("{:#?}", expr.eval())
}

trait Expr<T: Expr<T>>: std::fmt::Debug {
    fn eval(&self) -> T;
}

#[derive(Debug, Clone, Copy, BasicExpr)]
struct Int(i32);

#[derive(Debug, Clone, Copy, BasicExpr)]
struct Float(f64);

#[derive(Debug, Clone, Copy, BasicExpr)]
struct Bool(bool);

#[derive(Debug)]
struct RandomBool;

impl Expr<Bool> for RandomBool {
    fn eval(&self) -> Bool {
        Bool(random::<bool>())
    }
}

#[derive(Debug)]
struct Sum {
    items: Vec<Box<dyn Expr<Int>>>,
}

impl Expr<Int> for Sum {
    fn eval(&self) -> Int {
        let total = self.items.iter().map(|x| x.eval().0).sum();
        Int(total)
    }
}

#[derive(Debug)]
struct Product {
    items: Vec<Box<dyn Expr<Int>>>,
}

impl Expr<Int> for Product {
    fn eval(&self) -> Int {
        let total = self.items.iter().map(|x| x.eval().0).product();
        Int(total)
    }
}

#[derive(Debug)]
struct And {
    items: Vec<Box<dyn Expr<Bool>>>,
}

impl Expr<Bool> for And {
    fn eval(&self) -> Bool {
        let result = self.items.iter().all(|x| x.eval().0);
        Bool(result)
    }
}

#[derive(Debug)]
struct Or {
    items: Vec<Box<dyn Expr<Bool>>>,
}

impl Expr<Bool> for Or {
    fn eval(&self) -> Bool {
        let result = self.items.iter().any(|x| x.eval().0);
        Bool(result)
    }
}
