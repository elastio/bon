#![allow(missing_docs)]

use e2e_tests::benchmark::*;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn criterion_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Main");
    group.bench_function(BenchmarkId::new("Regular", ""), |b| {
        b.iter(|| {
            regular_bench();
        })
    });
    group.bench_function(BenchmarkId::new("Builder", ""), |b| {
        b.iter(|| {
            builder_bench();
        })
    });
    group.finish();
}

criterion_group!(benches, criterion_bench);
criterion_main!(benches);
