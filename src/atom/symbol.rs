use super::AtomType;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, char, multispace0, multispace1},
    multi::{many0, many1},
    sequence::{delimited, pair, tuple},
    IResult,
};

fn parse_global_vars(input: &str) -> IResult<&str, AtomType> {
    let (input, (_, symbol, _)) = delimited(
        multispace0,
        tuple((
            char('*'),
            many1(alt((alphanumeric1, tag("-"), tag("_")))),
            char('*'),
        )),
        multispace0,
    )(input)?;
    Ok((input, AtomType::Symbol(format!("*{}*", symbol.concat()))))
}

fn parse_arithmetics(input: &str) -> IResult<&str, AtomType> {
    let (input, symbol) = delimited(
        multispace0,
        alt((tag("+"), tag("*"), tag("/"), tag("-"))),
        multispace1,
    )(input)?;
    Ok((input, AtomType::Symbol(symbol.to_string())))
}

fn parse_regular_symbols(input: &str) -> IResult<&str, AtomType> {
    let (input, (first_ch_of_symbol_name, rest_chars_of_symbol_name)) = delimited(
        multispace0,
        pair(
            alpha1,
            many0(alt((alphanumeric1, tag("-"), tag("_"), tag("*")))),
        ),
        multispace0,
    )(input)?;
    Ok((
        input,
        AtomType::Symbol(format!(
            "{}{}",
            first_ch_of_symbol_name,
            rest_chars_of_symbol_name.concat()
        )),
    ))
}

pub fn parse(input: &str) -> IResult<&str, AtomType> {
    let (input, symbol) =
        alt((parse_arithmetics, parse_global_vars, parse_regular_symbols))(input)?;
    Ok((input, symbol))
}

pub fn parse0(input: &str) -> IResult<&str, Vec<AtomType>> {
    let (input, symbol) = many0(parse)(input)?;
    Ok((input, symbol))
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn global_vars() {
        let input = " *global-var* another_input";
        let (rest, atom) = parse_global_vars(input).unwrap();
        assert_eq!(rest, "another_input");
        assert_eq!(atom, AtomType::Symbol("*global-var*".to_string()));
        let input = "*global_var* another_input";
        let (rest, atom) = parse_global_vars(input).unwrap();
        assert_eq!(rest, "another_input");
        assert_eq!(atom, AtomType::Symbol("*global_var*".to_string()));

        let input = "**";
        assert!(parse_global_vars(input).is_err());
    }
    #[test]
    fn arithmetics() {
        let input = "/ 4 2";
        let (rest, atom) = parse_arithmetics(input).unwrap();
        assert_eq!(rest, "4 2");
        assert_eq!(atom, AtomType::Symbol("/".to_string()));
        let input = "* 4 2";
        let (rest, atom) = parse_arithmetics(input).unwrap();
        assert_eq!(rest, "4 2");
        assert_eq!(atom, AtomType::Symbol("*".to_string()));
        let input = "- 4 2";
        let (rest, atom) = parse_arithmetics(input).unwrap();
        assert_eq!(rest, "4 2");
        assert_eq!(atom, AtomType::Symbol("-".to_string()));
        let input = "+ 4 2";
        let (rest, atom) = parse_arithmetics(input).unwrap();
        assert_eq!(rest, "4 2");
        assert_eq!(atom, AtomType::Symbol("+".to_string()));
    }
    #[test]
    fn arithmetics_dont_parse_global_vars() {
        let input = "*my_var*";
        assert!(parse_arithmetics(input).is_err());
    }
    #[test]
    fn arithmetics_dont_parse_negative_and_positive_numbers() {
        let input = "-42";
        assert!(parse_arithmetics(input).is_err());
        let input = "+42";
        assert!(parse_arithmetics(input).is_err());
    }

    #[test]
    fn regular_symbols() {
        let input = "a";
        let (rest, atom) = parse_regular_symbols(input).unwrap();
        assert_eq!(rest, "");
        assert_eq!(atom, AtomType::Symbol("a".to_string()));
        let input = " a ";
        let (rest, atom) = parse_regular_symbols(input).unwrap();
        assert_eq!(rest, "");
        assert_eq!(atom, AtomType::Symbol("a".to_string()));
        let input = "a1";
        let (rest, atom) = parse_regular_symbols(input).unwrap();
        assert_eq!(rest, "");
        assert_eq!(atom, AtomType::Symbol("a1".to_string()));
        let input = "my-func";
        let (rest, atom) = parse_regular_symbols(input).unwrap();
        assert_eq!(rest, "");
        assert_eq!(atom, AtomType::Symbol("my-func".to_string()));
        let input = "my-func +42";
        let (rest, atom) = parse_regular_symbols(input).unwrap();
        assert_eq!(rest, "+42");
        assert_eq!(atom, AtomType::Symbol("my-func".to_string()));
        let input = "addition_function 42 -32";
        let (rest, atom) = parse_regular_symbols(input).unwrap();
        assert_eq!(rest, "42 -32");
        assert_eq!(atom, AtomType::Symbol("addition_function".to_string()));
    }
    #[test]
    fn regular_symbols_dont_parse_numbers() {
        let input = "42";
        assert!(parse_regular_symbols(input).is_err());
    }

    #[test]
    fn test_combinator() {
        let input = "a";
        let (rest, atom) = parse(input).unwrap();
        assert_eq!(rest, "");
        assert_eq!(atom, AtomType::Symbol("a".to_string()));
        let input = "+ 3 1";
        let (rest, atom) = parse(input).unwrap();
        assert_eq!(rest, "3 1");
        assert_eq!(atom, AtomType::Symbol("+".to_string()));
        let input = "- 4 1";
        let (rest, atom) = parse(input).unwrap();
        assert_eq!(rest, "4 1");
        assert_eq!(atom, AtomType::Symbol("-".to_string()));
        let input = "*global_var*";
        let (rest, atom) = parse(input).unwrap();
        assert_eq!(rest, "");
        assert_eq!(atom, AtomType::Symbol("*global_var*".to_string()));
        let input = "my_func";
        let (rest, atom) = parse(input).unwrap();
        assert_eq!(rest, "");
        assert_eq!(atom, AtomType::Symbol("my_func".to_string()));
        let input = "my-func +42";
        let (rest, atom) = parse(input).unwrap();
        assert_eq!(rest, "+42");
        assert_eq!(atom, AtomType::Symbol("my-func".to_string()));
        let input = "addition_function 42 -32";
        let (rest, atom) = parse(input).unwrap();
        assert_eq!(rest, "42 -32");
        assert_eq!(atom, AtomType::Symbol("addition_function".to_string()));
    }

    #[test]
    fn test_combinator0() {
        let input = "my_func *my_var* another-var";
        let (rest, atoms) = parse0(input).unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            atoms,
            vec![
                AtomType::Symbol("my_func".to_string()),
                AtomType::Symbol("*my_var*".to_string()),
                AtomType::Symbol("another-var".to_string())
            ]
        );
        let input = "    my_func       *my_var*           another-var";
        let (rest, atoms) = parse0(input).unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            atoms,
            vec![
                AtomType::Symbol("my_func".to_string()),
                AtomType::Symbol("*my_var*".to_string()),
                AtomType::Symbol("another-var".to_string())
            ]
        );
    }
}
