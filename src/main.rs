use nom::branch::alt;
use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::character::complete::multispace0;
use nom::combinator::opt;
use nom::sequence::tuple;
use nom::IResult;
#[derive(Debug, PartialEq)]
enum LispValue {
    Atom(AtomType),
    List(Vec<LispValue>),
}

#[derive(Debug, PartialEq)]
enum AtomType {
    Integer(i64),
    Float(f64),
    Symbol(String),
    String(String),
    Char(char),
    OneLineComment(String),
}

fn parse_integer(input: &str) -> IResult<&str, AtomType> {
    let (input, (_, opt_sign, integer_str)) = tuple((
        multispace0,                      // Match and discard leading whitespace
        opt(alt((char('-'), char('+')))), // Optionally match '-' or '+'
        digit1,                           // Match one or more digits
    ))(input)?;

    let formatted_integer = format!("{}{}", opt_sign.unwrap_or(' '), integer_str);
    let formatted_integer = formatted_integer.trim();

    match formatted_integer.parse::<i64>() {
        Ok(number) => Ok((input, AtomType::Integer(number))),
        Err(_) => Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Digit,
        ))),
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_int() {
        let input = "+42";
        let (rest, atom) = parse_integer(input).unwrap();
        assert_eq!(rest, "");
        assert_eq!(atom, AtomType::Integer(42));
        let input = "-42";
        let (rest, atom) = parse_integer(input).unwrap();
        assert_eq!(rest, "");
        assert_eq!(atom, AtomType::Integer(-42));
        let input = "42";
        let (rest, atom) = parse_integer(input).unwrap();
        assert_eq!(rest, "");
        assert_eq!(atom, AtomType::Integer(42));
        let input = " 42";
        let (rest, atom) = parse_integer(input).unwrap();
        assert_eq!(rest, "");
        assert_eq!(atom, AtomType::Integer(42));
        let input = " -42";
        let (rest, atom) = parse_integer(input).unwrap();
        assert_eq!(rest, "");
        assert_eq!(atom, AtomType::Integer(-42));
    }
}
