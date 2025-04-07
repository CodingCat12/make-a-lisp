pub mod bool;
pub mod float;
pub mod int;
pub mod string;

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
