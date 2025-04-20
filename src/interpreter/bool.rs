use crate::expr::Variable;
use crate::interpreter::define_list_function;
use nom::Parser;
use nom::character::complete::alphanumeric1;
use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace0, multispace1},
    combinator::map,
    multi::separated_list0,
    sequence::{delimited, preceded},
};

use crate::expr::{Expr, builtins::bool::*};

pub fn parse_expr(input: &str) -> IResult<&str, Box<dyn Expr<bool>>> {
    alt((parse_bool, parse_all, parse_any, parse_var)).parse(input)
}

fn parse_bool(input: &str) -> IResult<&str, Box<dyn Expr<bool>>> {
    alt((
        map(tag("true"), |_| -> Box<dyn Expr<bool>> { Box::new(true) }),
        map(tag("false"), |_| -> Box<dyn Expr<bool>> { Box::new(false) }),
    ))
    .parse(input)
}

fn parse_list(input: &str) -> IResult<&str, Vec<Box<dyn Expr<bool>>>> {
    delimited(
        preceded(tag("("), multispace0),
        separated_list0(multispace1, parse_expr),
        preceded(multispace0, tag(")")),
    )
    .parse(input)
}

fn parse_var(input: &str) -> IResult<&str, Box<dyn Expr<bool>>> {
    let (remaining, chars) = alphanumeric1.parse(input)?;
    println!("{}", chars);
    Ok((remaining, Variable::new(chars)))
}

define_list_function!(parse_all, tag("&&"), And, bool, parse_list);
define_list_function!(parse_any, tag("||"), Or, bool, parse_list);
