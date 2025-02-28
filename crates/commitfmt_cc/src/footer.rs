use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::{line_ending, space0, space1};
use nom::character::one_of;
use nom::combinator::{all_consuming, map};
use nom::multi::{many1, separated_list0};
use nom::sequence::{preceded, separated_pair};
use nom::{IResult, Parser};

pub const DEFAULT_SEPARATOR: &str = ":";
const BREAKING_TAG: &str = "BREAKING CHANGES";

/// Footer represents a commit footer
/// It consists of a key and a value separated by a colon (by default).
#[derive(Debug, PartialEq)]
pub struct Footer {
    pub key: String,
    pub value: String,
}

impl Footer {
    /// Checks if a character is a valid key character
    fn is_valid_key_char(c: char) -> bool {
        c.is_ascii_alphabetic() || c == '-'
    }

    /// Parse a footer key
    fn key_parser(input: &str) -> IResult<&str, &str> {
        alt((
            tag(BREAKING_TAG),
            take_while1(Self::is_valid_key_char),
        )).parse(input)
    }

    /// Parse a footer value
    fn value_line(input: &str) -> IResult<&str, &str> {
        preceded(space1, take_while1(|c| c != '\n')).parse(input)
    }

    /// Parse a footer value
    fn value_parser(input: &str) -> IResult<&str, String> {
        map(
            many1(alt((
                take_while1(|c| c != '\n'),
                preceded(line_ending, Self::value_line),
            ))),
            |lines| lines.join("\n"),
        ).parse(input)
    }

    /// Parse a list of footers
    pub fn parse(input: &str, separators: &str) -> Option<Vec<Self>> {
        let footer_parser = map(
            separated_pair(
                Self::key_parser,
                preceded(space0, one_of(separators)),
                preceded(space1, Self::value_parser)
            ),
            |(key, value)| Self {
                key: key.to_string(),
                value,
            },
        );

        let result = all_consuming(separated_list0(
            tag("\n"),
            footer_parser
        )).parse(input);

        match result {
            Ok((_, footers)) => Some(footers),
            Err(_) => None,
        }
    }
}

#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple() {
        let input = "Authored-By: John Doe";
        let expected = Some(vec![
            Footer {
                key: "Authored-By".to_string(),
                value: "John Doe".to_string(),
            },
        ]);
        assert_eq!(Footer::parse(input, DEFAULT_SEPARATOR), expected);
    }

    #[test]
    fn test_parse_multiple() {
        let input = "Authored-By: John Doe\nReviewed-By: Jane Doe";
        let expected = Some(vec![
            Footer {
                key: "Authored-By".to_string(),
                value: "John Doe".to_string(),
            },
            Footer {
                key: "Reviewed-By".to_string(),
                value: "Jane Doe".to_string(),
            },
        ]);
        assert_eq!(Footer::parse(input, DEFAULT_SEPARATOR), expected);
    }

    #[test]
    fn test_parse_breaking() {
        let input = "BREAKING CHANGES: This is a breaking change";
        let expected = Some(vec![
            Footer {
                key: "BREAKING CHANGES".to_string(),
                value: "This is a breaking change".to_string(),
            },
        ]);
        assert_eq!(Footer::parse(input, DEFAULT_SEPARATOR), expected);
    }

    #[test]
    fn test_parse_multiline() {
        let input = "Long-Trailer: First\n Second\n Third";
        let expected = Some(vec![
            Footer {
                key: "Long-Trailer".to_string(),
                value: "First\nSecond\nThird".to_string(),
            },
        ]);
        assert_eq!(Footer::parse(input, DEFAULT_SEPARATOR), expected);
    }
}
