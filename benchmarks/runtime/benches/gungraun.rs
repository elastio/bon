#![allow(
    missing_docs,

    // Allowing unsafe code due to this issue in gungraun:
    // https://github.com/gungraun/gungraun/issues/490
    unsafe_code,
)]
use gungraun::{library_benchmark, library_benchmark_group, main};

#[library_benchmark]
fn regular_bench() {
    runtime_benchmarks::regular_bench();
}

#[library_benchmark]
fn builder_bench() {
    runtime_benchmarks::builder_bench();
}

library_benchmark_group!(
    name = bench_builder_group;
    benchmarks = regular_bench, builder_bench
);

main!(library_benchmark_groups = bench_builder_group);
