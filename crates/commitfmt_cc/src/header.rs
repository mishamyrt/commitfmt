use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::{char, space0};
use nom::combinator::{opt, verify};
use nom::multi::separated_list0;
use nom::sequence::{delimited, preceded};
use nom::{IResult, Parser};

/// kind(scope1,scope2)!: description
#[derive(Debug, PartialEq)]
pub struct Header {
    pub description: String,
    pub kind: Option<String>,
    pub breaking: bool,
    pub scope: Vec<String>,
}

impl Header {
    fn is_valid_kind_char(c: char) -> bool {
        c.is_alphabetic()
    }

    fn parse_kind(input: &str) -> IResult<&str, &str> {
        verify(take_while1(Self::is_valid_kind_char), |s: &str| !s.contains(' ')).parse(input)
    }

    fn parse_scopes(input: &str) -> IResult<&str, Vec<String>> {
        delimited(
            preceded(space0, char('(')),
            separated_list0(
                preceded(space0, char(',')),
                preceded(space0, take_while1(|c: char| !c.is_whitespace() && c != ',' && c != ')')),
            ),
            preceded(space0, char(')')),
        )
        .parse(input)
        .map(|(next_input, scopes)| (next_input, scopes.into_iter().map(String::from).collect()))
    }

    fn parse_breaking(input: &str) -> IResult<&str, bool> {
        opt(preceded(space0, char('!'))).parse(input).map(|(next_input, opt_char)| (next_input, opt_char.is_some()))
    }

    fn parse_description(input: &str) -> IResult<&str, String> {
        preceded(preceded(space0, tag(":")), take_while1(|c: char| !c.is_control()))
            .parse(input)
            .map(|(next_input, desc)| (next_input, desc.to_string()))
    }

    pub fn from(input: &str) -> Self {
        let Ok(result) =
            (Self::parse_kind, opt(Self::parse_scopes), Self::parse_breaking, Self::parse_description).parse(input)
        else {
            return Self {
                kind: None,
                scope: Vec::new(),
                breaking: false,
                description: input.to_string(),
            };
        };

        let (_, (kind, scope, breaking, description)) = result;

        Self {
            kind: Some(kind.to_string()),
            scope: scope.unwrap_or_else(Vec::new),
            breaking,
            description,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_header() {
        let header = "feat: my feature";
        let parsed = Header::from(header);
        assert_eq!(parsed.kind, Some("feat".to_string()));
        assert_eq!(parsed.scope.len(), 0);
        assert_eq!(parsed.description, " my feature");
    }

    #[test]
    fn test_parse_header_with_scope() {
        let header = "feat(my_scope): my feature";
        let parsed = Header::from(header);
        assert_eq!(parsed.kind, Some("feat".to_string()));
        assert_eq!(parsed.scope.len(), 1);
        assert_eq!(parsed.scope[0], "my_scope".to_string());
        assert_eq!(parsed.description, " my feature");
    }

    #[test]
    fn test_parse_header_with_breaking_changes() {
        let header = "fix!: my fix";
        let parsed = Header::from(header);
        assert_eq!(parsed.kind, Some("fix".to_string()));
        assert_eq!(parsed.scope.len(), 0);
        assert_eq!(parsed.description, " my fix");
        assert_eq!(parsed.breaking, true);
    }

    #[test]
    fn test_parse_wrong_formatted_header() {
        let header = "refactor     ( scope_a,    scope_b ) ! : my fix";
        let parsed = Header::from(header);
        assert_eq!(parsed.kind, Some("refactor".to_string()));
        assert_eq!(parsed.scope.len(), 2);
        assert_eq!(parsed.description, " my fix");
        assert_eq!(parsed.breaking, true);
    }
}
