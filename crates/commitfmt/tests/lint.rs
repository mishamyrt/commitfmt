use commitfmt::testing::pipe_from_string;
use commitfmt_git::testing::TestBed;
use insta::assert_snapshot;
use std::process::Command;

#[test]
fn test_cli_lint_configured() {
    let test_bed = TestBed::empty().unwrap();
    let input = "
Feat  (  Test   ): Test

body text and more text

Footer: value
BREAKING CHANGES: test
"
    .trim();

    let config = r#"
[lint.header]
type-case = "lower"
description-case = "lower-first"
scope-min-length = 5
scope-case = "kebab"
min-length = 18
type-max-length = 3

[lint.body]
case = "upper-first"
full-stop = true
max-line-length = 20
max-length = 20

[lint.footer]
key-case = "lower"
min-length = 15
max-length = 20
exists = ["Issue-ID"]
"#;
    std::fs::write(test_bed.path().join("commitfmt.toml"), config).unwrap();

    let exe = env!("CARGO_BIN_EXE_commitfmt");

    let mut cmd = Command::new(exe);
    cmd.arg("--lint");
    cmd.stdin(pipe_from_string(input));
    cmd.current_dir(test_bed.path());

    let output = cmd.output().unwrap();
    assert!(!output.status.success());

    let result = String::from_utf8(output.stdout).unwrap();

    assert_snapshot!("cli_configured", result);
}
