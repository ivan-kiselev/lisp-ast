use super::AtomType;
use nom::{
    branch::alt,
    character::complete::{char, digit0, digit1, multispace0},
    combinator::opt,
    sequence::tuple,
    IResult,
};

fn parse(input: &str) -> IResult<&str, AtomType> {
    let (input, (_, opt_sign, whole_part_str, _, fract_part_str, _)) = tuple((
        multispace0,
        opt(alt((char('-'), char('+')))),
        digit1,
        char('.'),
        opt(digit0),
        multispace0,
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
    use rstest::rstest;

    #[rstest]
    #[case("42.", "", AtomType::Float(42.))]
    #[case("-42.", "", AtomType::Float(-42.))]
    #[case(" 42.", "", AtomType::Float(42.))]
    #[case(" -42.", "", AtomType::Float(-42.))]
    #[case("+42.", "", AtomType::Float(42.))]
    #[case("+42.42", "", AtomType::Float(42.42))]
    #[case("    +42.42       ", "", AtomType::Float(42.42))]
    #[case(
        "    +42.42       something_else",
        "something_else",
        AtomType::Float(42.42)
    )]

    fn parse_float(
        #[case] input: &str,
        #[case] rest_expected: &str,
        #[case] atom_expected: AtomType,
    ) {
        let (rest_got, atom_got) = parse(input).unwrap();
        assert_eq!(rest_got, rest_expected);
        assert_eq!(atom_got, atom_expected);
    }
}
