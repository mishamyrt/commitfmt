use std::fs::Permissions;

use commitfmt_git::testing::TestBed;
use insta::assert_snapshot;

fn write_hook(test_bed: &TestBed) {
    let exe: &str = env!("CARGO_BIN_EXE_commitfmt");
    let hook_path = test_bed.path().join(".git/hooks/prepare-commit-msg");

    std::fs::write(&hook_path, format!("#!/bin/sh\n\n{exe}")).unwrap();

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&hook_path, Permissions::from_mode(0o755)).unwrap();
    }
}

#[test]
fn test_hook_default() {
    let test_bed = TestBed::with_default_history().unwrap();
    let repo = test_bed.repo.clone();

    write_hook(&test_bed);

    repo.commit("feat(   test   ) : test").unwrap();

    let log = repo.get_log("HEAD~1", "HEAD").unwrap();
    assert_eq!(log[0].message, "feat(test): test\n");
}

#[test]
fn test_hook_with_config_correct_message() {
    let test_bed = TestBed::with_default_history().unwrap();
    let repo = test_bed.repo.clone();
    let config = r#"
[lint.header]
scope-enum = ["api", "core"]
type-enum = ["feat", "fix"]

[lint.footer]
exists = ["Issue-ID"]
"#;
    std::fs::write(test_bed.path().join(".commitfmt.toml"), config).unwrap();
    write_hook(&test_bed);

    repo.commit("feat (  api   ) : test\nbody\n\nIssue-ID: 123").unwrap();

    let log = repo.get_log("HEAD~1", "HEAD").unwrap();
    assert_eq!(log[0].message, "feat(api): test\n\nbody\n\nIssue-ID: 123\n");
}

#[test]
fn test_hook_with_config_incorrect_message() {
    let test_bed = TestBed::with_default_history().unwrap();
    let repo = test_bed.repo.clone();
    let config = r#"
[lint.header]
type-min-length = 4
max-length = 30
scope-max-length = 10

[lint.body]
min-length = 10

[lint.footer]
max-line-length = 10
"#;

    std::fs::write(test_bed.path().join(".commitfmt.toml"), config).unwrap();
    write_hook(&test_bed);

    let err = repo
        .commit(
            "fix(application): test message about feature

body

BREAKING-CHANGE: Description about my feature and a lot of text",
        )
        .unwrap_err();

    match err {
        commitfmt_git::GitError::CommandFailed(_, message) => {
            assert_snapshot!("error_with_config", &message);
        }
        _ => panic!("Expected GitError::CommandFailed"),
    }
}

#[test]
fn test_hook_append_footers() {
    let test_bed = TestBed::with_default_history().unwrap();
    let repo = test_bed.repo.clone();
    test_bed.switch_to_new("feature/CFMT-123/description").unwrap();

    let config = r#"
[[additional-footers]]
key = "Authored-by"
value-template = "{{ echo 'John Doe' }}"

[[additional-footers]]
key = "Ticket-ID"
branch-value-pattern = "(?:.*)/([A-Z0-9-]+)/?(?:.*)"
"#;
    std::fs::write(test_bed.path().join(".commitfmt.toml"), config).unwrap();

    write_hook(&test_bed);

    repo.commit("feat(test): test\nbody").unwrap();

    let log = repo.get_log("HEAD~1", "HEAD").unwrap();
    assert_eq!(
        log[0].message,
        "feat(test): test\n\nbody\n\nAuthored-by: John Doe\nTicket-ID: CFMT-123\n"
    );
}
