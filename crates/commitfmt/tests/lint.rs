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

#[test]
fn test_cli_lint_skip_merge() {
    let test_bed = TestBed::empty().unwrap();
    let config_data = r#"
[lint.header]
type-required = true
description-max-length = 5
"#;

    let config_path = test_bed.path().join(".commitfmt.toml");
    std::fs::write(config_path, config_data).unwrap();

    let messages = vec![
        "Merge branch 'main' into test",
        "Merge branches 'feature/auth' and 'feature/payments' into main",
        "Merge tag 'v1.0.0' into main",
        "Merge pull request #123 from feature/auth",
        "Merge commit 'a1b2c3d' into main",
    ];

    let exe = env!("CARGO_BIN_EXE_commitfmt");
    for message in messages {
        let mut cmd = Command::new(exe);
        cmd.stdin(pipe_from_string(message));
        cmd.current_dir(test_bed.path());

        let output = cmd.output().unwrap();
        assert!(output.status.success());

        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains("Skipping merge commit"));
    }
}
