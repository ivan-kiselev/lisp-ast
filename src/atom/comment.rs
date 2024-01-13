use nom::{
    bytes::complete::{tag, take_until},
    combinator::value,
    sequence::tuple,
    IResult,
};

pub fn parse_multiline_comment(input: &str) -> IResult<&str, ()> {
    value(
        (), // Output is thrown away.
        tuple((tag("#|"), take_until("|#"), tag("|#"))),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiline_comment() {
        let input = r#"#|
         Comment
         Commnet
         |#"#;
        let (rest, result) = parse_multiline_comment(input).unwrap();
        assert_eq!(rest, "");
        assert_eq!(result, ());
    }
}
