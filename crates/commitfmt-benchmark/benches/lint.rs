use codspeed_criterion_compat::{criterion_group, criterion_main, Criterion};
use commitfmt_cc::Message;
use commitfmt_linter::Check;
use commitfmt_workspace::CommitSettings;

fn lint_message() -> bool {
    let config_data = r#"
[lint.header]
type-enum = ["feat", "fix", "docs"]
type-required = true
scope-required = true
description-max-length = 15
"#;
    let settings = CommitSettings::from_toml(config_data).unwrap();

    let input = "feat(scope): description";
    let message = Message::parse(input, Some(":"), Some("#")).unwrap();

    let mut check = Check::new(&settings.rules.settings, settings.rules.set);
    check.lint(&message);

    check.report.violations.is_empty()
}

pub fn lint_message_benchmark(c: &mut Criterion) {
    c.bench_function("lint message", |b| {
        b.iter(|| {
            let result = lint_message();
            std::hint::black_box(result)
        });
    });
}

criterion_group!(benches, lint_message_benchmark);
criterion_main!(benches);
