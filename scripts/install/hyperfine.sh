#!/usr/bin/env bash

set -euo pipefail

. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

: "${TOOL_VERSION:=1.18.0}"

base_url=https://github.com/sharkdp/hyperfine/releases/download/v$TOOL_VERSION
file_stem=hyperfine-v$TOOL_VERSION-$arch_rust-unknown-linux-musl

download_and_decompress \
    "$base_url/$file_stem.tar.gz" \
    --strip-components 1 \
    "$file_stem/hyperfine"

move_exe_to_path hyperfine
