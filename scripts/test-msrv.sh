#!/usr/bin/env bash

set -euo pipefail

. "$(dirname "${BASH_SOURCE[0]}")/util/lib.sh"

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

    info "Running in a temp dir $(pwd)"
fi

step echo '1.59.0' > rust-toolchain

step cargo --version --verbose

with_log cd bon

step echo '[workspace]' >> Cargo.toml

step cargo update -p syn --precise 2.0.56
step cargo update -p tokio --precise 1.29.1
step cargo update -p expect-test --precise 1.4.1
step cargo update -p windows-sys --precise 0.52.0
step cargo update -p libc --precise 0.2.163

export RUSTFLAGS="${RUSTFLAGS:-} --allow unknown-lints"

features=experimental-overwritable,experimental-getter

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
