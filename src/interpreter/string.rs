use super::define_list_function;
use nom::Parser;
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

use crate::expr::{Expr, builtins::string::Joined};

pub fn parse_expr(input: &str) -> IResult<&str, Box<dyn Expr<String>>> {
    alt((parse_string, parse_sum)).parse(input)
}

pub fn parse_string(input: &str) -> IResult<&str, Box<dyn Expr<String>>> {
    map(
        delimited(char('"'), is_not("\""), char('"')),
        |s: &str| -> Box<dyn Expr<String>> { Box::new(s.to_string()) },
    )
    .parse(input)
}

fn parse_list(input: &str) -> IResult<&str, Vec<Box<dyn Expr<String>>>> {
    delimited(
        preceded(tag("("), multispace0),
        separated_list0(multispace1, parse_expr),
        preceded(multispace0, tag(")")),
    )
    .parse(input)
}

define_list_function!(parse_sum, tag("+"), Joined, String, parse_list);
