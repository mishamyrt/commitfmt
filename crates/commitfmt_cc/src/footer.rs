use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::{line_ending, space0, space1};
use nom::character::one_of;
use nom::combinator::{all_consuming, map, recognize};
use nom::error::Error;
use nom::multi::{fold_many1, separated_list1};
use nom::sequence::preceded;
use nom::{IResult, Parser};

/// Indicates on which side of the separator the space should be
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum SeparatorAlignment {
    #[default]
    Left,
    Right,
}

impl SeparatorAlignment {
    pub fn from(v: &str) -> Option<Self> {
        match v {
            "left" => Some(SeparatorAlignment::Left),
            "right" => Some(SeparatorAlignment::Right),
            _ => None,
        }
    }
}

/// Footer represents a commit footer
/// It consists of a key and a value separated by a separator.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Footer {
    pub key: String,
    pub value: String,
    pub separator: char,
    pub alignment: SeparatorAlignment,
}

impl Footer {
    /// Keep this constant because it's exception and breaks git trailers format
    pub(crate) const BREAKING_TAG: &'static str = "BREAKING CHANGES";

    /// Default key and value separator
    pub const DEFAULT_SEPARATOR: &'static str = ":";

    /// Returns the number of characters in the right formatted footer
    pub fn len(&self) -> usize {
        let mut value_len = self.value.len();
        for ch in self.value.chars() {
            if ch == '\n' {
                // Add one for the space after newline
                value_len += 1;
            }
        }

        self.key.len() + value_len + 2
    }

    pub fn is_empty(&self) -> bool {
        self.key.is_empty() && self.value.is_empty()
    }

    /// Returns a footer from a string. Returns `None` if the input is not a valid footer.
    pub fn from_value(input: &str) -> Option<Self> {
        match Self::take(Self::DEFAULT_SEPARATOR).parse(input) {
            Ok((_, footer)) => Some(footer),
            Err(_) => None,
        }
    }

    /// Returns a footer from a string. Returns `None` if the input is not a valid footer.
    pub fn from_value_with_separators(input: &str, separators: &str) -> Option<Self> {
        match Self::take(separators).parse(input) {
            Ok((_, footer)) => Some(footer),
            Err(_) => None,
        }
    }

    /// Checks if a key is a breaking change.
    pub fn is_breaking_key(key: &str) -> bool {
        let lower_key = key.to_lowercase();
        key == Self::BREAKING_TAG
            || lower_key == "breaking-changes"
            || lower_key == "breakingchanges"
    }

    pub fn is_breaking_change(&self) -> bool {
        Self::is_breaking_key(&self.key)
    }

    /// Parses a separator and its alignment.
    /// Returns `None` if the input is not a valid separator string.
    fn parse_separator(separator_with_spaces: &str) -> Option<(char, SeparatorAlignment)> {
        let mut balance: i16 = 0;
        let mut separator: char = '\0';
        for ch in separator_with_spaces.chars() {
            if ch.is_whitespace() {
                if separator == '\0' {
                    balance += 1;
                } else {
                    balance -= 1;
                }
            } else if separator == '\0' {
                separator = ch;
            } else {
                return None;
            }
        }

        let alignment =
            if balance > 0 { SeparatorAlignment::Right } else { SeparatorAlignment::Left };

        Some((separator, alignment))
    }

