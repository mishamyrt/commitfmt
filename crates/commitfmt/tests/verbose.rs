use std::process::Command;

use commitfmt::testing::pipe_from_string;

#[test]
fn test_verbose() {
    let exe = env!("CARGO_BIN_EXE_commitfmt");
    let input = "feat(test): test";

    let output = Command::new(exe).stdin(pipe_from_string(input)).output().unwrap();
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).unwrap();
    let non_verbose_count = stdout.lines().count();

    let output =
        Command::new(exe).arg("--verbose").stdin(pipe_from_string(input)).output().unwrap();
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).unwrap();
    let verbose_count = stdout.lines().count();

    assert!(verbose_count > non_verbose_count);
}
