#!/usr/bin/env bash

set -euo pipefail

. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

: "${TOOL_VERSION:=$(cargo pkgid gungraun | cut -d@ -f2)}"

base_url=https://github.com/gungraun/gungraun/releases/download/v$TOOL_VERSION
file_stem=gungraun-runner-v$TOOL_VERSION-x86_64-unknown-linux-gnu

download_and_decompress \
    --check-hash sha256 \
    "$base_url/$file_stem.tar.gz" \
    --strip-components 1 \
    "$file_stem/gungraun-runner"

move_exe_to_path gungraun-runner
