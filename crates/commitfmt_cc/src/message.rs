use thiserror::Error;

use crate::body::parse_body;
use crate::footer::{Footer, Footers};
use crate::header::Header;

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
    pub fn parse(input: &str) -> Result<Self, ParseError> {
        Self::parse_with_separators(input, Footer::DEFAULT_SEPARATOR)
    }

    pub fn parse_with_separators(input: &str, separators: &str) -> Result<Self, ParseError> {
        let header_end = input.find('\n').unwrap_or(input.len());
        let header = Header::from(&input[..header_end]);

        if header_end == input.len() {
            return Ok(Message { header, body: None, footers: Footers::default() });
        }

        let (body, footers) = parse_body(&input[header_end + 1..], separators);

        Ok(Message { header, body, footers: footers.unwrap_or_default() })
    }
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.header)?;
        if let Some(body) = &self.body {
            write!(f, "\n{body}")?;
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

        let parsed = Message::parse(commit_msg);
        let expected = Message {
            header: Header {
                kind: Some("feat".to_string()),
                scope: Scope::default(),
                description: "my feature".to_string(),
                breaking: false,
            },
            body: Some("\nDescription body".to_string()),
            footers: footer_vec![{
                key: "Authored-By".to_string(),
                value: "John Doe".to_string(),
                separator: ':',
                alignment: SeparatorAlignment::Left,
            }],
        };

        match parsed {
            Ok(parsed) => assert_eq!(parsed, expected),
            Err(e) => panic!("Unable to parse commit message: {}", e),
        }
    }

    #[test]
    fn test_parse_without_body() {
        let commit_msg = "feat: my feature\n\nAuthored-By: John Doe\n";

        let parsed = Message::parse(commit_msg).unwrap();
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
    fn test_display() {
        let commit_msg = Message {
            header: Header {
                kind: Some("feat".to_string()),
                scope: Scope::default(),
                description: "my feature".to_string(),
                breaking: false,
            },
            body: Some("\nDescription body".to_string()),
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
}
