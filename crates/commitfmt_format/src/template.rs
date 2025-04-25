use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_until},
    character::complete::multispace0,
    combinator::map,
    error::{Error as NomError, ErrorKind},
    sequence::{delimited, preceded},
    IResult, Parser,
};
use std::{process::Command, str};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TemplateError {
    #[error("IO error during command execution")]
    CommandIOError(#[from] std::io::Error),

    #[error("Command failed with output: {0}")]
    CommandFailed(String),

    #[error("Command produced non-UTF8 output")]
    OutputNotUtf8(#[from] std::str::Utf8Error),

    #[error("Template parsing failed: {0}")]
    ParseError(String),
}

// Helper to convert Nom's error to our custom error type
// Note: This loses some specific Nom context but avoids lifetime complexity.
impl<'a> From<NomError<&'a str>> for TemplateError {
    fn from(err: NomError<&'a str>) -> Self {
        TemplateError::ParseError(format!(
            "Parsing failed near: '{}', error code: {:?}",
            err.input, err.code
        ))
    }
}

// Define segment types we can parse
#[derive(Debug, PartialEq)]
enum Segment<'a> {
    Literal(&'a str),
    Command(&'a str),
}

const TAG_CMD_START: &str = "{{";
const TAG_CMD_END: &str = "}}";

/// Parser for text NOT containing TAG_CMD_START up to the end or next TAG_CMD_END
fn parse_literal(input: &str) -> IResult<&str, Segment, NomError<&str>> {
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
fn parse_command(input: &str) -> IResult<&str, Segment, NomError<&str>> {
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
fn parse_segment(input: &str) -> IResult<&str, Segment, NomError<&str>> {
    alt((parse_command, parse_literal)).parse(input)
}

/// Executes a shell command and captures its stdout.
/// Uses `sh -c` on Unix and `cmd /C` on Windows for shell interpretation.
fn execute_command(command_str: &str) -> Result<String, TemplateError> {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd").arg("/C").arg(command_str).output()
    } else {
        Command::new("sh").arg("-c").arg(command_str).output()
    };

    match output {
        Ok(out) => {
            if out.status.success() {
                // Trim trailing newline often added by commands
                Ok(str::from_utf8(&out.stdout)?.trim_end_matches('\n').to_string())
            } else {
                Err(TemplateError::CommandFailed(
                    String::from_utf8_lossy(&out.stderr).to_string(),
                ))
            }
        }
        Err(e) => Err(TemplateError::CommandIOError(e)),
    }
}

/// Parses the template string, executes commands within {{...}},
/// and replaces them with their standard output.
pub(crate) fn render_template(template: &str) -> Result<String, TemplateError> {
    let mut result = String::new();
    let mut remaining_input = template;

    while !remaining_input.is_empty() {
        match parse_segment(remaining_input) {
            Ok((next_input, segment)) => {
                match segment {
                    Segment::Literal(text) => {
                        result.push_str(text);
                    }
                    Segment::Command(cmd) => {
                        let cmd_output = execute_command(cmd)?;
                        result.push_str(&cmd_output);
                    }
                }
                remaining_input = next_input;
            }
            Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => {
                return Err(TemplateError::from(e));
            }
            Err(nom::Err::Incomplete(_)) => {
                return Err(TemplateError::ParseError(
                    "Parsing failed due to incomplete input".to_string(),
                ));
            }
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

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

    // Helper function to determine if we're on Windows.
    fn is_windows() -> bool {
        cfg!(target_os = "windows")
    }

    // For testing commands, we use a simple command that echoes a known string.
    // On Windows, "echo" is available via cmd, and on Unix via sh.
    fn echo_command(input: &str) -> String {
        // Use Command to capture output. Note: this is only for test expectation.
        if is_windows() {
            Command::new("cmd")
                .args(&["/C", "echo", input])
                .output()
                .map(|o| {
                    // echo on windows typically appends a newline, so trim it.
                    String::from_utf8_lossy(&o.stdout)
                        .trim_end_matches('\r')
                        .trim_end_matches('\n')
                        .to_string()
                })
                .unwrap_or_else(|_| "".to_string())
        } else {
            Command::new("sh")
                .args(&["-c", &format!("echo {}", input)])
                .output()
                .map(|o| {
                    // echo on unix appends a newline, so trim it.
                    String::from_utf8_lossy(&o.stdout).trim_end_matches('\n').to_string()
                })
                .unwrap_or_else(|_| "".to_string())
        }
    }

    #[test]
    fn test_plain_literal() {
        // A template without any commands should be returned as-is.
        let template = "Hello, world!";
        let rendered = render_template(template).expect("Rendering plain literal failed");
        assert_eq!(rendered, template);
    }

    #[test]
    fn test_single_command() {
        // Build a template that contains a single command.
        // We use "echo hello" which should output "hello" after trimming newline.
        let template = "Greeting: {{ echo hello }}";
        let expected_output = format!("Greeting: {}", echo_command("hello"));
        let rendered = render_template(template).expect("Rendering single command failed");
        assert_eq!(rendered, expected_output);
    }

    #[test]
    fn test_multiple_commands() {
        // Template with multiple commands interspersed with literal text.
        // Use echo commands to output known strings.
        let template = "A: {{ echo a }}, B: {{ echo b }}, and C: {{   echo    c   }}";
        let expected_output = format!(
            "A: {}, B: {}, and C: {}",
            echo_command("a"),
            echo_command("b"),
            echo_command("c"),
        );
        let rendered = render_template(template).expect("Rendering multiple commands failed");
        assert_eq!(rendered, expected_output);
    }

    #[test]
    fn test_unclosed_command_block() {
        // A template string with an unclosed command block should fail parsing.
        let template = "This is broken: {{ echo broken ";
        let rendered = render_template(template);
        match rendered {
            Err(TemplateError::ParseError(_)) => {} // Expected error variant
            _ => panic!("Expected a ParseError for incomplete command block"),
        }
    }

    #[test]
    fn test_nonexistent_command() {
        // A command that doesn't exist should result in a CommandFailed error.
        // We use a command name that is highly likely to be absent.
        let template = "Will fail: {{ nonexistent_command_xyz }}";
        let rendered = render_template(template);
        match rendered {
            Err(TemplateError::CommandFailed(err_msg)) => {
                // Ensure that stderr output is captured (the message may differ per OS).
                assert!(!err_msg.is_empty(), "Error message should not be empty");
            }
            Err(e) => {
                panic!("Expected CommandFailed error, got: {:?}", e);
            }
            Ok(output) => {
                panic!("Expected failure, but got output: {}", output);
            }
        }
    }
}
