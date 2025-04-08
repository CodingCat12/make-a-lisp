use super::define_list_function;
use crate::{
    builtins::math::{Average, Median, Product, Sum},
    expr::{EvalTo, ListOf},
};

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
    map(double, |n| -> EvalTo<f64> { Box::new(n) })(input)
}

pub fn parse_expr(input: &str) -> IResult<&str, EvalTo<f64>> {
    alt((
        parse_sum,
        parse_product,
        parse_average,
        parse_number,
        parse_median,
    ))(input)
}

fn parse_list(input: &str) -> IResult<&str, ListOf<f64>> {
    delimited(
        preceded(tag("("), multispace0),
        separated_list0(multispace1, parse_expr),
        preceded(multispace0, tag(")")),
    )(input)
}

fn parse_sum(input: &str) -> IResult<&str, EvalTo<f64>> {
    let (remaining, expressions) = delimited(
        preceded(
            preceded(tag("("), multispace0),
            preceded(tag("+"), multispace0),
        ),
        parse_list,
        preceded(multispace0, tag(")")),
    )(input)?;
    let result = Sum::new(expressions);
    Ok((remaining, result))
}
define_list_function!(parse_product, tag("*"), Product, f64);
define_list_function!(parse_average, tag("avg"), Average, f64);
define_list_function!(parse_median, tag("med"), Median, f64);
