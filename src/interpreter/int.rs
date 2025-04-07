use crate::{
    builtins::math::{Average, Product, Sum},
    expr::{EvalTo, ListOf},
};

use nom::character::complete::i32;
use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::multispace1,
    combinator::map,
    multi::separated_list0,
    sequence::{delimited, preceded},
};

fn parse_num(input: &str) -> IResult<&str, EvalTo<i32>> {
    map(i32, |n| -> EvalTo<i32> { Box::new(n) })(input)
}

pub fn parse_expr(input: &str) -> IResult<&str, EvalTo<i32>> {
    alt((
        parse_sum,
        parse_product,
        parse_average,
        parse_num,
        /*        map(parse_float_expr, |n| -> EvalTo<i32> {
            Box::new(n.eval() as i32)
        }),*/
    ))(input)
}

fn parse_int_vector(input: &str) -> IResult<&str, ListOf<i32>> {
    delimited(tag("("), separated_list0(multispace1, parse_expr), tag(")"))(input)
}

fn parse_sum(input: &str) -> IResult<&str, EvalTo<i32>> {
    let (remaining, _) = preceded(tag("(+"), multispace1)(input)?;
    let (remaining, expressions) = parse_int_vector(remaining)?;
    let sum = Sum::new(expressions);
    Ok((remaining, sum))
}

fn parse_product(input: &str) -> IResult<&str, EvalTo<i32>> {
    let (remaining, _) = preceded(tag("(*"), multispace1)(input)?;
    let (remaining, expressions) = parse_int_vector(remaining)?;
    let product = Product::new(expressions);
    Ok((remaining, product))
}

fn parse_average(input: &str) -> IResult<&str, EvalTo<i32>> {
    let (remaining, _) = preceded(tag("(avg"), multispace1)(input)?;
    let (remaining, expressions) = parse_int_vector(remaining)?;
    let average = Average::new(expressions);
    Ok((remaining, average))
}
