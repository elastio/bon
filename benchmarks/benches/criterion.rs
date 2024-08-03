#![allow(missing_docs)]

use benchmarks::{builder_bench, regular_bench};

fn criterion_bench(c: &mut criterion::Criterion) {
    let mut group = c.benchmark_group("main");
    group.bench_function(criterion::BenchmarkId::new("builder_bench", ""), |b| {
        b.iter(builder_bench)
    });
    group.bench_function(criterion::BenchmarkId::new("regular_bench", ""), |b| {
        b.iter(regular_bench)
    });
    group.finish();
}

criterion::criterion_group!(benches, criterion_bench);
criterion::criterion_main!(benches);
