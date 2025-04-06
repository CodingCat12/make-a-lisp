use make_a_lisp::{
    builtins::{
        io::Print,
        math::{Average, Product, Sum},
    },
    expr::{EvalTo, Expr, ListOf},
};

use nom::character::complete::i32;
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

fn parse_int(input: &str) -> IResult<&str, EvalTo<i32>> {
    map(i32, |n| -> EvalTo<i32> { Box::new(n) })(input)
}

fn parse_int_expr(input: &str) -> IResult<&str, EvalTo<i32>> {
    alt((
        parse_int_sum,
        parse_int_product,
        parse_int_average,
        parse_int,
        /*        map(parse_float_expr, |n| -> EvalTo<i32> {
            Box::new(n.eval() as i32)
        }),*/
    ))(input)
}

fn parse_int_vector(input: &str) -> IResult<&str, ListOf<i32>> {
    delimited(
        tag("("),
        separated_list0(multispace1, parse_int_expr),
        tag(")"),
    )(input)
}

fn parse_int_sum(input: &str) -> IResult<&str, EvalTo<i32>> {
    let (remaining, _) = preceded(tag("(+"), multispace1)(input)?;
    let (remaining, expressions) = parse_int_vector(remaining)?;
    let sum = Sum::new(expressions);
    Ok((remaining, sum))
}

fn parse_int_product(input: &str) -> IResult<&str, EvalTo<i32>> {
    let (remaining, _) = preceded(tag("(*"), multispace1)(input)?;
    let (remaining, expressions) = parse_int_vector(remaining)?;
    let product = Product::new(expressions);
    Ok((remaining, product))
}

fn parse_int_average(input: &str) -> IResult<&str, EvalTo<i32>> {
    let (remaining, _) = preceded(tag("(avg"), multispace1)(input)?;
    let (remaining, expressions) = parse_int_vector(remaining)?;
    let average = Average::new(expressions);
    Ok((remaining, average))
}

fn parse_float(input: &str) -> IResult<&str, EvalTo<f64>> {
    map(double, |n| -> EvalTo<f64> { Box::new(n) })(input)
}

fn parse_float_expr(input: &str) -> IResult<&str, EvalTo<f64>> {
    alt((
        parse_float_sum,
        parse_float_product,
        parse_float_average,
        parse_float,
        /*        map(parse_int_expr, |n| -> EvalTo<f64> {
            Box::new(n.eval() as f64)
        }),*/
    ))(input)
}

fn parse_float_vector(input: &str) -> IResult<&str, ListOf<f64>> {
    delimited(
        tag("("),
        separated_list0(multispace1, parse_float_expr),
        tag(")"),
    )(input)
}

fn parse_float_sum(input: &str) -> IResult<&str, EvalTo<f64>> {
    let (remaining, _) = preceded(tag("(+"), multispace1)(input)?;
    let (remaining, expressions) = parse_float_vector(remaining)?;
    let sum = Sum::new(expressions);
    Ok((remaining, sum))
}

fn parse_float_product(input: &str) -> IResult<&str, EvalTo<f64>> {
    let (remaining, _) = preceded(tag("(*"), multispace1)(input)?;
    let (remaining, expressions) = parse_float_vector(remaining)?;
    let product = Product::new(expressions);
    Ok((remaining, product))
}

fn parse_float_average(input: &str) -> IResult<&str, EvalTo<f64>> {
    let (remaining, _) = preceded(tag("(avg"), multispace1)(input)?;
    let (remaining, expressions) = parse_float_vector(remaining)?;
    let average = Average::new(expressions);
    Ok((remaining, average))
}

fn parse_expr(input: &str) -> IResult<&str, EvalTo<()>> {
    alt((
        map(parse_int_expr, |o| -> EvalTo<()> {
            Box::new(Print(o.eval()))
        }),
        map(parse_float_expr, |o| -> EvalTo<()> {
            Box::new(Print(o.eval()))
        }),
    ))(input)
}

fn main() {
    let input = r#"(avg (2.0 (+ (1.0 2.0))))"#;
    match parse_expr(input) {
        Ok((_, result)) => result.eval(),
        Err(e) => eprintln!("Parsing Error: {:?}", e),
    }
}
