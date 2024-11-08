#!/usr/bin/env bash

set -euxo pipefail

macros=(
    bon
    typed-builder
    derive_builder
)

suites=(
    structs_100_fields_10
    structs_10_fields_50
)

hyperfine \
    --setup 'cargo build -p compilation-benchmarks --features={suite},{macro}' \
    --prepare 'cargo clean -p compilation-benchmarks' \
    --shell=none \
    --export-markdown results.md \
    --parameter-list macro "$(IFS=, ; echo "${macros[*]}")," \
    --parameter-list suite "$(IFS=, ; echo "${suites[*]}")" \
    --command-name '{suite} {macro}' \
    'cargo build -p compilation-benchmarks --features={suite},{macro}'
