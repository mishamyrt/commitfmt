use std::{collections::HashMap, process::Command};

use crate::{Error, Result};
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::multispace0,
    combinator::map,
    error::{Error as NomError, ErrorKind},
    sequence::{delimited, preceded},
    IResult, Parser,
};

const TAG_VARIABLE_START: &str = "${{";
const TAG_CMD_START: &str = "{{";
const TAG_END: &str = "}}";

/// A segment of a template
#[derive(Debug, Clone, PartialEq)]
pub enum Segment {
    Literal(String),
    Command(String),
    Variable(String),
}

/// A template is a sequence of segments
#[derive(Debug, Clone, PartialEq)]
pub struct Template {
    segments: Vec<Segment>,
}

impl Template {
    /// Parse the template from the given input
    ///
    /// # Errors
    ///
    /// Returns an error if the template is invalid
    pub fn parse(input: &str) -> Result<Self> {
        let mut remaining_input = input;

        let mut segments = Vec::new();

        while !remaining_input.is_empty() {
            match parse_segment(remaining_input) {
                Ok((next_input, segment)) => {
                    segments.push(segment);
                    remaining_input = next_input;
                }
                Err(nom::Err::Error(e) | nom::Err::Failure(e)) => {
                    return Err(Error::ParseError(e.to_string()));
                }
                Err(nom::Err::Incomplete(_)) => return Err(Error::UnclosedTag),
            }
        }

        Ok(Template { segments })
    }

    /// Get the number of segments in the template
    pub fn segments_len(&self) -> usize {
        self.segments.len()
    }

    /// Iterate over the segments of the template
    pub fn segments_iter(&self) -> impl Iterator<Item = &Segment> {
        self.segments.iter()
    }
}

impl Template {
    /// Render the template with the given variables
    ///
    /// # Errors
    ///
    /// Returns an error if the template contains an undefined variable or if the command fails
    pub fn render(&self, variables: &HashMap<String, String>) -> Result<String> {
        let mut result = String::new();
        for segment in &self.segments {
            match segment {
                Segment::Literal(literal) => result.push_str(literal),
                Segment::Command(command) => {
                    result.push_str(&execute_command(command)?);
                }
                Segment::Variable(variable) => {
                    // Check local variables first
                    if let Some(value) = variables.get(variable) {
                        result.push_str(value);
                    // Check environment variables
                    } else if let Ok(value) = std::env::var(variable) {
                        result.push_str(&value);
                    } else {
                        return Err(Error::UndefinedVariable(variable.clone()));
                    }
                }
            }
        }
        Ok(result)
    }
}

