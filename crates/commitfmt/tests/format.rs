use std::process::{Command, Stdio};

use commitfmt::{testing::pipe_from_string, Commitfmt, Error};
use commitfmt_git::testing::TestBed;

fn app_with_config(config: &str) -> (TestBed, Commitfmt) {
    let test_bed = TestBed::empty().unwrap();
    std::fs::write(test_bed.path().join(".commitfmt.toml"), config).unwrap();
    let app = Commitfmt::from_path(&test_bed.path()).unwrap();
    (test_bed, app)
}

#[test]
fn test_format_default() {
    testing_logger::setup();
    let input = "
feat  (  test   ): test
body
"
    .trim();
    let test_bed = TestBed::empty().unwrap();
    let app = Commitfmt::from_path(&test_bed.path()).unwrap();

    let result = app.format_commit_message(input);

    assert!(result.is_ok());

    assert_eq!(result.unwrap(), "feat(test): test\n\nbody");
}

#[test]
fn test_format_applies_safe_fix() {
    let test_bed = TestBed::empty().unwrap();
    let app = Commitfmt::from_path(&test_bed.path()).unwrap();

    let result = app.format_commit_message("feat: description.").unwrap();

    assert_eq!(result, "feat: description");
}

#[test]
fn test_format_rejects_disabled_unsafe_fix() {
    let (_test_bed, app) = app_with_config(
        r#"
[lint.body]
full-stop = true
"#,
    );

    let result = app.format_commit_message("feat: description\n\nBody");

    assert!(matches!(result, Err(Error::Unfixable(1))));
}

#[test]
fn test_format_applies_enabled_unsafe_fix() {
    let (_test_bed, app) = app_with_config(
        r#"
[lint]
unsafe-fixes = true

[lint.body]
full-stop = true
"#,
    );

    let result = app.format_commit_message("feat: description\n\nBody").unwrap();

    assert_eq!(result, "feat: description\n\nBody.");
}

#[test]
fn test_format_skips_existing_footer_by_default() {
    let (_test_bed, app) = app_with_config(
        r#"
[[additional-footers]]
key = "Ticket-ID"
value = "NEW-123"
"#,
    );

    let result = app.format_commit_message("feat: description\n\nTicket-ID: OLD-123").unwrap();

    assert_eq!(result, "feat: description\n\nTicket-ID: OLD-123");
}

#[test]
fn test_format_appends_existing_footer_when_configured() {
    let (_test_bed, app) = app_with_config(
        r#"
[[additional-footers]]
key = "Ticket-ID"
value = "NEW-123"
on-conflict = "append"
"#,
    );

    let result = app.format_commit_message("feat: description\n\nTicket-ID: OLD-123").unwrap();

    assert_eq!(result, "feat: description\n\nTicket-ID: OLD-123\nTicket-ID: NEW-123");
}

#[test]
fn test_format_rejects_existing_footer_when_configured() {
    let (_test_bed, app) = app_with_config(
        r#"
[[additional-footers]]
key = "Ticket-ID"
value = "NEW-123"
on-conflict = "error"
"#,
    );

    let result = app.format_commit_message("feat: description\n\nTicket-ID: OLD-123");

    assert!(matches!(result, Err(Error::AlreadyExists(key)) if key == "Ticket-ID"));
}

#[test]
fn test_format_skips_footer_when_branch_does_not_match() {
    let (_test_bed, app) = app_with_config(
        r#"
[[additional-footers]]
key = "Ticket-ID"
branch-pattern = "^feature/(?<TICKET_ID>[A-Z0-9-]+)$"
value = "${{ TICKET_ID }}"
"#,
    );

    let result = app.format_commit_message("feat: description").unwrap();

    assert_eq!(result, "feat: description");
}

#[test]
fn test_cli_format_default_stdin() {
    let input = "
feat  (  test   ): test
body
"
    .trim();

    let test_bed = TestBed::empty().unwrap();
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

    let test_bed = TestBed::with_default_history().unwrap();
    let exe = env!("CARGO_BIN_EXE_commitfmt");

    test_bed.repo.write_commit_message(input).unwrap();

    let mut cmd = Command::new(exe);
    cmd.stdin(Stdio::null());
    cmd.current_dir(test_bed.path());

    let output = cmd.output().unwrap();
    assert!(output.status.success());

    let output = test_bed.repo.read_commit_message().unwrap();
    assert_eq!(output, "feat(test): test\n\nbody\n\nfooter-key: value");
}
