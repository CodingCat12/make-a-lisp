use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric0, char, multispace1},
    combinator::map,
    multi::separated_list0,
    sequence::{delimited, preceded},
};

use crate::{
    builtins::string::Joined,
    expr::{EvalTo, ListOf},
};

pub fn parse_expr(input: &str) -> IResult<&str, EvalTo<String>> {
    alt((parse_string, parse_sum))(input)
}

pub fn parse_string(input: &str) -> IResult<&str, EvalTo<String>> {
    map(
        delimited(char('"'), alphanumeric0, char('"')),
        |s: &str| -> EvalTo<String> { Box::new(s.to_string()) },
    )(input)
}

fn parse_list(input: &str) -> IResult<&str, ListOf<String>> {
    delimited(tag("("), separated_list0(multispace1, parse_expr), tag(")"))(input)
}

fn parse_sum(input: &str) -> IResult<&str, EvalTo<String>> {
    let (remaining, _) = preceded(tag("(+"), multispace1)(input)?;
    let (remaining, expressions) = parse_list(remaining)?;
    let joined = Joined::new(expressions);
    Ok((remaining, joined))
}
