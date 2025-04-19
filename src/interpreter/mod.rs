pub mod bool;
pub mod float;
pub mod int;
pub mod string;
#[cfg(test)]
mod tests;

use nom::Parser;
use nom::{IResult, branch::alt, combinator::map};

pub fn parse_expr(input: &str) -> IResult<&str, Box<dyn std::any::Any>> {
    alt((
        map(int::parse_expr, |o| -> Box<dyn std::any::Any> {
            Box::new(o.eval())
        }),
        map(float::parse_expr, |o| -> Box<dyn std::any::Any> {
            Box::new(o.eval())
        }),
        map(string::parse_expr, |o| -> Box<dyn std::any::Any> {
            Box::new(o.eval())
        }),
        map(bool::parse_expr, |o| -> Box<dyn std::any::Any> {
            Box::new(o.eval())
        }),
    ))
    .parse(input)
}

pub macro define_list_function($name:ident, $op:expr, $constructor:ident, $ty:ty, $list_parser:ident) {
    use crate::expr::Expr;
    use nom::character::complete::multispace0;
    use nom::{
        IResult,
        bytes::complete::tag,
        sequence::{delimited, preceded},
    };

    fn $name(input: &str) -> IResult<&str, Box<dyn Expr<$ty>>> {
        let (remaining, expressions) = delimited(
            preceded(preceded(tag("("), multispace0), preceded($op, multispace0)),
            $list_parser,
            preceded(multispace0, tag(")")),
        )
        .parse(input)?;

        let result = $constructor::new(expressions);
        Ok((remaining, result))
    }
}

pub macro define_two_param_function($name:ident, $op:expr, $constructor:ident, $ty:ty, $expr_parser:ident) {
    use crate::expr::Expr;
    use nom::character::complete::multispace0;
    use nom::{IResult, bytes::complete::tag, sequence::preceded};

    fn $name(input: &str) -> IResult<&str, Box<dyn Expr<$ty>>> {
        let (remaining, _) =
            preceded(preceded(tag("("), multispace0), preceded($op, multispace0)).parse(input)?;
        let (remaining, a) = $expr_parser.parse(remaining)?;
        let (remaining, _) = multispace0.parse(remaining)?;
        let (remaining, b) = $expr_parser.parse(remaining)?;

        let result = $constructor::new(a, b);
        Ok((remaining, result))
    }
}
