use std::fmt::Display;

use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::{char, space0};
use nom::combinator::{opt, verify};
use nom::multi::separated_list1;
use nom::sequence::{delimited, preceded};
use nom::{IResult, Parser};

/// Scope of a commit is a list of strings
/// Example: (scope1, scope2)
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Scope(pub Vec<Box<str>>);

impl Scope {
    const SEPARATOR_CHAR: char = ',';
    const SEPARATOR_DISPLAY: &str = ", ";

    /// Create a scope from an iterator
    pub fn from<I: IntoIterator<Item = T>, T: Into<Box<str>>>(iter: I) -> Self {
        Self(iter.into_iter().map(std::convert::Into::into).collect())
    }

    /// Parse a list of scopes.
    /// Returns `None` if the input does not contain a valid list of scopes
    /// Scopes format: `(scope1, scope2)`
    pub fn parse(input: &str) -> IResult<&str, Vec<Box<str>>> {
        delimited(
            preceded(space0, char('(')),
            separated_list1(
                preceded(space0, char(Self::SEPARATOR_CHAR)),
                preceded(
                    space0,
                    take_while1(|c: char| {
                        !c.is_whitespace() && c != Self::SEPARATOR_CHAR && c != ')'
                    }),
                ),
            ),
            preceded(space0, char(')')),
        )
        .parse(input)
        .map(|(next_input, scopes)| {
            (next_input, scopes.into_iter().map(std::convert::Into::into).collect())
        })
    }

    /// Returns the number of scopes
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns the number of characters in the scopes.
    /// It's like formatted string length
    pub fn str_len(&self) -> usize {
        if self.0.is_empty() {
            return 0;
        }
        let mut len: usize = 2; // parentheses
        len += Self::SEPARATOR_DISPLAY.len() * (self.0.len() - 1); // comma and space
        len += self.0.iter().map(|c| c.len()).reduce(|a, b| a + b).unwrap_or(0); // scopes
        len
    }

    /// Returns `true` if the are no scopes
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns an iterator over the scopes
    pub fn iter(&self) -> impl Iterator<Item = &Box<str>> {
        self.0.iter()
    }
}

impl Display for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.0.is_empty() {
            write!(f, "(")?;
            write!(f, "{}", self.0.join(Self::SEPARATOR_DISPLAY))?;
            write!(f, ")")?;
        }

        Ok(())
    }
}

/// kind(scope1,scope2)!: description
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Header {
    pub description: String,
    pub kind: Option<String>,
    pub breaking: bool,
    pub scope: Scope,
}

impl Header {
    /// Parse a commit header
    pub fn from(input: &str) -> Self {
        let Ok(result) = (
            Self::parse_kind,
            opt(Scope::parse),
            Self::parse_breaking,
            Self::parse_description,
        )
            .parse(input)
        else {
            return Self {
                kind: None,
                scope: Scope::default(),
                breaking: false,
                description: input.to_string(),
            };
        };

        let (_, (kind, scope, breaking, description)) = result;

        let scope = match scope {
            Some(scopes) => Scope::from(scopes),
            None => Scope::default(),
        };

        Self { kind: Some(kind.to_string()), scope, breaking, description }
    }

    /// Returns the number of characters in the header
    pub fn len(&self) -> usize {
        // Description
        let mut len: usize = self.description.len();

        if let Some(kind) = &self.kind {
            // Kind + colon + space
            len += kind.len() + 2;
        }

        if !self.scope.is_empty() {
            len += self.scope.str_len();
        }

        if self.breaking {
            len += 1;
        }
        len
    }

    /// Returns `true` if the header is empty
    pub fn is_empty(&self) -> bool {
        self.description.len() == 0
    }

    /// Parse a commit kind.
    /// Returns `None` if the input does not contain a valid kind
    fn parse_kind(input: &str) -> IResult<&str, &str> {
        verify(take_while1(|c: char| c.is_alphabetic() || c == '-'), |s: &str| {
            !s.contains(' ')
        })
        .parse(input)
    }

    /// Parse a breaking change indicator
    fn parse_breaking(input: &str) -> IResult<&str, bool> {
        opt(preceded(space0, char('!')))
            .parse(input)
            .map(|(next_input, opt_char)| (next_input, opt_char.is_some()))
    }

