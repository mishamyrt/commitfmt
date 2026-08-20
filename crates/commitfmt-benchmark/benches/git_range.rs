use codspeed_criterion_compat::BenchmarkId;
use commitfmt_benchmark::criterion::{criterion_group, criterion_main, Criterion, Throughput};
use commitfmt_git::testing::TestBed;

const MAX_RANGE_SIZE: usize = 32;
const RANGE_SIZES: &[usize] = &[1, 8, MAX_RANGE_SIZE];

fn benchmark_git_range(c: &mut Criterion) {
    let messages: Vec<_> = (0..=MAX_RANGE_SIZE)
        .map(|index| format!("feat(bench): commit {index}\n\n{}", "Body payload. ".repeat(64)))
        .collect();
    let messages: Vec<_> = messages.iter().map(String::as_str).collect();
    let test_bed = TestBed::with_history(&messages).unwrap();
    let mut group = c.benchmark_group("git_range_stream");
    group.sample_size(20);

    for &commits_count in RANGE_SIZES {
        let from = format!("HEAD~{commits_count}");
        group.throughput(Throughput::Elements(commits_count as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(commits_count),
            &commits_count,
            |b, &commits_count| {
                b.iter(|| {
                    let commits = test_bed
                        .repo
                        .stream_log(std::hint::black_box(&from), "HEAD")
                        .unwrap()
                        .fold(0, |count, commit| {
                            std::hint::black_box(commit.unwrap());
                            count + 1
                        });
                    assert_eq!(commits, commits_count);
                    std::hint::black_box(commits)
                });
            },
        );
    }

    group.finish();
}

criterion_group!(benches, benchmark_git_range);
criterion_main!(benches);
