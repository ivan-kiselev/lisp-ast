use super::AtomType;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, char, multispace0},
    multi::{many0, many1},
    sequence::{delimited, pair, tuple},
    IResult,
};

fn parse_global_vars(input: &str) -> IResult<&str, AtomType> {
    delimited(
        multispace0,
        tuple((
            char('*'),
            many1(alt((alphanumeric1, tag("-"), tag("_")))),
            char('*'),
        )),
        multispace0,
    )(input)
    .map(|(input, (_, symbol, _))| (input, AtomType::Symbol(format!("*{}*", symbol.concat()))))
}

fn parse_arithmetics(input: &str) -> IResult<&str, AtomType> {
    delimited(
        multispace0,
        alt((tag("+"), tag("*"), tag("/"), tag("-"))),
        multispace0,
    )(input)
    .map(|(i, o)| (i, AtomType::Symbol(o.to_string())))
}

fn parse_regular_symbols(input: &str) -> IResult<&str, AtomType> {
    delimited(
        multispace0,
        pair(
            alpha1,
            many0(alt((alphanumeric1, tag("-"), tag("_"), tag("*")))),
        ),
        multispace0,
    )(input)
    .map(
        |(input, (first_ch_of_symbol_name, rest_chars_of_symbol_name))| {
            (
                input,
                AtomType::Symbol(format!(
                    "{}{}",
                    first_ch_of_symbol_name,
                    rest_chars_of_symbol_name.concat()
                )),
            )
        },
    )
}

pub fn parse(input: &str) -> IResult<&str, AtomType> {
    alt((parse_global_vars, parse_arithmetics, parse_regular_symbols))(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(" ** another_input", parse_global_vars)]
    #[case("**another_input", parse_global_vars)]
    #[case("**", parse_global_vars)]
    #[case("42", parse_arithmetics)]
    #[case("42", parse_regular_symbols)]
    #[case("(symbol)", parse_regular_symbols)]
    #[case("(+)", parse_regular_symbols)]
    fn generic_error_test<I, R, E, P>(#[case] input: I, #[case] parser: P)
    where
        I: std::cmp::PartialEq + std::fmt::Debug,
        R: std::cmp::PartialEq + std::fmt::Debug,
        E: std::fmt::Debug,
        P: Fn(I) -> IResult<I, R, E>,
    {
        assert!(parser(input).is_err());
    }

    #[test]
    fn regular_symbols_dont_parse_numbers() {
        let input = "42";
        assert!(parse_regular_symbols(input).is_err());
    }

    #[rstest]
    #[case(" *global-var* another_input", "another_input", AtomType::Symbol("*global-var*".to_string()),parse_global_vars)]
    #[case("*global_var* another_input", "another_input", AtomType::Symbol("*global_var*".to_string()), parse_global_vars)]
    #[case("/ 4 2", "4 2", AtomType::Symbol("/".to_string()), parse_arithmetics)]
    #[case("* 4 2", "4 2", AtomType::Symbol("*".to_string()), parse_arithmetics)]
    #[case("- 4 2", "4 2", AtomType::Symbol("-".to_string()), parse_arithmetics)]
    #[case("+ 4 2", "4 2", AtomType::Symbol("+".to_string()), parse_arithmetics)]
    #[case("   +      4 2", "4 2", AtomType::Symbol("+".to_string()), parse_arithmetics)]
    #[case("a", "", AtomType::Symbol("a".to_string()), parse_regular_symbols)]
    #[case(" a ", "", AtomType::Symbol("a".to_string()), parse_regular_symbols)]
    #[case("a1", "", AtomType::Symbol("a1".to_string()), parse_regular_symbols)]
    #[case("my-func +42", "+42", AtomType::Symbol("my-func".to_string()), parse_regular_symbols)]
    #[case("addition_function 42 -21", "42 -21", AtomType::Symbol("addition_function".to_string()), parse_regular_symbols)]
    #[case("a", "", AtomType::Symbol("a".to_string()), parse)]
    #[case("+ 3 1", "3 1", AtomType::Symbol("+".to_string()),parse)]
    #[case("- 4 1", "4 1", AtomType::Symbol("-".to_string()), parse)]
    #[case("*global_var*", "", AtomType::Symbol("*global_var*".to_string()),parse)]
    #[case("my_func 42 -21", "42 -21", AtomType::Symbol("my_func".to_string()),parse)]
    #[case("/2", "2", AtomType::Symbol("/".to_string()),parse)]
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
