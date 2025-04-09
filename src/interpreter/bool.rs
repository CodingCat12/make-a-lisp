use crate::interpreter::define_list_function;
use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace0, multispace1},
    combinator::map,
    multi::separated_list0,
    sequence::{delimited, preceded},
};

use crate::expr::{
    builtins::bool::*,
    {EvalTo, ListOf},
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
    delimited(
        preceded(tag("("), multispace0),
        separated_list0(multispace1, parse_expr),
        preceded(multispace0, tag(")")),
    )(input)
}

define_list_function!(parse_all, tag("&&"), And, bool);
define_list_function!(parse_any, tag("||"), Or, bool);
