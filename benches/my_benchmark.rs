use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use phoenixlang::test_hash;

pub fn criterion_benchmark(c: &mut Criterion) {
    let keywords = vec![
        "and", "trait", "pub", "self", "infix", "suffix"
    ];
    let mut group = c.benchmark_group("from_elem");

    for identifier in keywords {
        group.bench_with_input(BenchmarkId::from_parameter(identifier), identifier, |b, identifier| {
            b.iter(|| test_hash(identifier, 0));
        });
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

