use std::{process::Command, str};

use crate::parse::{parse_segment, Segment};
use crate::TplError;

/// Executes a shell command and captures its stdout.
/// Uses `sh -c` on Unix and `cmd /C` on Windows for shell interpretation.
fn execute_command(command_str: &str) -> Result<String, TplError> {
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
                Err(TplError::CommandFailed(String::from_utf8_lossy(&out.stderr).to_string()))
            }
        }
        Err(e) => Err(TplError::CommandIOError(e)),
    }
}

/// Parses the template string, executes commands within {{...}},
/// and replaces them with their standard output.
pub fn render(template: &str) -> Result<String, TplError> {
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
                return Err(TplError::from(e));
            }
            Err(nom::Err::Incomplete(_)) => {
                return Err(TplError::ParseError(
                    "Parsing failed due to incomplete input".to_string(),
                ));
            }
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::{render, TplError};
    use std::process::Command;

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
        let rendered = render(template).expect("Rendering plain literal failed");
        assert_eq!(rendered, template);
    }

    #[test]
    fn test_single_command() {
        // Build a template that contains a single command.
        // We use "echo hello" which should output "hello" after trimming newline.
        let template = "Greeting: {{ echo hello }}";
        let expected_output = format!("Greeting: {}", echo_command("hello"));
        let rendered = render(template).expect("Rendering single command failed");
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
        let rendered = render(template).expect("Rendering multiple commands failed");
        assert_eq!(rendered, expected_output);
    }

    #[test]
    fn test_unclosed_command_block() {
        // A template string with an unclosed command block should fail parsing.
        let template = "This is broken: {{ echo broken ";
        let rendered = render(template);
        match rendered {
            Err(TplError::ParseError(_)) => {} // Expected error variant
            _ => panic!("Expected a ParseError for incomplete command block"),
        }
    }

    #[test]
    fn test_nonexistent_command() {
        // A command that doesn't exist should result in a CommandFailed error.
        // We use a command name that is highly likely to be absent.
        let template = "Will fail: {{ nonexistent_command_xyz }}";
        let rendered = render(template);
        match rendered {
            Err(TplError::CommandFailed(err_msg)) => {
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
