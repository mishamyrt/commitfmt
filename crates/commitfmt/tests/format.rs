use std::process::Command;

use commitfmt::{testing::pipe_from_string, Commitfmt};
use commitfmt_git::testing::TestBed;

#[test]
fn test_format_default() {
    testing_logger::setup();
    let input = "
feat  (  test   ): test
body
"
    .trim();
    let test_bed = TestBed::new().unwrap();
    let app = Commitfmt::from_path(&test_bed.path()).unwrap();

    let result = app.format_commit_message(input, false);

    assert!(result.is_ok());

    assert_eq!(result.unwrap(), "feat(test): test\n\nbody");
}

#[test]
fn test_cli_format_default_stdin() {
    let input = "
feat  (  test   ): test
body
"
    .trim();

    let test_bed = TestBed::new().unwrap();
    let exe = env!("CARGO_BIN_EXE_commitfmt");

    let mut cmd = Command::new(exe);
    cmd.stdin(pipe_from_string(input));
    cmd.current_dir(test_bed.path());

    let output = cmd.output().unwrap();
    assert!(output.status.success());

    let result = String::from_utf8(output.stdout).unwrap();

    assert_eq!(result, "feat(test): test\n\nbody\n");
}

#[test]
fn test_cli_format_default_commit() {
    let input = "
feat  (  test   ): test
body

footer-key: value
"
    .trim();

    let test_bed = TestBed::new_with_history().unwrap();
    let exe = env!("CARGO_BIN_EXE_commitfmt");

    test_bed.repo.write_commit_message(input).unwrap();

    let mut cmd = Command::new(exe);
    cmd.current_dir(test_bed.path());

    let output = cmd.output().unwrap();
    assert!(output.status.success());
}
