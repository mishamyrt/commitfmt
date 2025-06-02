use std::{
    io::{pipe, Write},
    process::{Command, Stdio},
};

use commitfmt::Commitfmt;
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
    cmd.stdin({
        let (reader, mut writer) = pipe().unwrap();
        let _ = writer.write(input.as_bytes()).unwrap();
        Stdio::from(reader)
    });
    cmd.current_dir(test_bed.path());

    let output = cmd.output().unwrap();
    assert!(output.status.success());

    let result = String::from_utf8(output.stdout).unwrap();

    assert_eq!(result, "feat(test): test\n\nbody\n");
}