    /// Parse a commit description
    fn parse_description(input: &str) -> IResult<&str, String> {
        preceded(preceded(space0, tag(":")), take_while1(|c: char| !c.is_control()))
            .parse(input)
            .map(|(next_input, desc)| (next_input, desc.trim().to_string()))
    }
}

impl std::fmt::Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(kind) = &self.kind {
            write!(f, "{kind}")?;
        }

        if !self.scope.is_empty() {
            write!(f, "{}", self.scope)?;
        }

        if self.breaking {
            write!(f, "!")?;
        }

        if self.kind.is_some() {
            write!(f, ": ")?;
        }

        write!(f, "{}", self.description)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scope_parse() {
        let input = "(scope1,scope2)";
        let result = Scope::parse(input).unwrap().1;
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].as_ref(), "scope1");
        assert_eq!(result[1].as_ref(), "scope2");
    }

    #[test]
    fn test_scope_parse_empty() {
        let inputs = vec!["", "()", "(,)", " "];
        for input in inputs {
            let result = Scope::parse(input);
            assert!(result.is_err());
        }
    }

    #[test]
    fn test_scope_format() {
        let scope = Scope::from(vec!["scope1".to_string(), "scope2".to_string()]);
        assert_eq!(scope.to_string(), "(scope1, scope2)");

        let scope = Scope::from(vec!["scope1".to_string()]);
        assert_eq!(scope.to_string(), "(scope1)");

        let scope = Scope::from::<_, String>(vec![]);
        assert_eq!(scope.to_string(), "");
    }

    #[test]
    fn test_scope_len() {
        let scope = Scope::from(vec!["scope1".to_string(), "scope2".to_string()]);
        assert_eq!(scope.str_len(), scope.to_string().len());

        let scope = Scope::from(vec!["scope1".to_string()]);
        assert_eq!(scope.str_len(), scope.to_string().len());

        let scope = Scope::from::<_, String>(vec![]);
        assert_eq!(scope.str_len(), scope.to_string().len());
    }

    #[test]
    fn test_parse_header() {
        let header = "feat: my feature";
        let parsed = Header::from(header);
        assert_eq!(parsed.kind, Some("feat".to_string()));
        assert_eq!(parsed.scope.len(), 0);
        assert_eq!(parsed.description, "my feature");
    }

    #[test]
    fn test_parse_header_with_scope() {
        let header = "feat(my_scope): my feature";
        let parsed = Header::from(header);
        assert_eq!(parsed.kind, Some("feat".to_string()));
        assert_eq!(parsed.scope.len(), 1);
        assert_eq!(parsed.scope.0[0].as_ref(), "my_scope");
        assert_eq!(parsed.description, "my feature");
    }

    #[test]
    fn test_parse_header_with_breaking_changes() {
        let header = "fix!: my fix";
        let parsed = Header::from(header);
        assert_eq!(parsed.kind, Some("fix".to_string()));
        assert_eq!(parsed.scope.len(), 0);
        assert_eq!(parsed.description, "my fix");
        assert!(parsed.breaking);
    }

    #[test]
    fn test_parse_wrong_formatted_header() {
        let header = "refactor     ( scope_a,    scope_b ) ! : my fix";
        let parsed = Header::from(header);
        assert_eq!(parsed.kind, Some("refactor".to_string()));
        assert_eq!(parsed.scope.len(), 2);
        assert_eq!(parsed.description, "my fix");
        assert!(parsed.breaking);
    }

    #[test]
    fn test_header_as_string() {
        let header = Header::from("feat: my feature");
        assert_eq!(header.to_string(), "feat: my feature");

        let header = Header::from("feat(my_scope): my feature");
        assert_eq!(header.to_string(), "feat(my_scope): my feature");

        let header = Header::from("fix(scope1, scope2)!: my fix");
        assert_eq!(header.to_string(), "fix(scope1, scope2)!: my fix");
    }

    #[test]
    fn test_header_len() {
        let header = Header::from("feat: my feature");
        assert_eq!(header.len(), header.to_string().len());

        let header = Header::from("feat(my_scope): my feature");
        assert_eq!(header.len(), header.to_string().len());

        let header = Header::from("fix(scope1, scope2)!: my fix");
        assert_eq!(header.len(), header.to_string().len());
    }

    #[test]
    fn test_header_is_empty() {
        let header = Header::from("");
        assert!(header.is_empty());

        let header = Header::from("feat: my feature");
        assert!(!header.is_empty());
    }
}
