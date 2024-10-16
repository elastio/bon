#!/usr/bin/env bash

set -euxo pipefail

macros=(
    bon
    # bon-overwritable
    typed-builder
    derive_builder
)

suites=(
    structs_100_fields_10
    # structs_10_fields_50
    # structs_200_fields_20
)

hyperfine \
    --setup 'cargo build -p compilation-benchmarks --features={suite},{macro}' \
    --prepare 'cargo clean -p compilation-benchmarks' \
    --shell=none \
    --export-markdown results.md \
    --parameter-list macro "$(IFS=, ; echo "${macros[*]}")," \
    --parameter-list suite "$(IFS=, ; echo "${suites[*]}")" \
    'cargo build -p compilation-benchmarks --features={suite},{macro}'
