use codspeed_criterion_compat::BenchmarkId;
use commitfmt_cc::Message;

use commitfmt_benchmark::criterion::{criterion_group, criterion_main, Criterion, Throughput};

struct TestCase {
    name: String,
    message: String,
}

fn create_test_cases() -> Vec<TestCase> {
    vec![
        TestCase {
            name: "header".to_string(),
            message: "feat: description".to_string(),
        },
        TestCase {
            name: "header+scope".to_string(),
            message: "feat(scope): description".to_string(),
        },
        TestCase {
            name: "header+scope+body".to_string(),
            message: "feat(scope): description\n\nBody".to_string(),
        },
        TestCase {
            name: "header+scope+body+footer".to_string(),
            message: "feat(scope): description\n\nBody\n\nIssue-ID: 123".to_string(),
        },
        TestCase {
            name: "header+scope+body+footer+comment".to_string(),
            message: "feat(scope): description\n\nBody\n\nIssue-ID: 123\n\nSigned-off-by: John Doe <john.doe@example.com>".to_string(),
        },
        TestCase {
            name: "header+scope+body+footers+comment".to_string(),
            message: "feat(scope): description\n\nBody\n\nIssue-ID: 123\n\nSigned-off-by: John Doe <john.doe@example.com>\n\n# comment\n# comment2\n# comment3".to_string(),
        },
    ]
}

pub fn benchmark_parser(c: &mut Criterion) {
    let cases = create_test_cases();
    let mut group = c.benchmark_group("parser");

    for case in cases {
        group.throughput(Throughput::Bytes(case.message.len() as u64));
        group.bench_with_input(BenchmarkId::from_parameter(&case.name), &case, |b, case| {
            b.iter(|| {
                let parsed = Message::parse(
                    &case.message,
                    std::hint::black_box(Some(":")),
                    std::hint::black_box(Some("#")),
                )
                .expect("Failed to parse message");

                std::hint::black_box(parsed)
            });
        });
    }

    group.finish();
}

criterion_group!(benches, benchmark_parser);
criterion_main!(benches);
