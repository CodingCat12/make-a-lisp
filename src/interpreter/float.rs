use super::{define_list_function, define_two_param_function};
use crate::expr::{
    builtins::math::{Average, Median, Product, Subtraction, Sum},
    {EvalTo, ListOf},
};
use nom::Parser;

use nom::number::complete::double;
use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::multispace0,
    character::complete::multispace1,
    combinator::map,
    multi::separated_list0,
    sequence::{delimited, preceded},
};

fn parse_number(input: &str) -> IResult<&str, EvalTo<f64>> {
    map(double, |n| -> EvalTo<f64> { Box::new(n) }).parse(input)
}

pub fn parse_expr(input: &str) -> IResult<&str, EvalTo<f64>> {
    alt((
        parse_sum,
        parse_product,
        parse_average,
        parse_number,
        parse_median,
        parse_subtraction,
    ))
    .parse(input)
}

fn parse_list(input: &str) -> IResult<&str, ListOf<f64>> {
    delimited(
        preceded(tag("("), multispace0),
        separated_list0(multispace1, parse_expr),
        preceded(multispace0, tag(")")),
    )
    .parse(input)
}

define_list_function!(parse_sum, tag("+"), Sum, f64, parse_list);
define_two_param_function!(parse_subtraction, tag("-"), Subtraction, f64, parse_expr);
define_list_function!(parse_product, tag("*"), Product, f64, parse_list);
define_list_function!(parse_average, tag("avg"), Average, f64, parse_list);
define_list_function!(parse_median, tag("med"), Median, f64, parse_list);
