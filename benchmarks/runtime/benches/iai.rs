#![allow(missing_docs)]
use iai_callgrind::{library_benchmark, library_benchmark_group, main};

#[library_benchmark]
fn builder_bench() {
    runtime_benchmarks::builder_bench();
}

#[library_benchmark]
fn regular_bench() {
    runtime_benchmarks::regular_bench();
}

library_benchmark_group!(
    name = bench_builder_group;
    benchmarks = builder_bench, regular_bench
);

main!(library_benchmark_groups = bench_builder_group);