fn execute_command(command: &str) -> Result<String> {
    let output = Command::new("sh").arg("-c").arg(command).output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Parser for text NOT containing tags
fn parse_literal(input: &str) -> IResult<&str, Segment, NomError<&str>> {
    if input.is_empty() {
        return Err(nom::Err::Error(NomError::new(input, ErrorKind::TakeUntil)));
    }

    // Find the nearest tag start (either ${{ or {{)
    let var_pos = input.find(TAG_VARIABLE_START);
    let cmd_pos = input.find(TAG_CMD_START);

    let nearest_pos = match (var_pos, cmd_pos) {
        (Some(v), Some(c)) => Some(v.min(c)),
        (Some(v), None) => Some(v),
        (None, Some(c)) => Some(c),
        (None, None) => None,
    };

    match nearest_pos {
        Some(0) => Err(nom::Err::Error(NomError::new(input, ErrorKind::IsNot))),
        Some(pos) => {
            let literal = &input[..pos];
            let remaining = &input[pos..];
            Ok((remaining, Segment::Literal(literal.to_string())))
        }
        None => Ok(("", Segment::Literal(input.to_string()))),
    }
}

/// Parser for command segments
/// Example: `{{ echo 'hello' }}`
fn parse_command(input: &str) -> IResult<&str, Segment, NomError<&str>> {
    map(
        delimited(tag(TAG_CMD_START), preceded(multispace0, is_not(TAG_END)), tag(TAG_END)),
        |cmd: &str| Segment::Command(cmd.trim().to_string()),
    )
    .parse(input)
}

/// Parser for variable segments
/// Example: `${{ TASK_ID }}`
fn parse_variable(input: &str) -> IResult<&str, Segment, NomError<&str>> {
    map(delimited(tag(TAG_VARIABLE_START), is_not(TAG_END), tag(TAG_END)), |var: &str| {
        Segment::Variable(var.trim().to_string())
    })
    .parse(input)
}

/// Parser for either a literal or a command block
fn parse_segment(input: &str) -> IResult<&str, Segment, NomError<&str>> {
    alt((parse_command, parse_variable, parse_literal)).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_parse_literal() {
        let template = Template::parse("hello world").unwrap();
        assert_eq!(template.segments_len(), 1);
        let segments: Vec<&Segment> = template.segments_iter().collect();
        assert_eq!(segments[0], &Segment::Literal("hello world".to_string()));
    }

    #[test]
    fn test_parse_variable() {
        let template = Template::parse("Hello ${{ NAME }}").unwrap();
        assert_eq!(template.segments_len(), 2);
        let segments: Vec<&Segment> = template.segments_iter().collect();
        assert_eq!(segments[0], &Segment::Literal("Hello ".to_string()));
        assert_eq!(segments[1], &Segment::Variable("NAME".to_string()));
    }

    #[test]
    fn test_parse_command() {
        let template = Template::parse("Date: {{ date }}").unwrap();
        assert_eq!(template.segments_len(), 2);
        let segments: Vec<&Segment> = template.segments_iter().collect();
        assert_eq!(segments[0], &Segment::Literal("Date: ".to_string()));
        assert_eq!(segments[1], &Segment::Command("date".to_string()));
    }

    #[test]
    fn test_parse_mixed_template() {
        let template = Template::parse("Hello ${{ NAME }}, today is {{ date }}!").unwrap();
        assert_eq!(template.segments_len(), 5);
        let segments: Vec<&Segment> = template.segments_iter().collect();
        assert_eq!(segments[0], &Segment::Literal("Hello ".to_string()));
        assert_eq!(segments[1], &Segment::Variable("NAME".to_string()));
        assert_eq!(segments[2], &Segment::Literal(", today is ".to_string()));
        assert_eq!(segments[3], &Segment::Command("date".to_string()));
        assert_eq!(segments[4], &Segment::Literal("!".to_string()));
    }

    #[test]
    fn test_parse_command_with_whitespace() {
        let template = Template::parse("{{   echo hello   }}").unwrap();
        assert_eq!(template.segments_len(), 1);
        let segments: Vec<&Segment> = template.segments_iter().collect();
        assert_eq!(segments[0], &Segment::Command("echo hello".to_string()));
    }

    #[test]
    fn test_parse_variable_with_whitespace() {
        let template = Template::parse("${{   USER_NAME   }}").unwrap();
        assert_eq!(template.segments_len(), 1);
        let segments: Vec<&Segment> = template.segments_iter().collect();
        assert_eq!(segments[0], &Segment::Variable("USER_NAME".to_string()));
    }

    #[test]
    fn test_parse_empty_string() {
        let template = Template::parse("").unwrap();
        assert_eq!(template.segments_len(), 0);
    }

    #[test]
    fn test_parse_unclosed_variable() {
        let result = Template::parse("Hello ${{ NAME");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_unclosed_command() {
        let result = Template::parse("Date: {{ date");
        assert!(result.is_err());
    }

    #[test]
    fn test_render_literal() {
        let template = Template::parse("hello world").unwrap();
        let vars = HashMap::new();
        let result = template.render(&vars).unwrap();
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_render_variable_from_locals() {
        let template = Template::parse("Hello ${{ NAME }}").unwrap();
        let mut vars = HashMap::new();
        vars.insert("NAME".to_string(), "John".to_string());
        let result = template.render(&vars).unwrap();
        assert_eq!(result, "Hello John");
    }

    #[test]
    fn test_render_variable_from_env() {
        std::env::set_var("TEST_VAR", "test_value");
        let template = Template::parse("Value: ${{ TEST_VAR }}").unwrap();
        let vars = HashMap::new();
        let result = template.render(&vars).unwrap();
        assert_eq!(result, "Value: test_value");
        std::env::remove_var("TEST_VAR");
    }

    #[test]
    fn test_render_command() {
        let template = Template::parse("{{ echo hello }}").unwrap();
        let vars = HashMap::new();
        let result = template.render(&vars).unwrap();
        assert_eq!(result.trim(), "hello");
    }

    #[test]
    fn test_render_mixed_template() {
        let template =
            Template::parse("Hello ${{ NAME }}, result: {{ echo success }}").unwrap();
        let mut vars = HashMap::new();
        vars.insert("NAME".to_string(), "Alice".to_string());
        let result = template.render(&vars).unwrap();
        assert_eq!(result.trim(), "Hello Alice, result: success");
    }

    #[test]
    fn test_render_undefined_variable() {
        let template = Template::parse("Hello ${{ UNDEFINED }}").unwrap();
        let vars = HashMap::new();
        let result = template.render(&vars);
        assert!(result.is_err());
        match result.unwrap_err() {
            crate::Error::UndefinedVariable(var) => assert_eq!(var, "UNDEFINED"),
            _ => panic!("Expected UndefinedVariable error"),
        }
    }

    #[test]
    fn test_render_locals_override_env() {
        std::env::set_var("OVERRIDE_TEST", "env_value");
        let template = Template::parse("Value: ${{ OVERRIDE_TEST }}").unwrap();
        let mut vars = HashMap::new();
        vars.insert("OVERRIDE_TEST".to_string(), "local_value".to_string());
        let result = template.render(&vars).unwrap();
        assert_eq!(result, "Value: local_value");
        std::env::remove_var("OVERRIDE_TEST");
    }

    #[test]
    fn test_render_complex_command() {
        let template = Template::parse("Number: {{ echo 42 }}").unwrap();
        let vars = HashMap::new();
        let result = template.render(&vars).unwrap();
        // Check that result is not empty and contains expected value
        assert!(!result.trim().is_empty());
        assert_eq!(result.trim(), "Number: 42");
    }

    #[test]
    fn test_multiple_consecutive_variables() {
        let template = Template::parse("${{ A }}${{ B }}${{ C }}").unwrap();
        let mut vars = HashMap::new();
        vars.insert("A".to_string(), "1".to_string());
        vars.insert("B".to_string(), "2".to_string());
        vars.insert("C".to_string(), "3".to_string());
        let result = template.render(&vars).unwrap();
        assert_eq!(result, "123");
    }

    #[test]
    fn test_multiple_consecutive_commands() {
        let template = Template::parse("{{ echo hello }}{{ echo world }}").unwrap();
        let vars = HashMap::new();
        let result = template.render(&vars).unwrap();
        // Remove newlines since echo adds them
        let result = result.replace('\n', "");
        assert_eq!(result, "helloworld");
    }

    #[test]
    fn test_command_with_arguments() {
        let template = Template::parse("{{ echo hello world }}").unwrap();
        let vars = HashMap::new();
        let result = template.render(&vars).unwrap();
        assert_eq!(result.trim(), "hello world");
    }
}
