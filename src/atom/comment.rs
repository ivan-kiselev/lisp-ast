use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{multispace0, not_line_ending},
    combinator::value,
    multi::many0,
    sequence::{delimited, tuple},
    IResult,
};

use super::AtomType;

fn parse_single_line_comment(input: &str) -> IResult<&str, AtomType> {
    delimited(
        multispace0,
        value(AtomType::Null, tuple((tag(";;"), not_line_ending))),
        multispace0,
    )(input)
}

fn parse_multiline_comment(input: &str) -> IResult<&str, AtomType> {
    delimited(
        multispace0,
        value(
            AtomType::Null,
            delimited(tag("#|"), take_until("|#"), tag("|#")),
        ),
        multispace0,
    )(input)
}
pub fn parse_comment(input: &str) -> IResult<&str, AtomType> {
    alt((parse_multiline_comment, parse_single_line_comment))(input)
}

pub fn parse0(input: &str) -> IResult<&str, Vec<AtomType>> {
    many0(parse_comment)(input)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(
        r#";; comment
    something_else"#,
        "something_else",
        AtomType::Null,
        parse_single_line_comment
    )]
    #[case(
        r#"#|
    Comment
    Commnet
    |#"#,
        "",
        AtomType::Null,
        parse_multiline_comment
    )]
    #[case(
        r#"
        
        
    #|
     Comment
     Commnet
     |#
     
     
     
     
     "#,
        "",
        AtomType::Null,
        parse_multiline_comment
    )]
    #[case(r#";; comment"#, "", AtomType::Null, parse_comment)]
    #[case(r#"#| comment |#"#, "", AtomType::Null, parse_comment)]
    #[case(r#"#| comment |#"#, "", vec![AtomType::Null], parse0)]
    #[case(r#";; comment |#"#, "", vec![AtomType::Null], parse0)]
    #[case(r#";; comment 
    #|
    another comment |#
    
    
    "#, "", vec![AtomType::Null, AtomType::Null], parse0)]

    fn generic_test<I, R, P, E>(
        #[case] input: I,
        #[case] rest_expected: I,
        #[case] atoms_expected: R,
        #[case] parser: P,
    ) where
        I: std::cmp::PartialEq + std::fmt::Debug,
        R: std::cmp::PartialEq + std::fmt::Debug,
        E: std::fmt::Debug,
        P: Fn(I) -> IResult<I, R, E>,
    {
        let (rest_got, atoms_got) = parser(input).unwrap();
        assert_eq!(rest_got, rest_expected);
        assert_eq!(atoms_got, atoms_expected);
    }
}
