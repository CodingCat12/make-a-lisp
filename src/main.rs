mod builtins;
mod expr;

use crate::expr::{Expr, Overwrite};

use crate::builtins::control_flow::{If, IfElse};
use crate::builtins::io::PrintLine;
use crate::builtins::math::{Product, Sum};
use crate::builtins::random::RandomBool;

fn main() {
    let expr = &IfElse {
        check: &RandomBool,
        case_one: &Product {
            items: &[
                &Sum {
                    items: &[&1.0, &1.0],
                },
                &4.0,
            ],
        },
        case_two: &Overwrite {
            function: &If {
                check: &true,
                case: &PrintLine("very useful text"),
            },
            value: 5.0,
        },
    };

    let print = PrintLine(expr.eval());
    print.eval()
}
