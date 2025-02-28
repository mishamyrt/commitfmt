use crate::{
    body::parse_body,
    footer::{Footer, DEFAULT_SEPARATOR},
    header::Header,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Unable to parse commit message")]
    InvalidCommitMessage(String),
}

/// Commit message
#[derive(Debug, PartialEq)]
pub struct Message {
    pub header: Header,
    pub body: Option<String>,
    pub footers: Vec<Footer>,
}

impl Message {
    pub fn parse(input: &str) -> Result<Self, ParseError> {
        Self::parse_with_separators(input, DEFAULT_SEPARATOR)
    }

    pub fn parse_with_separators(input: &str, separators: &str) -> Result<Self, ParseError> {
        let header_end = input.find('\n').unwrap_or(input.len());
        let header = Header::from(&input[..header_end]);

        if header_end == input.len() {
            return Ok(Message {
                header,
                body: None,
                footers: vec![],
            });
        }

        let (body, footers) = parse_body(&input[header_end + 1..], separators);

        Ok(Message {
            header,
            body,
            footers: footers.unwrap_or_default(),
        })
    }
}

#[cfg(test)]
mod tests {
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
                scope: vec![],
                description: " my feature".to_string(),
                breaking: false,
            },
            body: Some("\nDescription body".to_string()),
            footers: vec![Footer {
                key: "Authored-By".to_string(),
                value: "John Doe".to_string(),
            }],
        };

        match parsed {
            Ok(parsed) => assert_eq!(parsed, expected),
            Err(e) => panic!("Unable to parse commit message: {}", e),
        }
    }

    #[test]
    fn test_parse_without_body() {
        let commit_msg = "feat: my feature

Authored-By: John Doe";

        let parsed = Message::parse(commit_msg);
        let expected = Message {
            header: Header {
                kind: Some("feat".to_string()),
                scope: vec![],
                description: " my feature".to_string(),
                breaking: false,
            },
            body: None,
            footers: vec![Footer {
                key: "Authored-By".to_string(),
                value: "John Doe".to_string(),
            }],
        };

        match parsed {
            Ok(parsed) => assert_eq!(parsed, expected),
            Err(e) => panic!("Unable to parse commit message: {}", e),
        }
    }
}
