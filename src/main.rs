use rand::random;

fn main() {
    let expr = IfElse {
        check: Box::new(RandomBool),
        case_one: Box::new(3),
        case_two: Box::new(4),
    };

    let print = Print(expr.eval());
    print.eval();
}

trait Expr<T: Expr<T>>: std::fmt::Debug {
    fn eval(&self) -> T;
}

#[allow(type_alias_bounds)]
type EvalTo<T: Expr<T>> = Box<dyn Expr<T>>;

impl<T: Clone + std::fmt::Debug> Expr<T> for T {
    fn eval(&self) -> Self {
        self.clone()
    }
}

#[derive(Debug)]
struct If<T: Expr<T>> {
    check: EvalTo<bool>,
    case: EvalTo<T>,
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
struct IfElse<T: Expr<T>> {
    check: Box<dyn Expr<bool>>,
    case_one: EvalTo<T>,
    case_two: EvalTo<T>,
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

#[derive(Debug)]
struct Print<T: Expr<T>>(T);

impl<T: Expr<T>> Expr<()> for Print<T> {
    fn eval(&self) {
        println!("{:#?}", self.0);
    }
}

#[derive(Debug)]
struct RandomBool;

impl Expr<bool> for RandomBool {
    fn eval(&self) -> bool {
        random::<bool>()
    }
}

#[derive(Debug)]
struct Sum {
    items: Vec<EvalTo<i32>>,
}

impl Expr<i32> for Sum {
    fn eval(&self) -> i32 {
        self.items.iter().map(|x| (*x).eval()).sum()
    }
}

#[derive(Debug)]
struct Product {
    items: Vec<EvalTo<i32>>,
}

impl Expr<i32> for Product {
    fn eval(&self) -> i32 {
        self.items.iter().map(|x| (*x).eval()).product()
    }
}

#[derive(Debug)]
struct And {
    items: Vec<EvalTo<bool>>,
}

impl Expr<bool> for And {
    fn eval(&self) -> bool {
        self.items.iter().all(|x| (*x).eval())
    }
}

#[derive(Debug)]
struct Or {
    items: Vec<EvalTo<bool>>,
}

impl Expr<bool> for Or {
    fn eval(&self) -> bool {
        self.items.iter().any(|x| (*x).eval())
    }
}

#[derive(Debug)]
struct Overwrite<D: Expr<D>, T: Expr<T>> {
    function: Box<EvalTo<D>>,
    value: T,
}

impl<D: Expr<D> + Copy, T: Expr<T> + Copy> Expr<T> for Overwrite<D, T> {
    fn eval(&self) -> T {
        self.function.eval();
        self.value
    }
}
