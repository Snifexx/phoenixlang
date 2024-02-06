use criterion::{criterion_group, criterion_main, Criterion, black_box};

pub fn negate_bit(c: &mut Criterion) {
    c.bench_function("negating bit", |b| b.iter(|| f64::from_bits(0x8000000000000000 ^ black_box(0.4_f64).to_bits())));
}

pub fn negate(c: &mut Criterion) {
    c.bench_function("negate normal", |b| b.iter(|| -0.4_f64 ));
}

criterion_group!(benches, negate, negate_bit);
criterion_main!(benches);

