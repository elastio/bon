#!/usr/bin/env bash

set -euo pipefail

. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

: "${TOOL_VERSION:=0.9.2}"

# The release has only a musl binary for x86_64 and only a gnu binary for aarch64
libc=musl
if [[ $arch_rust == aarch64 ]]; then
    libc=gnu
fi

base_url=https://github.com/bnjbvr/cargo-machete/releases/download/v$TOOL_VERSION
file_stem=cargo-machete-v$TOOL_VERSION-$arch_rust-unknown-linux-$libc

download_and_decompress \
    --check-hash sha256 \
    "$base_url/$file_stem.tar.gz" \
    --strip-components 1 \
    "$file_stem/cargo-machete"

move_exe_to_path cargo-machete
