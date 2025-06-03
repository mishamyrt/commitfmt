use std::{io::Write, process::Stdio};

/// Create a pipe filled with the given string.
pub fn pipe_from_string(input: &str) -> Stdio {
    let (reader, mut writer) = std::io::pipe().unwrap();
    let _ = writer.write(input.as_bytes()).unwrap();
    Stdio::from(reader)
}
