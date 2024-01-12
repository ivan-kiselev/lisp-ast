use super::AtomType;
use nom::{
    branch::alt,
    character::complete::{char, digit1, multispace0},
    combinator::opt,
    sequence::tuple,
    IResult,
};

fn parse(input: &str) -> IResult<&str, AtomType> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_int() {
        let input = "+42";
        let (rest, atom) = parse(input).unwrap();
        assert_eq!(rest, "");
        assert_eq!(atom, AtomType::Integer(42));
        let input = "-42";
        let (rest, atom) = parse(input).unwrap();
        assert_eq!(rest, "");
        assert_eq!(atom, AtomType::Integer(-42));
        let input = "42";
        let (rest, atom) = parse(input).unwrap();
        assert_eq!(rest, "");
        assert_eq!(atom, AtomType::Integer(42));
        let input = " 42";
        let (rest, atom) = parse(input).unwrap();
        assert_eq!(rest, "");
        assert_eq!(atom, AtomType::Integer(42));
        let input = " -42";
        let (rest, atom) = parse(input).unwrap();
        assert_eq!(rest, "");
        assert_eq!(atom, AtomType::Integer(-42));
    }
}
