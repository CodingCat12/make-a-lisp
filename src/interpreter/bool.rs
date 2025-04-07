use nom::{
    IResult,
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{char, multispace1},
    combinator::map,
    multi::separated_list0,
    sequence::{delimited, preceded},
};

use crate::{
    builtins::bool::*,
    expr::{EvalTo, ListOf},
};

pub fn parse_expr(input: &str) -> IResult<&str, EvalTo<bool>> {
    alt((parse_bool, parse_all, parse_any))(input)
}

fn parse_bool(input: &str) -> IResult<&str, EvalTo<bool>> {
    alt((
        map(tag("true"), |_| -> EvalTo<bool> { Box::new(true) }),
        map(tag("false"), |_| -> EvalTo<bool> { Box::new(false) }),
    ))(input)
}

fn parse_list(input: &str) -> IResult<&str, ListOf<bool>> {
    delimited(tag("("), separated_list0(multispace1, parse_expr), tag(")"))(input)
}

fn parse_all(input: &str) -> IResult<&str, EvalTo<bool>> {
    let (remaining, _) = preceded(tag("(&&"), multispace1)(input)?;
    let (remaining, expressions) = parse_list(remaining)?;
    let all = And::new(expressions);
    Ok((remaining, all))
}

fn parse_any(input: &str) -> IResult<&str, EvalTo<bool>> {
    let (remaining, _) = preceded(tag("(||"), multispace1)(input)?;
    let (remaining, expressions) = parse_list(remaining)?;
    let any = Or::new(expressions);
    Ok((remaining, any))
}
