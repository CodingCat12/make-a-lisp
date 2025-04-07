use crate::{
    builtins::math::{Average, Median, Product, Sum},
    expr::{EvalTo, ListOf},
};

use nom::number::complete::double;
use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
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

fn parse_float_vector(input: &str) -> IResult<&str, ListOf<f64>> {
    delimited(tag("("), separated_list0(multispace1, parse_expr), tag(")"))(input)
}

fn parse_sum(input: &str) -> IResult<&str, EvalTo<f64>> {
    let (remaining, _) = preceded(tag("(+"), multispace1)(input)?;
    let (remaining, expressions) = parse_float_vector(remaining)?;
    let sum = Sum::new(expressions);
    Ok((remaining, sum))
}

fn parse_product(input: &str) -> IResult<&str, EvalTo<f64>> {
    let (remaining, _) = preceded(tag("(*"), multispace1)(input)?;
    let (remaining, expressions) = parse_float_vector(remaining)?;
    let product = Product::new(expressions);
    Ok((remaining, product))
}

fn parse_average(input: &str) -> IResult<&str, EvalTo<f64>> {
    let (remaining, _) = preceded(tag("(avg"), multispace1)(input)?;
    let (remaining, expressions) = parse_float_vector(remaining)?;
    let average = Average::new(expressions);
    Ok((remaining, average))
}

fn parse_median(input: &str) -> IResult<&str, EvalTo<f64>> {
    let (remaining, _) = preceded(tag("(med"), multispace1)(input)?;
    let (remaining, expressions) = parse_float_vector(remaining)?;
    let average = Median::new(expressions);
    Ok((remaining, average))
}
