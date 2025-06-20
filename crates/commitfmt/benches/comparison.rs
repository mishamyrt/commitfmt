use std::{path::Path, time::Duration};

use commitfmt_git::testing::TestBed;
use criterion::{criterion_group, criterion_main, Criterion};
use std::process::{Command, Stdio};

const COMMITFMT_CONFIG: &str = r#"
[lint.header]
type-enum = ["feat", "fix", "docs"]
type-required = true
scope-enum = ["core", "api"]

[lint.body]
max-length = 100
case = "upper-first"
full-stop = false

[lint.footer]
exists = ["Issue-ID"]
"#;

const COMMITLINT_CONFIG: &str = r#"
export default {
    rules: {
        "type-enum": [2, "always", ["feat", "fix", "docs"]],
        "type-empty": [2, "never"],
        "scope-enum": [2, "always", ["core", "api"]],
        "body-max-length": [2, "always", 100],
        "body-case": [2, "always", "sentence-case"],
        "trailer-exists": [2, "always", "Issue-ID:"],
    },
}"#;

/// Runs the `commitfmt` binary as a subprocess with the given commit message.
fn run_commitfmt(path: &Path) {
    // This environment variable is set by Cargo when compiling the benchmark.
    let bin_path = env!("CARGO_BIN_EXE_commitfmt");
    let mut child = Command::new(bin_path)
        .current_dir(path)
        .arg("--from")
        .arg("HEAD~10")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to spawn commitfmt process");

    let status = child.wait().expect("Failed to wait for commitfmt process");

    // Ensure the process exited successfully, indicating the lint passed.
    assert!(status.success());
}

/// Runs `commitlint` via `npx` as a subprocess with the given commit message.
fn run_commitlint(path: &Path) {
    let mut child = Command::new("commitlint")
        .current_dir(path)
        .arg("--from")
        .arg("HEAD~10")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("`commitlint` failed. Is Node.js and commitlint installed?");

    let status = child.wait().expect("Failed to wait for commitlint process");

    // Ensure the process exited successfully, indicating the lint passed.
    assert!(status.success());
}

fn comparison_benchmark(c: &mut Criterion) {
    let commits = vec![
        "feat(core): add support for parsing breakings\n\nBody\n\nIssue-ID: 123456",
        "fix(api): fix parsing of breakings\n\nBody\n\nIssue-ID: 123456",
        "docs: add documentation for parsing breakings\n\nBody\n\nIssue-ID: 123456",
        "feat(core): add support for parsing breakings\n\nBody\n\nIssue-ID: 123456",
        "fix(api): fix parsing of breakings\n\nBody\n\nIssue-ID: 123456",
        "docs: add documentation for parsing breakings\n\nBody\n\nIssue-ID: 123456",
        "feat(core): add support for parsing breakings\n\nBody\n\nIssue-ID: 123456",
        "fix(api): fix parsing of breakings\n\nBody\n\nIssue-ID: 123456",
        "docs: add documentation for parsing breakings\n\nBody\n\nIssue-ID: 123456",
        "feat(core): add support for parsing breakings\n\nBody\n\nIssue-ID: 123456",
        "fix(api): fix parsing of breakings\n\nBody\n\nIssue-ID: 123456",
    ];

    let mut group = c.benchmark_group("Linting");
    group.throughput(criterion::Throughput::Elements(commits.len() as u64));

    let commitfmt_bed = TestBed::with_history(&commits).expect("Failed to create test bed");
    let commitfmt_path = commitfmt_bed.path().join(".commitfmt.toml");
    std::fs::write(commitfmt_path, COMMITFMT_CONFIG).unwrap();

    let commitlint_bed = TestBed::with_history(&commits).expect("Failed to create test bed");
    let commitlint_path = commitlint_bed.path().join(".commitlintrc.js");
    std::fs::write(commitlint_path, COMMITLINT_CONFIG).unwrap();

    group.bench_function("commitfmt", |b| {
        b.iter(|| run_commitfmt(&commitfmt_bed.path()));
    });

    group.bench_function("commitlint", |b| {
        b.iter(|| run_commitlint(&commitlint_bed.path()));
    });

    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(10)).sample_size(20);
    targets = comparison_benchmark
}
criterion_main!(benches);
