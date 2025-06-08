use thiserror::Error;

use crate::body::{parse_body, DEFAULT_COMMENT_SYMBOL};
use crate::footer::Footers;
use crate::header::Header;
use crate::Footer;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Unable to parse commit message")]
    InvalidCommitMessage(String),
}

/// Commit message
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Message {
    pub header: Header,
    pub body: Option<String>,
    pub footers: Footers,
}

impl Message {
    pub fn parse(
        input: &str,
        footer_separators: Option<&str>,
        comment_symbol: Option<&str>,
    ) -> Result<Self, ParseError> {
        let header_end = input.find('\n').unwrap_or(input.len());
        let header = Header::from(&input[..header_end]);

        if header_end == input.len() {
            return Ok(Message { header, body: None, footers: Footers::default() });
        }

        let footer_separators = footer_separators.unwrap_or(Footer::DEFAULT_SEPARATOR);
        let comment_symbol = comment_symbol.unwrap_or(DEFAULT_COMMENT_SYMBOL);

        let body_input = &input[header_end + 1..];
        let (body, footers) = parse_body(body_input, footer_separators, comment_symbol);

        Ok(Message { header, body, footers: footers.unwrap_or_default() })
    }
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.header)?;
        if let Some(body) = &self.body {
            write!(f, "\n\n{body}")?;
        }
        if !self.footers.is_empty() {
            writeln!(f)?;
            write!(f, "\n{}", self.footers)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{footer::SeparatorAlignment, footer_vec, header::Scope};

    use super::*;

    #[test]
    fn test_parse() {
        let commit_msg = "feat: my feature

Description body

Authored-By: John Doe";

        let parsed = Message::parse(commit_msg, Some(":"), Some("#"));
        let expected = Message {
            header: Header {
                kind: Some("feat".to_string()),
                scope: Scope::default(),
                description: "my feature".to_string(),
                breaking: false,
            },
            body: Some("Description body".to_string()),
            footers: footer_vec![{
                key: "Authored-By".to_string(),
                value: "John Doe".to_string(),
                separator: ':',
                alignment: SeparatorAlignment::Left,
            }],
        };

        match parsed {
            Ok(parsed) => assert_eq!(parsed, expected),
            Err(e) => panic!("Unable to parse commit message: {e}"),
        }
    }

    #[test]
    fn test_parse_without_body() {
        let commit_msg = "feat: my feature\n\nAuthored-By: John Doe";

        let parsed = Message::parse(commit_msg, Some(":"), Some("#")).unwrap();
        let expected = Message {
            header: Header {
                kind: Some("feat".to_string()),
                scope: Scope::default(),
                description: "my feature".to_string(),
                breaking: false,
            },
            body: None,
            footers: footer_vec![{
                key: "Authored-By".to_string(),
                value: "John Doe".to_string(),
                separator: ':',
                alignment: SeparatorAlignment::Left,
            }],
        };

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_ignores_extra_newlines() {
        let commit_msg_with_newlines = "feat: my feature\n\nbody";
        let commit_msg_without_newlines = "feat: my feature\nbody";

        let parsed_with_newlines =
            Message::parse(commit_msg_with_newlines, None, None).unwrap();
        let parsed_without_newlines =
            Message::parse(commit_msg_without_newlines, None, None).unwrap();

        assert_eq!(parsed_with_newlines, parsed_without_newlines);
    }

    #[test]
    fn test_parse_with_comments() {
        let msg = "feat: rework footers config to be unified

# Please enter the commit message for your changes. Lines starting
# with '#' will be ignored, and an empty message aborts the commit.
#
# Date:      Sun Jun 8 21:57:34 2025 +0300
#
# On branch feature/unified-footers
# Your branch is up to date with 'origin/feature/unified-footers'.
#
# Changes to be committed:
#       modified:   Cargo.lock";

        let parsed = Message::parse(msg, None, None).unwrap();
        let expected = Message {
            header: Header {
                kind: Some("feat".to_string()),
                scope: Scope::default(),
                description: "rework footers config to be unified".to_string(),
                breaking: false,
            },
            body: None,
            footers: footer_vec![],
        };

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_display() {
        let commit_msg = Message {
            header: Header {
                kind: Some("feat".to_string()),
                scope: Scope::default(),
                description: "my feature".to_string(),
                breaking: false,
            },
            body: Some("Description body".to_string()),
            footers: footer_vec![{
                key: "Authored-By".to_string(),
                value: "John Doe".to_string(),
                separator: ':',
                alignment: SeparatorAlignment::Left,
            }],
        };

        assert_eq!(
            commit_msg.to_string(),
            "feat: my feature\n\nDescription body\n\nAuthored-By: John Doe"
        );
    }

    #[test]
    fn test_display_without_body() {
        let commit_msg = Message {
            header: Header {
                kind: Some("feat".to_string()),
                scope: Scope::default(),
                description: "my feature".to_string(),
                breaking: false,
            },
            body: None,
            footers: footer_vec![{
                key: "Authored-By".to_string(),
                value: "John Doe".to_string(),
                separator: ':',
                alignment: SeparatorAlignment::Left,
            }],
        };

        assert_eq!(commit_msg.to_string(), "feat: my feature\n\nAuthored-By: John Doe");
    }
    #[test]
    fn test_display_without_footers() {
        let commit_msg = Message {
            header: Header {
                kind: Some("feat".to_string()),
                scope: Scope::default(),
                description: "my feature".to_string(),
                breaking: false,
            },
            body: None,
            footers: footer_vec![],
        };

        assert_eq!(commit_msg.to_string(), "feat: my feature");
    }

    #[test]
    fn test_display_without_body_and_footers() {
        let commit_msg = Message {
            header: Header {
                kind: Some("feat".to_string()),
                scope: Scope::default(),
                description: "my feature".to_string(),
                breaking: false,
            },
            body: None,
            footers: footer_vec![],
        };

        assert_eq!(commit_msg.to_string(), "feat: my feature");
    }

    #[test]
    fn test_format() {
        let tests: Vec<(&str, &str)> = vec![
            ("feat: test", "feat: test"),
            ("feat(test): test", "feat(test): test"),
            ("feat(test): test\n\nbody", "feat(test): test\n\nbody"),
            (
                "feat(test): test\n\nbody\n\nAuthored-By: John Doe",
                "feat(test): test\n\nbody\n\nAuthored-By: John Doe",
            ),
        ];

        for (input, expected) in tests {
            let message = Message::parse(input, None, None).unwrap();
            assert_eq!(message.to_string(), expected);
        }
    }
}
