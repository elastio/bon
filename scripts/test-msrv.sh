#!/usr/bin/env bash

set -euo pipefail

. "$(dirname "${BASH_SOURCE[0]}")/util/lib.sh"

msrv="${1:-1.88.0}"

# If not on CI - create temp dir
if [[ ! -v CI ]]; then
    trap cleanup SIGINT SIGTERM ERR EXIT

    temp_dir=$(mktemp -d)

    function cleanup {
        # Unset the trap to prevent an infinite loop
        trap - SIGINT SIGTERM ERR EXIT

        step rm -rf "$temp_dir"
    }

    step cp -r README.md Cargo.toml bon bon-macros "$temp_dir"

    with_log pushd "$temp_dir"

    step echo "$msrv" > rust-toolchain

    info "Running in a temp dir $(pwd)"
fi

step cargo --version --verbose

# Reshape the workspace manifest for this run:
#
# - Trim the members down to the two published crates. The other members
#   (benchmarks, the sandbox, the website doctests) aren't bound by the MSRV,
#   and their dependencies can't be pinned to MSRV-compatible versions.
#
# - Drop the `clippy` and `rustdoc` lints. Their config is written for the dev
#   toolchain, and lint names change between releases, so it isn't necessarily
#   valid for the MSRV one. The `rust` lints are kept, because they carry the
#   `check-cfg` config, without which the build warns about `cfg(nightly)`.
info "Reshaping the workspace manifest for the MSRV run"

awk '
    /^members = \[/ { print "members = [\"bon\", \"bon-macros\"]"; in_members = 1; next }
    in_members && /^\]/ { in_members = 0; next }
    in_members { next }

    /^\[/ { in_dev_lints = ($0 ~ /^\[workspace\.lints\.(clippy|rustdoc)\]/) }
    !in_dev_lints { print }
' Cargo.toml > Cargo.toml.msrv

mv Cargo.toml.msrv Cargo.toml

with_log cd bon

step cargo update --precise 1.0.10  -p dissimilar
step cargo update --precise 0.24.1  -p darling
step cargo update --precise 1.0.22  -p unicode-ident
step cargo update --precise 1.0.15  -p itoa
step cargo update --precise 1.0.101 -p proc-macro2
step cargo update --precise 1.0.40  -p quote
step cargo update --precise 1.17.2  -p once_cell
step cargo update --precise 1.0.89  -p trybuild
step cargo update --precise 1.0.143 -p serde_json
step cargo update --precise 1.0.20  -p ryu
step cargo update --precise 1.0.194 -p serde
step cargo update --precise 0.3.0   -p prettyplease
step cargo update --precise 3.0.3   -p syn@3
step cargo update --precise 1.29.1  -p tokio
step cargo update --precise 1.4.1   -p expect-test
step cargo update --precise 0.52.0  -p windows-sys
step cargo update --precise 0.2.163 -p libc
step cargo update --precise 0.3.2   -p glob

export RUSTFLAGS="${RUSTFLAGS:-} --allow unknown-lints"

features=experimental-overwritable,experimental-generics-setters

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