    /// Takes all footers from the input
    /// Returns them and the rest of the input
    pub(crate) fn parse<'input, 'sep: 'input>(
        input: &'input str,
        separators: &'sep str,
    ) -> IResult<&'input str, Vec<Footer>> {
        all_consuming(separated_list1(line_ending, Self::take(separators))).parse(input)
    }

    /// Parses one footer (trailer) from the input.
    /// Returns it and the rest of the input.
    fn take(separators: &str) -> impl Parser<&str, Output = Footer, Error = Error<&str>> {
        map(
            (
                Self::key_parser,
                recognize((preceded(space0, one_of(separators)), space0)),
                Self::value_parser,
            ),
            |(key, separator_with_spaces, value)| {
                let (separator, alignment) =
                    Self::parse_separator(separator_with_spaces).unwrap();

                Self { key: key.to_string(), value, separator, alignment }
            },
        )
    }

    /// Checks if a character is a valid key character
    fn is_valid_key_char(c: char) -> bool {
        c.is_ascii_alphabetic() || c.is_ascii_digit() || c == '-'
    }

    /// Parse a footer key
    fn key_parser(input: &str) -> IResult<&str, &str> {
        alt((tag(Self::BREAKING_TAG), take_while1(Self::is_valid_key_char))).parse(input)
    }

    /// Parse a footer value
    fn value_line(input: &str) -> IResult<&str, &str> {
        preceded(space1, take_while1(|c| c != '\n')).parse(input)
    }

    /// Parse a footer value
    fn value_parser(input: &str) -> IResult<&str, String> {
        fold_many1(
            alt((take_while1(|c| c != '\n'), preceded(line_ending, Self::value_line))),
            String::new,
            |mut acc, piece: &str| {
                if !acc.is_empty() {
                    acc.push('\n');
                }
                acc.push_str(piece);
                acc
            },
        )
        .parse(input)
    }
}

impl std::fmt::Display for Footer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.key)?;
        if self.alignment == SeparatorAlignment::Right {
            write!(f, " {}", self.separator)?;
        } else {
            write!(f, "{} ", self.separator)?;
        }
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_take() {
        let input = "Authored-By: John Doe\nCommitter: Jane Doe";
        let expected = Footer {
            key: "Authored-By".into(),
            value: "John Doe".into(),
            separator: ':',
            alignment: SeparatorAlignment::Left,
        };

        let (rest, footer) = Footer::take(":").parse(input).unwrap();
        assert_eq!(footer, expected);
        assert_eq!(rest, "\nCommitter: Jane Doe");
    }

    #[test]
    fn test_take_multiline() {
        let input = "BREAKING CHANGES: Long description\n That even contains newlines";
        let expected = Footer {
            key: "BREAKING CHANGES".into(),
            value: "Long description\nThat even contains newlines".into(),
            separator: ':',
            alignment: SeparatorAlignment::Left,
        };

        let (_, footer) = Footer::take(":").parse(input).unwrap();
        assert_eq!(footer, expected);
    }

    #[test]
    fn test_take_alignment() {
        let mut parser = Footer::take(":#");

        let (_, footer) = parser.parse("Authored-By: John Doe").unwrap();
        assert_eq!(footer.alignment, SeparatorAlignment::Left);

        let (_, footer) = parser.parse("Authored-By : John Doe").unwrap();
        assert_eq!(footer.alignment, SeparatorAlignment::Left);

        let (_, footer) = parser.parse("Issue #123").unwrap();
        assert_eq!(footer.alignment, SeparatorAlignment::Right);
    }

    #[test]
    fn test_is_breaking_key() {
        assert!(Footer::is_breaking_key("BREAKING CHANGES"));
        assert!(Footer::is_breaking_key("breaking-changes"));
        assert!(Footer::is_breaking_key("breakingchanges"));
        assert!(Footer::is_breaking_key("BreakingChanges"));
        assert!(!Footer::is_breaking_key("BREAKING CHANGES BUT NO"));
        assert!(!Footer::is_breaking_key("not-breaking"));
        assert!(!Footer::is_breaking_key(""));
    }

    #[test]
    fn test_parse_single() {
        let footer = Footer::parse("foo: bar", Footer::DEFAULT_SEPARATOR).unwrap().1;

        assert_eq!(footer.len(), 1);
        assert_eq!(footer[0].key, "foo");
        assert_eq!(footer[0].value, "bar");
    }

    #[test]
    fn test_parse_multiple() {
        let result = Footer::parse("foo: bar\nbaz: qux", Footer::DEFAULT_SEPARATOR).unwrap().1;

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].key, "foo");
        assert_eq!(result[0].value, "bar");
        assert_eq!(result[1].key, "baz");
        assert_eq!(result[1].value, "qux");
    }
}
