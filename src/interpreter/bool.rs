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
    builtins::string::Joined,
    expr::{EvalTo, ListOf},
};

pub fn parse_expr(input: &str) -> IResult<&str, EvalTo<bool>> {
    alt((

    ))(input)
}

fn parse_bool(input: &str) -> IResult<&str EvalTo<bool>> {

}
