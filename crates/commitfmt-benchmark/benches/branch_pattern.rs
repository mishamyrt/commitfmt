use codspeed_criterion_compat::BenchmarkId;
use commitfmt_benchmark::criterion::{criterion_group, criterion_main, Criterion, Throughput};
use commitfmt_workspace::CommitSettings;

const CONFIG: &str = r#"
[[additional-footers]]
key = "Ticket-ID"
branch-pattern = "(?:.*)/(?<TICKET_ID>[A-Z]+-[0-9]+)/?(?:.*)"
value = "${{ TICKET_ID }}"
"#;

const BRANCHES: &[(&str, &str)] = &[
    ("match_short", "feature/CFMT-123"),
    ("match_deep", "users/alice/feature/CFMT-123/description"),
    ("no_match", "main"),
];

fn benchmark_branch_pattern(c: &mut Criterion) {
    let settings = CommitSettings::from_toml(CONFIG).unwrap();
    let footers = settings.footers.borrow();
    let pattern = footers[0].branch_pattern.as_ref().unwrap();
    let mut group = c.benchmark_group("branch_pattern_captures");

    assert!(pattern.captures(BRANCHES[0].1).is_some());
    assert!(pattern.captures(BRANCHES[1].1).is_some());
    assert!(pattern.captures(BRANCHES[2].1).is_none());

    for (name, branch) in BRANCHES {
        group.throughput(Throughput::Bytes(branch.len() as u64));
        group.bench_with_input(BenchmarkId::from_parameter(name), branch, |b, branch| {
            b.iter(|| {
                let captures = pattern.captures(std::hint::black_box(branch));
                std::hint::black_box(captures)
            });
        });
    }

    group.finish();
}

criterion_group!(benches, benchmark_branch_pattern);
criterion_main!(benches);
