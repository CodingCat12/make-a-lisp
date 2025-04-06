use make_a_lisp::{
    builtins::math::{Average, Product, Sum},
    expr::{EvalTo, ListOf},
};
use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, multispace1},
    combinator::map,
    multi::separated_list0,
    sequence::{delimited, preceded},
};

fn parse_number(input: &str) -> IResult<&str, EvalTo<i32>> {
    map(digit1, |s: &str| -> EvalTo<i32> {
        Box::new(s.parse::<i32>().unwrap())
    })(input)
}

fn parse_expr(input: &str) -> IResult<&str, EvalTo<i32>> {
    alt((parse_sum, parse_product, parse_average, parse_number))(input)
}

fn parse_vector(input: &str) -> IResult<&str, ListOf<i32>> {
    delimited(tag("("), separated_list0(multispace1, parse_expr), tag(")"))(input)
}

fn parse_sum(input: &str) -> IResult<&str, EvalTo<i32>> {
    let (remaining, _) = preceded(tag("(+"), multispace1)(input)?;
    let (remaining, expressions) = parse_vector(remaining)?;
    let sum = Sum::new(expressions);
    Ok((remaining, sum))
}

fn parse_product(input: &str) -> IResult<&str, EvalTo<i32>> {
    let (remaining, _) = preceded(tag("(*"), multispace1)(input)?;
    let (remaining, expressions) = parse_vector(remaining)?;
    let product = Product::new(expressions);
    Ok((remaining, product))
}

fn parse_average(input: &str) -> IResult<&str, EvalTo<i32>> {
    let (remaining, _) = preceded(tag("(avg"), multispace1)(input)?;
    let (remaining, expressions) = parse_vector(remaining)?;
    let average = Average::new(expressions);
    Ok((remaining, average))
}

fn main() {
    let input = "(avg (2 4))";
    match parse_expr(input) {
        Ok((_, result)) => {
            println!("Parsed result: {:?}", result.eval());
        }
        Err(e) => eprintln!("Parsing Error: {:?}", e),
    }
}
