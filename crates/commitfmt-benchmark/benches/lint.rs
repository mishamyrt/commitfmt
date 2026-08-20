use codspeed_criterion_compat::{criterion_group, criterion_main, Criterion};
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

criterion_group!(benches, lint_message_benchmark);
criterion_main!(benches);
