use codspeed_criterion_compat::BenchmarkId;
use commitfmt_cc::Message;

use commitfmt_benchmark::criterion::{criterion_group, criterion_main, Criterion, Throughput};
use commitfmt_benchmark::{read_cc_files, TestCase};

fn create_cc_cases() -> Vec<TestCase> {
    read_cc_files().expect("Failed to read conventional commit files")
}

pub fn benchmark_parser(c: &mut Criterion) {
    let cases = create_cc_cases();
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
