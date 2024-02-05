use criterion::{criterion_group, criterion_main, Criterion, black_box};

pub fn native_sqrt(c: &mut Criterion) {
    println!("{}", f64::sqrt(45624569.0));
    c.bench_function("native sqrt 45624569", |b| b.iter(|| f64::sqrt(black_box(45624569.0)) ));
}

fn my_sqrt(n: f64) -> f64 {
    let mut i: i64 = n.to_bits() as i64;
    i = 0x5fe6eb50c7b537a9_i64.wrapping_sub(i >> 1);
    let y = f64::from_bits(i as u64);

    let mut x = n * y;
    let mut h = y * 0.5;

    let r = 1.5 - x * h; x *= r; h *= r;
    let r = 1.5 - x * h; x *= r; h *= r;
    let r = 1.5 - x * h; x *= r; h *= r;
    let r = 1.5 - x * h; x *= r;

    x
}

pub fn custom_sqrt(c: &mut Criterion) {
    println!("{}", my_sqrt(45624569.0));
    c.bench_function("custom sqrt 45624569", |b| b.iter(|| my_sqrt(black_box(45624569.0)) ));
}

criterion_group!(benches, native_sqrt, custom_sqrt);
criterion_main!(benches);

