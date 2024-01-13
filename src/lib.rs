pub mod atom;

use atom::{parse as parse_atom, AtomType};
use nom::{
    branch::alt,
    character::complete::{char, line_ending, multispace0},
    combinator::{all_consuming, eof, map, opt},
    multi::many0,
    sequence::{delimited, pair, tuple},
    IResult,
};
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum LispValue {
    Atom(AtomType),
    List(Vec<LispValue>),
}

impl fmt::Display for LispValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn format_list(
            list: &[LispValue],
            indent_level: usize,
            f: &mut fmt::Formatter,
        ) -> fmt::Result {
            writeln!(f, "{:indent$}List(", "", indent = indent_level)?;
            for value in list {
                match value {
                    LispValue::Atom(atom) => {
                        writeln!(f, "{:indent$}{},", "", atom, indent = indent_level + 2)?;
                    }
                    LispValue::List(nested_list) => {
                        format_list(nested_list, indent_level + 2, f)?;
                    }
                }
            }
            writeln!(f, "{:indent$})", "", indent = indent_level)
        }

        match self {
            LispValue::Atom(atom) => write!(f, "{}", atom),
            LispValue::List(list) => format_list(list, 0, f),
        }
    }
}

fn parse_single_atom(input: &str) -> IResult<&str, LispValue> {
    let (input, atom) = parse_atom(input)?;
    let lisp_value = LispValue::Atom(atom);
    Ok((input, lisp_value))
}

fn parse_nested_list(input: &str) -> IResult<&str, LispValue> {
    delimited(
        multispace0,
        delimited(
            char('('),
            map(
                many0(alt((parse_single_atom, parse_nested_list))),
                LispValue::List,
            ),
            char(')'),
        ),
        multispace0,
    )(input)
}

fn parse_expression(input: &str) -> IResult<&str, LispValue> {
    alt((parse_single_atom, parse_nested_list))(input)
}

pub fn parse_program(input: &str) -> IResult<&str, LispValue> {
    let (input, (parsed, _)) = all_consuming(pair(
        map(many0(parse_expression), |list| match list.as_slice() {
            [single] => single.clone(),
            _ => LispValue::List(list),
        }),
        opt(tuple((multispace0, opt(line_ending), opt(eof)))),
    ))(input)?;
    Ok((input, parsed))
}

#[cfg(test)]
mod tests {

    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(r#""test""#, LispValue::Atom(AtomType::String("test".to_string())))]
    #[case(r#"    symbol   "#, LispValue::Atom(AtomType::Symbol("symbol".to_string())))]
    fn test_singe_atom(#[case] input: &str, #[case] expected: LispValue) {
        let (rest, parsed) = parse_single_atom(input).unwrap();
        assert!(rest.is_empty());
        assert_eq!(parsed, expected);
    }

    #[rstest]
    #[case(r#"(first (list 1 (+ 2 3) 9))"#, 
    LispValue::List(vec![
        LispValue::Atom(AtomType::Symbol("first".to_string())),
        LispValue::List(vec![
            LispValue::Atom(AtomType::Symbol("list".to_string())),
            LispValue::Atom(AtomType::Integer(1)),
            LispValue::List(vec![
                LispValue::Atom(AtomType::Symbol("+".to_string())),
                LispValue::Atom(AtomType::Integer(2)),
                LispValue::Atom(AtomType::Integer(3)),
            ]),
            LispValue::Atom(AtomType::Integer(9)),
            ])
        ])
    )]
    #[case(r#"(first (list 1 (+ 2 3) 9))
    (first (list 1 (+ 2 3) 9))"#,
    LispValue::List(vec![
        LispValue::List(vec![
            LispValue::Atom(AtomType::Symbol("first".to_string())),
            LispValue::List(vec![
                LispValue::Atom(AtomType::Symbol("list".to_string())),
                LispValue::Atom(AtomType::Integer(1)),
                LispValue::List(vec![
                    LispValue::Atom(AtomType::Symbol("+".to_string())),
                    LispValue::Atom(AtomType::Integer(2)),
                    LispValue::Atom(AtomType::Integer(3)),
                ]),
                LispValue::Atom(AtomType::Integer(9)),
                ])
            ]),
        LispValue::List(vec![
            LispValue::Atom(AtomType::Symbol("first".to_string())),
            LispValue::List(vec![
                LispValue::Atom(AtomType::Symbol("list".to_string())),
                LispValue::Atom(AtomType::Integer(1)),
                LispValue::List(vec![
                    LispValue::Atom(AtomType::Symbol("+".to_string())),
                    LispValue::Atom(AtomType::Integer(2)),
                    LispValue::Atom(AtomType::Integer(3)),
                ]),
                LispValue::Atom(AtomType::Integer(9)),
                ])
            ]),
        ])
    )]

    fn test_parse_program(#[case] input: &str, #[case] expected: LispValue) {
        let (rest, parsed) = parse_program(input).unwrap();
        assert_eq!(parsed, expected);
        assert!(rest.is_empty());
    }
}
