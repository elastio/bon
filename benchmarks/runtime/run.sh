#!/usr/bin/env bash

set -euxo pipefail

# List src/ dir, remove `.rs` from file extensions and remove lib.rs
benches=${1:-$(find src -name '*.rs' | sed 's/\.rs$//' | sed 's/src\///' | grep -v lib)}

export CARGO_INCREMENTAL=0

for bench in $benches; do
    cargo build --features "$bench" --release -p runtime-benchmarks

    # If vscode is present, show diff:
    if command -v code; then
        cargo asm --features "$bench" --no-color "runtime_benchmarks::$bench::builder_bench" > builder.dbg.s || true
        cargo asm --features "$bench" --no-color "runtime_benchmarks::$bench::regular_bench" > regular.dbg.s || true

        code --diff regular.dbg.s builder.dbg.s
    fi

    cargo bench --features "$bench" -p runtime-benchmarks --profile release --bench iai
    cargo bench --features "$bench" -p runtime-benchmarks --profile release --bench criterion
done
