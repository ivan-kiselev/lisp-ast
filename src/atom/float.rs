use super::AtomType;
use nom::{
    branch::alt,
    character::complete::{char, digit0, digit1, multispace0},
    combinator::opt,
    sequence::tuple,
    IResult,
};

fn parse(input: &str) -> IResult<&str, AtomType> {
    let (input, (_, opt_sign, whole_part_str, _, fract_part_str)) = tuple((
        multispace0,                      // Match and discard leading whitespace
        opt(alt((char('-'), char('+')))), // Optionally match '-' or '+'
        digit1,                           // Match one or more digits
        char('.'),                        // Match . dividing fractional and whole part
        opt(digit0),                      // Match optional fractional part
    ))(input)?;

    let formatted_integer = format!(
        "{}{}.{}",
        opt_sign.unwrap_or(' '),
        whole_part_str,
        fract_part_str.unwrap_or("0")
    );
    let formatted_integer = formatted_integer.trim();

    match formatted_integer.parse::<f64>() {
        Ok(number) => Ok((input, AtomType::Float(number))),
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
    fn parse_float() {
        let input = "42.";
        let (rest, atom) = parse(input).unwrap();
        assert_eq!(rest, "");
        assert_eq!(atom, AtomType::Float(42.));
        let input = "-42.";
        let (rest, atom) = parse(input).unwrap();
        assert_eq!(rest, "");
        assert_eq!(atom, AtomType::Float(-42.));
        let input = "42.";
        let (rest, atom) = parse(input).unwrap();
        assert_eq!(rest, "");
        assert_eq!(atom, AtomType::Float(42.));
        let input = " 42.";
        let (rest, atom) = parse(input).unwrap();
        assert_eq!(rest, "");
        assert_eq!(atom, AtomType::Float(42.));
        let input = " -42.";
        let (rest, atom) = parse(input).unwrap();
        assert_eq!(rest, "");
        assert_eq!(atom, AtomType::Float(-42.));
        let input = " -42.5";
        let (rest, atom) = parse(input).unwrap();
        assert_eq!(rest, "");
        assert_eq!(atom, AtomType::Float(-42.5));
        let input = "42.5";
        let (rest, atom) = parse(input).unwrap();
        assert_eq!(rest, "");
        assert_eq!(atom, AtomType::Float(42.5));
    }
}
