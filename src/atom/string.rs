use super::AtomType;
use nom::{
    branch::alt,
    character::complete::{char, none_of},
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
        char('"'),
        recognize(many0(alt((parse_escaped_quote, none_of("\""))))),
        char('"'),
    )(input)?;
    Ok((input, AtomType::String(string.into())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_string() {
        let input = r#""asd""#;
        let (rest, atom) = parse(input).unwrap();
        assert_eq!(rest, "");
        assert_eq!(atom, AtomType::String("asd".to_string()));
        let input = r#""""#;
        let (rest, atom) = parse(input).unwrap();
        assert_eq!(rest, "");
        assert_eq!(atom, AtomType::String("".to_string()));
        let input = r#""$?!@#_:;;""#;
        let (rest, atom) = parse(input).unwrap();
        assert_eq!(rest, "");
        assert_eq!(atom, AtomType::String("$?!@#_:;;".to_string()));
        let input = r#""\"""#;
        let (rest, atom) = parse(input).unwrap();
        assert_eq!(rest, "");
        assert_eq!(atom, AtomType::String(r#"\""#.to_string()));
    }
}
