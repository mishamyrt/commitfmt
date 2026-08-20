use codspeed_criterion_compat::{
    criterion_group, criterion_main, BenchmarkId, Criterion, Throughput,
};
use commitfmt_cc::Message;
use commitfmt_linter::Check;
use commitfmt_workspace::CommitSettings;

const CONFIG_DATA: &str = r#"
[lint.header]
type-enum = ["feat", "fix", "docs"]
type-required = true
scope-required = true
description-max-length = 15
"#;

const COMPLEX_CONFIG_DATA: &str = r#"
[lint.header]
type-enum = ["feat", "fix", "docs"]
type-required = true
scope-required = true
description-max-length = 40

[lint.body]
case = "upper-first"
full-stop = true
max-length = 2000
max-line-length = 80
min-length = 10

[lint.footer]
exists = ["issue-id"]
key-case = "kebab"
max-length = 80
max-line-length = 80
min-length = 3
"#;

pub fn lint_message_benchmark(c: &mut Criterion) {
    let settings = CommitSettings::from_toml(CONFIG_DATA).unwrap();
    let message = Message::parse("feat(scope): description", Some(":"), Some("#"));

    c.bench_function("lint message", |b| {
        b.iter(|| {
            let mut check = Check::new(&settings.rules.settings, settings.rules.set);
            check.lint(std::hint::black_box(&message));
            std::hint::black_box(check.report.violations.is_empty())
        });
    });
}

pub fn lint_complexity_benchmark(c: &mut Criterion) {
    let settings = CommitSettings::from_toml(COMPLEX_CONFIG_DATA).unwrap();
    let large_message = format!(
        "feat(core): add benchmark\n\n{}\nissue-id: CFMT-123",
        "Body benchmark line.\n".repeat(64)
    );
    let inputs = [
        (
            "full_valid",
            "feat(core): add benchmark\n\nBody description.\n\nissue-id: CFMT-123".to_string(),
            true,
        ),
        (
            "full_invalid",
            "unknown: this description is deliberately longer than forty characters.\n\nshort\n\nWrong-Key: x"
                .to_string(),
            false,
        ),
        ("large_body", large_message, true),
    ];
    let messages: Vec<_> = inputs
        .iter()
        .map(|(name, input, valid)| {
            (*name, input.len(), Message::parse(input, Some(":"), Some("#")), *valid)
        })
        .collect();
    let mut group = c.benchmark_group("lint_complexity");

    for (_, _, message, valid) in &messages {
        let mut check = Check::new(&settings.rules.settings, settings.rules.set);
        check.lint(message);
        assert_eq!(check.report.violations.is_empty(), *valid);
    }

    for (name, input_len, message, _) in &messages {
        group.throughput(Throughput::Bytes(*input_len as u64));
        group.bench_with_input(BenchmarkId::from_parameter(name), message, |b, message| {
            b.iter(|| {
                let mut check = Check::new(&settings.rules.settings, settings.rules.set);
                check.lint(std::hint::black_box(message));
                std::hint::black_box(check.report)
            });
        });
    }

    group.finish();
}

criterion_group!(benches, lint_message_benchmark, lint_complexity_benchmark);
criterion_main!(benches);
