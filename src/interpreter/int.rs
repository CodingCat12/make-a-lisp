use super::{define_list_function, define_two_param_function};
use crate::expr::{
    builtins::math::{Average, Median, Product, Subtraction, Sum},
    {EvalTo, ListOf},
};
use nom::Parser;

use nom::character::complete::{i32, multispace0};
use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::multispace1,
    combinator::map,
    multi::separated_list0,
    sequence::{delimited, preceded},
};

fn parse_int(input: &str) -> IResult<&str, EvalTo<i32>> {
    map(i32, |n| -> EvalTo<i32> { Box::new(n) }).parse(input)
}

pub fn parse_expr(input: &str) -> IResult<&str, EvalTo<i32>> {
    alt((
        parse_int,
        parse_sum,
        parse_product,
        parse_average,
        parse_median,
        parse_subtraction,
    ))
    .parse(input)
}

fn parse_list(input: &str) -> IResult<&str, ListOf<i32>> {
    delimited(
        preceded(tag("("), multispace0),
        separated_list0(multispace1, parse_expr),
        preceded(multispace0, tag(")")),
    )
    .parse(input)
}

define_two_param_function!(parse_subtraction, tag("-"), Subtraction, i32, parse_expr);
define_list_function!(parse_sum, tag("+"), Sum, i32, parse_list);
define_list_function!(parse_product, tag("*"), Product, i32, parse_list);
define_list_function!(parse_average, tag("avg"), Average, i32, parse_list);
define_list_function!(parse_median, tag("med"), Median, i32, parse_list);
