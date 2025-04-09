use super::define_list_function;
use nom::character::complete::char;
use nom::{
    IResult,
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{multispace0, multispace1},
    combinator::map,
    multi::separated_list0,
    sequence::{delimited, preceded},
};

use crate::expr::{
    builtins::string::Joined,
    {EvalTo, ListOf},
};

pub fn parse_expr(input: &str) -> IResult<&str, EvalTo<String>> {
    alt((parse_string, parse_sum))(input)
}

pub fn parse_string(input: &str) -> IResult<&str, EvalTo<String>> {
    map(
        delimited(char('"'), is_not("\""), char('"')),
        |s: &str| -> EvalTo<String> { Box::new(s.to_string()) },
    )(input)
}

fn parse_list(input: &str) -> IResult<&str, ListOf<String>> {
    delimited(
        preceded(tag("("), multispace0),
        separated_list0(multispace1, parse_expr),
        preceded(multispace0, tag(")")),
    )(input)
}

define_list_function!(parse_sum, tag("+"), Joined, String, parse_list);
