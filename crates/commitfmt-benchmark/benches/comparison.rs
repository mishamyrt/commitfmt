use std::{
    path::{Path, PathBuf},
    time::Duration,
};

use commitfmt_benchmark::criterion::{criterion_group, criterion_main, Criterion, Throughput};
use commitfmt_git::testing::TestBed;
use std::process::{Command, Stdio};

const COMMITFMT_CONFIG: &str = r#"
[lint.header]
type-enum = ["feat", "fix", "docs"]
type-required = true
scope-enum = ["core", "api"]

[lint.body]
max-length = 100

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
        "trailer-exists": [2, "always", "Issue-ID:"],
    },
}"#;

fn run_linter(name: &str, dir: &Path, bin_path: &Path, args: &[&str]) {
    let status = Command::new(bin_path)
        .current_dir(dir)
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .unwrap_or_else(|error| panic!("Failed to run {name}: {error}"));

    assert!(status.success(), "{name} exited with {status}");
}

/// Runs the benchmarks for the comparison of `commitfmt` and `commitlint`.
///
/// This benchmark requires the local Node.js dependencies to be installed and
/// `commitfmt` compiled with the `dist` profile.
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

    let test_bed = TestBed::with_history(&commits).expect("Failed to create test bed");
    let test_bed_path = test_bed.path();
    let commitfmt_path = test_bed_path.join(".commitfmt.toml");
    std::fs::write(commitfmt_path, COMMITFMT_CONFIG).unwrap();

    let commitlint_path = test_bed_path.join("commitlint.config.mjs");
    std::fs::write(commitlint_path, COMMITLINT_CONFIG).unwrap();
    test_bed.repo.write_commit_message(commits[0]).expect("Failed to prepare COMMIT_EDITMSG");

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let workspace_root = std::path::Path::new(manifest_dir)
        .parent()
        .and_then(|p| p.parent())
        .expect("Failed to find workspace root from manifest dir");
    let target_dir = std::env::var_os("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| workspace_root.join("target"));
    let bin_path =
        target_dir.join("dist").join(format!("commitfmt{}", std::env::consts::EXE_SUFFIX));
    assert!(
        bin_path.is_file(),
        "commitfmt dist binary not found; run `make benchmark-comparison`"
    );
    let commitlint_executable = if cfg!(windows) { "commitlint.cmd" } else { "commitlint" };
    let commitlint_bin_path = workspace_root
        .join("crates")
        .join("commitfmt-benchmark")
        .join("comparison")
        .join("node_modules")
        .join(".bin")
        .join(commitlint_executable);
    assert!(
        commitlint_bin_path.is_file(),
        "local commitlint binary not found; run `make benchmark-comparison`"
    );

    let commitfmt_hook_args = ["--lint"];
    let commitlint_hook_args = ["--edit", ".git/COMMIT_EDITMSG"];
    run_linter("commitfmt", &test_bed_path, &bin_path, &commitfmt_hook_args);
    run_linter("commitlint", &test_bed_path, &commitlint_bin_path, &commitlint_hook_args);

    let mut hook_group = c.benchmark_group("Linting/1_commit_hook");
    hook_group.throughput(Throughput::Elements(1));
    hook_group.bench_function("commitfmt", |b| {
        b.iter(|| {
            run_linter("commitfmt", &test_bed_path, &bin_path, &commitfmt_hook_args);
        });
    });
    hook_group.bench_function("commitlint", |b| {
        b.iter(|| {
            run_linter(
                "commitlint",
                &test_bed_path,
                &commitlint_bin_path,
                &commitlint_hook_args,
            );
        });
    });
    hook_group.finish();

    let range_args = ["--from", "HEAD~10"];
    run_linter("commitfmt", &test_bed_path, &bin_path, &range_args);
    run_linter("commitlint", &test_bed_path, &commitlint_bin_path, &range_args);

    let mut range_group = c.benchmark_group("Linting/10_commits");
    range_group.throughput(Throughput::Elements(10));
    range_group.bench_function("commitfmt", |b| {
        b.iter(|| run_linter("commitfmt", &test_bed_path, &bin_path, &range_args));
    });
    range_group.bench_function("commitlint", |b| {
        b.iter(|| {
            run_linter("commitlint", &test_bed_path, &commitlint_bin_path, &range_args);
        });
    });
    range_group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(10)).sample_size(20);
    targets = comparison_benchmark
}
criterion_main!(benches);
