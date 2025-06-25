use codspeed_criterion_compat::BenchmarkId;
use commitfmt::Commitfmt;
use commitfmt_cc::SeparatorAlignment;
use commitfmt_git::testing::TestBed;
use std::collections::HashMap;

use commitfmt_benchmark::criterion::{criterion_group, criterion_main, Criterion, Throughput};
use commitfmt_benchmark::read_cc_files;

/// Test case for formatter benchmarks
struct FormatCase {
    name: &'static str,
    input: &'static str,
    lint_only: bool,
}

/// Benchmark configuration for footers
struct FooterConfig {
    name: &'static str,
    content: &'static str,
}

/// Setup data for benchmarks
struct BenchmarkSetup {
    test_bed: TestBed,
    app: Commitfmt,
    variables: HashMap<String, String>,
}

impl BenchmarkSetup {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let test_bed = TestBed::empty()?;
        let app = Commitfmt::from_path(&test_bed.path())?;
        let variables = HashMap::from([("BUILD_ID".to_string(), "123456".to_string())]);

        Ok(Self { test_bed, app, variables })
    }

    fn with_config(
        &self,
        config_content: &str,
    ) -> Result<Commitfmt, Box<dyn std::error::Error>> {
        use std::fs;
        let config_path = self.test_bed.path().join("commitfmt.toml");
        fs::write(&config_path, config_content)?;
        Ok(Commitfmt::from_path(&self.test_bed.path())?)
    }
}

/// Helper function to run benchmark group with common logic
fn run_benchmark_group<F>(
    c: &mut Criterion,
    group_name: &str,
    cases: &[FormatCase],
    app: &Commitfmt,
    throughput_fn: F,
) where
    F: Fn(&FormatCase) -> Throughput,
{
    let mut group = c.benchmark_group(group_name);
    group.sample_size(50);

    for case in cases {
        group.throughput(throughput_fn(case));
        group.bench_with_input(BenchmarkId::from_parameter(case.name), case, |b, case| {
            b.iter(|| {
                let result = app.format_commit_message(
                    std::hint::black_box(case.input),
                    std::hint::black_box(case.lint_only),
                );
                std::hint::black_box(result)
            });
        });
    }

    group.finish();
}

/// Calculate throughput based on message complexity
fn message_complexity_throughput(case: &FormatCase) -> Throughput {
    let lines = case.input.lines().count() as u64;
    let footers = case.input.matches('\n').filter(|_| case.input.contains(':')).count() as u64;
    let complexity = case.input.len() as u64 + lines * 10 + footers * 5;
    Throughput::Elements(complexity)
}

/// Simple byte-based throughput
fn byte_throughput(case: &FormatCase) -> Throughput {
    Throughput::Bytes(case.input.len() as u64)
}

/// Creates synthetic test cases for formatter benchmarks.
static FORMAT_CASES: &[FormatCase] = &[
    FormatCase {
        name: "simple_format",
        input: "feat  (  test   ): test\nbody",
        lint_only: false,
    },
    FormatCase {
        name: "with_footers",
        input: "feat  (  test   ): test\nbody\n\nfooter-key: value",
        lint_only: false,
    },
    FormatCase {
        name: "lint_only_valid",
        input: "feat(test): test\nbody",
        lint_only: true,
    },
    FormatCase {
        name: "lint_only_invalid",
        input: "feat  (  test   ): test",
        lint_only: true,
    },
    FormatCase {
        name: "complex_message",
        input: "feat  (  scope/subsystem   ): add new authentication feature\n\nImplement OAuth2 flow with refresh tokens.\nAdd user management endpoints.\n\nBreaking-Change: API endpoints changed\nCloses: #123\nRefs: #456",
        lint_only: false,
    },
];

/// Creates test cases for additional footers scenarios.
static FOOTER_CASES: &[FormatCase] = &[
    FormatCase {
        name: "static_footer",
        input: "feat(auth): add OAuth2 support\n\nImplement authentication flow",
        lint_only: false,
    },
    FormatCase {
        name: "multiple_static_footers",
        input: "fix(api): resolve timeout issue\n\nFix connection timeout in production",
        lint_only: false,
    },
    FormatCase {
        name: "branch_pattern_footer",
        input: "feat(ui): improve button design\n\nMake buttons more accessible",
        lint_only: false,
    },
    FormatCase {
        name: "with_existing_footer",
        input: "chore(deps): update dependencies\n\nUpdate to latest versions\n\nTicket-ID: EXISTING-123",
        lint_only: false,
    },
    FormatCase {
        name: "template_command_footer",
        input: "docs(readme): update installation guide\n\nAdd new package manager instructions",
        lint_only: false,
    },
    FormatCase {
        name: "custom_separator_footer",
        input: "style(css): improve responsive design\n\nOptimize mobile layout",
        lint_only: false,
    },
];

