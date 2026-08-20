use codspeed_criterion_compat::BenchmarkId;
use commitfmt_benchmark::criterion::{criterion_group, criterion_main, Criterion, Throughput};
use commitfmt_workspace::CommitSettings;

struct SettingsCase {
    name: &'static str,
    config: &'static str,
}

const CASES: &[SettingsCase] = &[
    SettingsCase { name: "empty", config: "" },
    SettingsCase {
        name: "all_rule_types",
        config: r#"
[lint]
unsafe-fixes = true

[lint.header]
description-case = "upper-first"
description-max-length = 72
scope-case = "kebab"
scope-enum = ["api", "core"]
type-case = "lower"
type-enum = ["feat", "fix", "docs"]
type-required = true

[lint.body]
case = "upper-first"
full-stop = true
max-length = 500
max-line-length = 100

[lint.footer]
exists = ["issue-id"]
key-case = "kebab"
max-length = 200
"#,
    },
    SettingsCase {
        name: "branch_patterns",
        config: r#"
[[additional-footers]]
key = "Ticket-ID"
branch-pattern = "(?:.*)/(?<TICKET_ID>[A-Z]+-[0-9]+)/?(?:.*)"
value = "${{ TICKET_ID }}"

[[additional-footers]]
key = "Issue-ID"
branch-pattern = "(?:.*)/(?<ISSUE_ID>[0-9]+)/?(?:.*)"
value = "${{ ISSUE_ID }}"
"#,
    },
];

fn benchmark_settings(c: &mut Criterion) {
    let mut group = c.benchmark_group("settings");

    for case in CASES {
        group.throughput(Throughput::Elements(1));
        group.bench_with_input(BenchmarkId::from_parameter(case.name), case, |b, case| {
            b.iter(|| {
                let settings = CommitSettings::from_toml(std::hint::black_box(case.config));
                std::hint::black_box(settings.unwrap())
            });
        });
    }

    group.finish();
}

criterion_group!(benches, benchmark_settings);
criterion_main!(benches);
