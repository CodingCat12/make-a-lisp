pub mod bool;
pub mod float;
pub mod int;
pub mod string;
#[cfg(test)]
mod tests;

use nom::{IResult, branch::alt, combinator::map};

pub fn parse_expr(input: &str) -> IResult<&str, Box<dyn std::fmt::Debug>> {
    alt((
        map(int::parse_expr, |o| -> Box<dyn std::fmt::Debug> {
            Box::new(o.eval())
        }),
        map(float::parse_expr, |o| -> Box<dyn std::fmt::Debug> {
            Box::new(o.eval())
        }),
        map(string::parse_expr, |o| -> Box<dyn std::fmt::Debug> {
            Box::new(o.eval())
        }),
        map(bool::parse_expr, |o| -> Box<dyn std::fmt::Debug> {
            Box::new(o.eval())
        }),
    ))(input)
}

#[macro_export]
macro_rules! define_list_function {
    ($name:ident, $op:expr, $constructor:ident, $ty:ty) => {
        fn $name(input: &str) -> IResult<&str, EvalTo<$ty>> {
            let (remaining, expressions) = delimited(
                preceded(preceded(tag("("), multispace0), preceded($op, multispace0)),
                parse_list,
                preceded(multispace0, tag(")")),
            )(input)?;

            let result = $constructor::new(expressions);
            Ok((remaining, result))
        }
    };
}
pub use define_list_function;

#[macro_export]
macro_rules! define_two_param_function {
    ($name:ident, $op:expr, $constructor:ident, $ty:ty) => {
        fn $name(input: &str) -> IResult<&str, EvalTo<$ty>> {
            let (remaining, _) =
                preceded(preceded(tag("("), multispace0), preceded($op, multispace0))(input)?;
            let (remaining, a) = parse_expr(remaining)?;
            let (remaining, _) = multispace0(remaining)?;
            let (remaining, b) = parse_expr(remaining)?;

            let result = $constructor::new(a, b);
            Ok((remaining, result))
        }
    };
}
pub use define_two_param_function;
