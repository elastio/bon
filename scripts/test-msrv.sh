#!/usr/bin/env bash

set -euo pipefail

. "$(dirname "${BASH_SOURCE[0]}")/util/lib.sh"

msrv="${1:-1.59.0}"

# If not on CI - create temp dir
if [[ ! -v CI ]]; then
    trap cleanup SIGINT SIGTERM ERR EXIT

    temp_dir=$(mktemp -d)

    function cleanup {
        # Unset the trap to prevent an infinite loop
        trap - SIGINT SIGTERM ERR EXIT

        step rm -rf "$temp_dir"
    }

    step cp -r README.md bon bon-macros "$temp_dir"

    with_log pushd "$temp_dir"

    step echo "$msrv" > rust-toolchain

    info "Running in a temp dir $(pwd)"
fi

step cargo --version --verbose

with_log cd bon

step echo '[workspace]' >> Cargo.toml

step cargo update --precise 1.0.15  -p itoa
step cargo update --precise 1.0.20  -p ryu
step cargo update --precise 1.0.101 -p proc-macro2
step cargo update --precise 1.0.40  -p quote
step cargo update --precise 1.17.2  -p once_cell
step cargo update --precise 1.0.89  -p trybuild
step cargo update --precise 1.0.143 -p serde_json
step cargo update --precise 1.0.194 -p serde
step cargo update --precise 0.2.17  -p prettyplease
step cargo update --precise 2.0.56  -p syn
step cargo update --precise 1.29.1  -p tokio
step cargo update --precise 1.4.1   -p expect-test
step cargo update --precise 0.52.0  -p windows-sys
step cargo update --precise 0.2.163 -p libc
step cargo update --precise 0.3.2   -p glob

export RUSTFLAGS="${RUSTFLAGS:-} --allow unknown-lints"

features=experimental-overwritable

step cargo clippy --all-targets --locked --features "$features"

test_args=(
    --locked
    --lib
    --tests
    --examples
    --bins
    --benches
    # We intentionally don't include doc tests, because they use
    # the syntax from the newest versions and that's fine because
    # doc tests are the face of this library.
    --
    # Don't run trybuild tests. We know they will fail because
    # Rust compiler error messages change from version to version.
    --skip ui::ui
)

step cargo test --features "$features" "${test_args[@]}"
