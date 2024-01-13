use super::AtomType;
use nom::{
    branch::alt,
    character::complete::{char, digit1, multispace0},
    combinator::opt,
    sequence::tuple,
    IResult,
};

fn parse(input: &str) -> IResult<&str, AtomType> {
    let (input, (_, opt_sign, integer_str, _)) = tuple((
        multispace0,
        opt(alt((char('-'), char('+')))),
        digit1,
        multispace0,
    ))(input)?;

    let formatted_integer = format!(
        "{}{}",
        opt_sign.map(String::from).unwrap_or_default(),
        integer_str
    );

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
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("42", "", AtomType::Integer(42))]
    #[case("-42", "", AtomType::Integer(-42))]
    #[case(" 42", "", AtomType::Integer(42))]
    #[case(" -42", "", AtomType::Integer(-42))]
    #[case("+42", "", AtomType::Integer(42))]
    #[case(
        "      +42                            something_else",
        "something_else",
        AtomType::Integer(42)
    )]
    fn parse_int(
        #[case] input: &str,
        #[case] rest_expected: &str,
        #[case] atom_expected: AtomType,
    ) {
        let (rest_got, atom_got) = parse(input).unwrap();
        assert_eq!(rest_got, rest_expected);
        assert_eq!(atom_got, atom_expected);
    }
}
