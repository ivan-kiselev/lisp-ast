use super::AtomType;
use nom::{
    branch::alt,
    character::complete::{char, multispace0, none_of},
    combinator::{recognize, value},
    error::ParseError,
    multi::many0,
    sequence::{delimited, preceded},
    IResult,
};

fn parse_escaped_quote<'a, E>(input: &'a str) -> IResult<&'a str, char, E>
where
    E: ParseError<&'a str>,
{
    value('"', preceded(char('\\'), char('"')))(input)
}

pub fn parse(input: &str) -> IResult<&str, AtomType> {
    let (input, string) = delimited(
        multispace0,
        delimited(
            char('"'),
            recognize(many0(alt((parse_escaped_quote, none_of("\""))))),
            char('"'),
        ),
        multispace0,
    )(input)?;
    Ok((input, AtomType::String(string.into())))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(r#""asd""#, "", AtomType::String("asd".to_string()))]
    #[case(r#""""#, "", AtomType::String("".to_string()))]
    #[case(r#""$?!@#_:;;""#, "", AtomType::String("$?!@#_:;;".to_string()))]
    #[case(r#""\"" symbol"#, "symbol", AtomType::String(r#"\""#.to_string()))]
    #[case(r#""\""            42"#, "42", AtomType::String(r#"\""#.to_string()))]
    fn parse_string(
        #[case] input: &str,
        #[case] rest_expected: &str,
        #[case] atom_expected: AtomType,
    ) {
        let (rest_got, atom_got) = parse(input).unwrap();
        assert_eq!(rest_got, rest_expected);
        assert_eq!(atom_got, atom_expected);
    }

    #[rstest]
    #[case(r#"(":")""#)]
    #[case(r#"("")""#)]
    fn test_err(#[case] input: &str) {
        assert!(parse(input).is_err())
    }
}
