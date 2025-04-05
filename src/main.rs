use make_a_lisp::expr::{Expr, Overwrite};

use make_a_lisp::builtins::control_flow::{If, IfElse};
use make_a_lisp::builtins::io::PrintLine;
use make_a_lisp::builtins::math::{Product, Sum};
use make_a_lisp::builtins::random::RandomBool;

fn main() {
    let expr = IfElse::new(
        RandomBool::new(),
        Product::new(vec![
            Sum::new(vec![Box::new(1.0), Box::new(1.0)]),
            Box::new(4.0),
        ]),
        Box::new(Overwrite {
            function: Box::new(If {
                check: Box::new(true),
                case: Box::new(PrintLine("very useful text")),
            }),
            value: Box::new(5.0),
        }),
    );

    let print = PrintLine(expr.eval());
    print.eval();
}
