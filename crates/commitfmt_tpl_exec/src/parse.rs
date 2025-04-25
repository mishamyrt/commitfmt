use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_until},
    character::complete::multispace0,
    combinator::map,
    error::{Error as NomError, ErrorKind},
    sequence::{delimited, preceded},
    IResult, Parser,
};
use std::str;

// Define segment types we can parse
#[derive(Debug, PartialEq)]
pub(crate) enum Segment<'a> {
    Literal(&'a str),
    Command(&'a str),
}

const TAG_CMD_START: &str = "{{";
const TAG_CMD_END: &str = "}}";

/// Parser for text NOT containing TAG_CMD_START up to the end or next TAG_CMD_END
fn parse_literal<'a>(input: &'a str) -> IResult<&'a str, Segment<'a>, NomError<&'a str>> {
    if input.is_empty() {
        return Err(nom::Err::Error(NomError::new(input, ErrorKind::TakeUntil)));
    }

    match take_until::<_, _, NomError<&str>>(TAG_CMD_START)(input) {
        Ok((remaining, literal)) => {
            if literal.is_empty() {
                Err(nom::Err::Error(NomError::new(input, ErrorKind::IsNot)))
            } else {
                Ok((remaining, Segment::Literal(literal)))
            }
        }
        Err(nom::Err::Error(e)) if e.code == ErrorKind::TakeUntil => {
            Ok(("", Segment::Literal(input)))
        }
        Err(e) => Err(e),
    }
}

/// Parser for "TAG_CMD_START command TAG_CMD_END"
fn parse_command<'a>(input: &'a str) -> IResult<&'a str, Segment<'a>, NomError<&'a str>> {
    map(
        delimited(
            tag(TAG_CMD_START),
            // Trim whitespace around the command itself
            preceded(multispace0, is_not(TAG_CMD_END)),
            tag(TAG_CMD_END),
        ),
        |cmd: &str| Segment::Command(cmd.trim()),
    )
    .parse(input)
}

/// Parser for either a literal or a command block
pub(crate) fn parse_segment<'a>(
    input: &'a str,
) -> IResult<&'a str, Segment<'a>, NomError<&'a str>> {
    alt((parse_command, parse_literal)).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_literal_no_command() {
        let input = "Hello, world!";
        let (remaining, segment) = parse_literal(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(segment, Segment::Literal("Hello, world!"));
    }

    #[test]
    fn test_parse_literal_with_command_marker_in_middle() {
        let input = "Hello {{ cmd }}";
        let (remaining, segment) = parse_literal(input).unwrap();
        assert_eq!(segment, Segment::Literal("Hello "));
        assert_eq!(remaining, "{{ cmd }}");
    }

    #[test]
    fn test_parse_command() {
        let input = "{{   ls -la  }}world";
        let (remaining, segment) = parse_command(input).unwrap();
        assert_eq!(segment, Segment::Command("ls -la"));
        assert_eq!(remaining, "world");
    }

    #[test]
    fn test_parse_segment_literal() {
        let input = "Just a simple text";
        let (remaining, segment) = parse_segment(input).unwrap();
        assert_eq!(segment, Segment::Literal("Just a simple text"));
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_parse_segment_command() {
        let input = "{{echo hi}}";
        let (remaining, segment) = parse_segment(input).unwrap();
        assert_eq!(segment, Segment::Command("echo hi"));
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_parse_multiple_segments() {
        let mut input = "Start {{cmd1}} middle {{ cmd2 }} end";
        let mut segments = Vec::new();
        while !input.is_empty() {
            let (next_input, segment) = parse_segment(input).unwrap();
            segments.push(segment);
            input = next_input;
        }
        assert_eq!(segments.len(), 5);
        assert_eq!(segments[0], Segment::Literal("Start "));
        assert_eq!(segments[1], Segment::Command("cmd1"));
        assert_eq!(segments[2], Segment::Literal(" middle "));
        assert_eq!(segments[3], Segment::Command("cmd2"));
        assert_eq!(segments[4], Segment::Literal(" end"));
    }
}
