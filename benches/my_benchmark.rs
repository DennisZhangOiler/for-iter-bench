use criterion::{black_box, criterion_group, criterion_main, Criterion};
use foo::{for_loop, iter};

pub fn for_benchmark(c: &mut Criterion) {
    c.bench_function("for", |b| {
        b.iter(|| for_loop(black_box(&mut vec![0; 100000])))
    });
}

pub fn iter_benchmark(c: &mut Criterion) {
    c.bench_function("iter", |b| b.iter(|| iter(black_box(&mut vec![0; 100000]))));
}

criterion_group!(benches, for_benchmark, iter_benchmark);
criterion_main!(benches);
