use std::process::Command;

use commitfmt::Commitfmt;
use commitfmt_git::testing::TestBed;
use insta::assert_snapshot;

#[test]
fn test_lint_correct_range() {
    testing_logger::setup();

    let test_bed = TestBed::new_with_history().unwrap();
    let commitfmt = Commitfmt::from_path(&test_bed.path()).unwrap();
    let range = ("HEAD~3", "HEAD");
    let result = commitfmt.lint_commit_range(range);
    assert!(result.is_ok());

    testing_logger::validate(|captured_logs| {
        assert_eq!(captured_logs.len(), 1);
        assert_snapshot!(captured_logs[0].body, @"No problems found in 3 commits");
    });
}

#[test]
fn test_lint_incorrect_range() {
    let incorrect_commits =
        vec!["chore: initial commit", "fea: test.", "feat(tes): test", "feat(core): test."];
    let config_data = r#"
[lint.header]
scope-enum = ["core", "api"]
type-enum = ["feat", "fix", "chore"]
description-min-length = 10
# And description-full-stop enabled by default
"#;
    let test_bed = TestBed::new_with_commits(&incorrect_commits).unwrap();

    let config_path = test_bed.path().join(".commitfmt.toml");
    std::fs::write(config_path, config_data).unwrap();

    let commitfmt = Commitfmt::from_path(&test_bed.path()).unwrap();

    colored::control::set_override(false);
    testing_logger::setup();

    let range = ("HEAD~3", "HEAD");
    let result = commitfmt.lint_commit_range(range);
    assert!(result.is_err());

    testing_logger::validate(|captured_logs| {
        assert_eq!(captured_logs.len(), 10);

        assert_snapshot!(captured_logs[1].body, @"- Header description is ended with a full stop [description-full-stop]");
        assert_snapshot!(captured_logs[2].body, @"- Description is shorter than 10 characters [description-min-length]");

        assert_snapshot!(captured_logs[4].body, @"- Scope is not allowed: tes [scope-enum]");
        assert_snapshot!(captured_logs[5].body, @"- Description is shorter than 10 characters [description-min-length]");

        assert_snapshot!(captured_logs[7].body, @"- Type is not allowed: fea [type-enum]");
        assert_snapshot!(captured_logs[8].body, @"- Header description is ended with a full stop [description-full-stop]");
        assert_snapshot!(captured_logs[9].body, @"- Description is shorter than 10 characters [description-min-length]");
    });
}

#[test]
fn test_cli_lint_correct_range() {
    let exe = env!("CARGO_BIN_EXE_commitfmt");

    let test_bed = TestBed::new_with_history().unwrap();

    let mut cmd = Command::new(exe);
    cmd.arg("--from").arg("HEAD~3").arg("--to").arg("HEAD");
    cmd.current_dir(test_bed.path());

    let output = cmd.output().unwrap();
    assert!(output.status.success());

    let commitfmt = Commitfmt::from_path(&test_bed.path()).unwrap();
    let range = ("HEAD~3", "HEAD");
    let result = commitfmt.lint_commit_range(range);
    assert!(result.is_ok());
}

#[test]
fn test_cli_lint_incorrect_range() {
    let exe = env!("CARGO_BIN_EXE_commitfmt");

    let incorrect_commits = vec!["chore: initial commit", "another commit", "feat(tes): test"];
    let config_data = r#"
[lint.header]
type-required = true
scope-required = true
description-max-length = 5
# And description-full-stop enabled by default
"#;
    let test_bed = TestBed::new_with_commits(&incorrect_commits).unwrap();

    let config_path = test_bed.path().join(".commitfmt.toml");
    std::fs::write(config_path, config_data).unwrap();

    let mut cmd = Command::new(exe);
    cmd.arg("--from").arg("HEAD~2").arg("--to").arg("HEAD");
    cmd.current_dir(test_bed.path());

    let output = cmd.output().unwrap();
    assert!(!output.status.success());

    let output_lines = String::from_utf8(output.stdout).unwrap();
    let output_lines = output_lines.lines().collect::<Vec<&str>>();

    assert_eq!(output_lines.len(), 5);

    assert_snapshot!(output_lines[1], @"- Commit type is required [type-required]");
    assert_snapshot!(output_lines[2], @"- Scope is required [scope-required]");
    assert_snapshot!(output_lines[3], @"- Description is longer than 5 characters [description-max-length]");
}

#[test]
fn test_cli_only_to() {
    let exe = env!("CARGO_BIN_EXE_commitfmt");

    let test_bed = TestBed::new_with_history().unwrap();

    let mut cmd = Command::new(exe);
    cmd.arg("--to").arg("HEAD");
    cmd.current_dir(test_bed.path());

    let output = cmd.output().unwrap();
    assert!(!output.status.success());

    let output_text = String::from_utf8(output.stdout).unwrap();
    let output_lines = output_text.lines().collect::<Vec<&str>>();

    assert_eq!(output_lines.len(), 1);
    assert_snapshot!(output_lines[0], @"--to requires --from");
}
