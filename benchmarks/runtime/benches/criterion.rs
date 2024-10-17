#![allow(missing_docs)]

use runtime_benchmarks::{builder_bench, regular_bench};

fn criterion_bench(c: &mut criterion::Criterion) {
    let builder_bench_addr: fn() -> u32 = runtime_benchmarks::bench::builder_bench;
    let regular_bebch_addr: fn() -> u32 = runtime_benchmarks::bench::regular_bench;

    let equal = if builder_bench_addr == regular_bebch_addr {
        "equal"
    } else {
        "not equal"
    };

    println!(
        "Benchmarks addresses ({equal}):\n\
        builder_bench: {builder_bench_addr:p}\n\
        regular_bench: {regular_bebch_addr:p}",
    );

    let mut group = c.benchmark_group("main");
    group.bench_function(criterion::BenchmarkId::new("regular_bench", ""), |b| {
        b.iter(regular_bench);
    });
    group.bench_function(criterion::BenchmarkId::new("builder_bench", ""), |b| {
        b.iter(builder_bench);
    });
    group.finish();
}

criterion::criterion_group!(benches, criterion_bench);
criterion::criterion_main!(benches);
