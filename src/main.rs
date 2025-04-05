pub mod builtins;
pub mod expr;

use crate::expr::{Expr, Overwrite};

use crate::builtins::control_flow::{If, IfElse};
use crate::builtins::io::PrintLine;
use crate::builtins::math::{Product, Sum};
use crate::builtins::random::RandomBool;

fn main() {
    let expr = Box::new(IfElse {
        check: Box::new(RandomBool),
        case_one: Box::new(Product {
            items: vec![
                Box::new(Sum {
                    items: vec![Box::new(1.0), Box::new(1.0)],
                }),
                Box::new(4.0),
            ],
        }),
        case_two: Box::new(Overwrite {
            function: Box::new(If {
                check: Box::new(true),
                case: Box::new(PrintLine("very useful text")),
            }),
            value: Box::new(5.0),
        }),
    });

    let print = PrintLine(expr.eval());
    print.eval();

    let let_statement = |x: i32| Sum {
        items: vec![Box::new(x), Box::new(5)],
    };
    let expr = let_statement(5);

    let print = PrintLine(expr.eval());
    print.eval();
}
