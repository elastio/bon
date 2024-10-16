#!/usr/bin/env bash

set -euxo pipefail

bench=${1:-structs_10_args_10}

hyperfine \
    --setup 'cargo build -p compilation-benchmarks --features={features}' \
    --prepare 'cargo clean -p compilation-benchmarks' \
    --shell=none \
    --export-markdown results.md \
    --parameter-list features bon,bon-overwritable,typed-builder,derive_builder,\
    'cargo build -p compilation-benchmarks --features={features}'
