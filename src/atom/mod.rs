use nom::{branch::alt, character::complete::multispace0, sequence::delimited, IResult};
use std::fmt;

mod comment;
mod float;
mod integer;
mod string;
mod symbol;

use comment::parse as parse_comment;
use float::parse as parse_float;
use integer::parse as parse_integer;
use string::parse as parse_string;
use symbol::parse as parse_symbol;
#[derive(Debug, PartialEq, Clone)]
pub enum AtomType {
    Integer(i64),
    Float(f64),
    Symbol(String),
    String(String),
    Null,
}

impl fmt::Display for AtomType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn parse(input: &str) -> IResult<&str, AtomType> {
    delimited(
        multispace0,
        alt((
            parse_comment,
            parse_float,
            parse_integer,
            parse_string,
            parse_symbol,
        )),
        multispace0,
    )(input)
}
