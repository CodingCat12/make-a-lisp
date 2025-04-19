use super::{define_list_function, define_two_param_function};
use crate::expr::{
    Expr, Variable,
    builtins::math::{Average, Median, Product, Sub, Sum},
};
use nom::{Parser, character::complete::alphanumeric1};

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

fn parse_number(input: &str) -> IResult<&str, Box<dyn Expr<f64>>> {
    map(double, |n| -> Box<dyn Expr<f64>> { Box::new(n) }).parse(input)
}

pub fn parse_expr(input: &str) -> IResult<&str, Box<dyn Expr<f64>>> {
    alt((
        parse_sum,
        parse_product,
        parse_average,
        parse_number,
        parse_median,
        parse_subtraction,
        parse_var,
    ))
    .parse(input)
}

fn parse_list(input: &str) -> IResult<&str, Vec<Box<dyn Expr<f64>>>> {
    delimited(
        preceded(tag("("), multispace0),
        separated_list0(multispace1, parse_expr),
        preceded(multispace0, tag(")")),
    )
    .parse(input)
}

fn parse_var(input: &str) -> IResult<&str, Box<dyn Expr<f64>>> {
    let (remaining, chars) = delimited(multispace0, alphanumeric1, multispace0).parse(input)?;
    Ok((remaining, Variable::new(chars)))
}

define_list_function!(parse_sum, tag("+"), Sum, f64, parse_list);
define_two_param_function!(parse_subtraction, tag("-"), Sub, f64, parse_expr);
define_list_function!(parse_product, tag("*"), Product, f64, parse_list);
define_list_function!(parse_average, tag("avg"), Average, f64, parse_list);
define_list_function!(parse_median, tag("med"), Median, f64, parse_list);