/// Footer configurations for benchmarks
static FOOTER_CONFIGS: &[FooterConfig] = &[
    FooterConfig {
        name: "static_footers",
        content: r#"
[[additional-footers]]
key = "Authored-By"
value = "John Doe"

[[additional-footers]]
key = "Team"
value = "Development"
"#,
    },
    FooterConfig {
        name: "branch_pattern_footers",
        content: r#"
[[additional-footers]]
key = "Ticket-ID"
branch-pattern = "(?:.*)/(?<TICKET_ID>[A-Z0-9-]+)/?(?:.*)"
value = "${{ TICKET_ID }}"

[[additional-footers]]
key = "Issue-Ref"
branch-pattern = "(?:.*)/(?<ISSUE_ID>[0-9]+)/?(?:.*)"
value = "${{ ISSUE_ID }}"
"#,
    },
    FooterConfig {
        name: "template_footers",
        content: r#"
[[additional-footers]]
key = "Authored-By"
value = "{{ echo $USER }}"

[[additional-footers]]
key = "Build-ID"
value = "{{ date +%s }}"
"#,
    },
    FooterConfig {
        name: "mixed_footers",
        content: r#"
[[additional-footers]]
key = "Authored-By"
value = "Static Author"

[[additional-footers]]
key = "Ticket-ID"
branch-pattern = "(?:.*)/(?<TICKET_ID>[A-Z0-9-]+)/?(?:.*)"
value = "${{ TICKET_ID }}"
on-conflict = "append"

[[additional-footers]]
key = "Build-ID"
value = "{{ echo $BUILD_ID }}"
separator = ":"
alignment = "right"
"#,
    },
    FooterConfig {
        name: "conflict_handling",
        content: r#"
[[additional-footers]]
key = "Ticket-ID"
value = "AUTO-123"
on-conflict = "skip"

[[additional-footers]]
key = "Priority"
value = "High"
on-conflict = "append"
"#,
    },
];

/// Creates test cases from real conventional commit examples.
fn create_cc_format_cases() -> Vec<FormatCase> {
    read_cc_files()
        .expect("Failed to read conventional commit files")
        .into_iter()
        .map(|test_case| FormatCase {
            name: Box::leak(format!("cc_{}", test_case.name).into_boxed_str()),
            input: Box::leak(test_case.message.into_boxed_str()),
            lint_only: false,
        })
        .collect()
}

/// Benchmarks formatter performance on synthetic test cases.
pub fn benchmark_formatter_synthetic(c: &mut Criterion) {
    let setup = BenchmarkSetup::new().unwrap();
    run_benchmark_group(
        c,
        "formatter_synthetic",
        FORMAT_CASES,
        &setup.app,
        message_complexity_throughput,
    );
}

/// Benchmarks formatter performance on real conventional commit messages.
pub fn benchmark_formatter_real_messages(c: &mut Criterion) {
    let setup = BenchmarkSetup::new().unwrap();
    let cases = create_cc_format_cases();
    run_benchmark_group(c, "formatter_real_messages", &cases, &setup.app, byte_throughput);
}

/// Benchmarks formatter performance with additional footers configurations.
pub fn benchmark_formatter_with_footers(c: &mut Criterion) {
    let setup = BenchmarkSetup::new().unwrap();
    let mut group = c.benchmark_group("formatter_additional_footers");
    group.sample_size(50);

    for config in FOOTER_CONFIGS {
        let app = setup.with_config(config.content).unwrap();

        for case in FOOTER_CASES {
            let bench_name = format!("{}_{}", config.name, case.name);
            group.throughput(message_complexity_throughput(case));
            group.bench_with_input(
                BenchmarkId::from_parameter(&bench_name),
                case,
                |b, case| {
                    b.iter(|| {
                        let result = app.format_commit_message(
                            std::hint::black_box(case.input),
                            std::hint::black_box(case.lint_only),
                        );
                        std::hint::black_box(result)
                    });
                },
            );
        }
    }

    group.finish();
}

/// Benchmarks the isolated additional footers processing.
pub fn benchmark_footer_processing_only(c: &mut Criterion) {
    use commitfmt_cc::Message;
    use commitfmt_workspace::CommitSettings;

    static CONFIG_CONTENT: &str = r#"
[[additional-footers]]
key = "Static-Footer"
value = "Static Value"

[[additional-footers]]
key = "Template-Footer"
value = "{{ echo test-value }}"

[[additional-footers]]
key = "Build-ID"
value = "${{ BUILD_ID }}"
"#;

    static TEST_INPUTS: &[(&str, &str)] = &[
        ("simple", "feat(test): add feature\n\nbody text"),
        ("with_existing_footers", "feat(test): add feature\n\nbody text\n\nExisting: value"),
        ("complex", "feat(scope): add complex feature\n\nDetailed description\nwith multiple lines\n\nBreaking-Change: something"),
    ];

    let setup = BenchmarkSetup::new().unwrap();

    let settings = CommitSettings::from_toml(CONFIG_CONTENT).unwrap();
    let footers = settings.footers.borrow();

    let mut group = c.benchmark_group("footer_processing_only");
    group.sample_size(50);

    for (name, input) in TEST_INPUTS {
        group.throughput(Throughput::Bytes(input.len() as u64));
        group.bench_with_input(BenchmarkId::from_parameter(name), input, |b, input| {
            b.iter(|| {
                let mut message = Message::parse(
                    std::hint::black_box(input),
                    std::hint::black_box(Some(":")),
                    std::hint::black_box(Some("#")),
                )
                .unwrap();

                for footer in footers.iter() {
                    let value = match footer.value.render(&setup.variables) {
                        Ok(v) => v,
                        Err(_) => "fallback-value".to_string(),
                    };
                    message.footers.push(commitfmt_cc::Footer {
                        key: footer.key.clone(),
                        value,
                        separator: ':',
                        alignment: SeparatorAlignment::default(),
                    });
                }

                std::hint::black_box(message)
            });
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    benchmark_formatter_synthetic,
    benchmark_formatter_real_messages,
    benchmark_formatter_with_footers,
    benchmark_footer_processing_only
);
criterion_main!(benches);
