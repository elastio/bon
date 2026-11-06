#!/usr/bin/env bash

set -euo pipefail

. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

: "${TOOL_VERSION:=1.52.0}"

# The release has only musl binaries for Linux
target=${triple_rust/%linux-gnu/linux-musl}

base_url=https://github.com/crate-ci/typos/releases/download/v$TOOL_VERSION
file_stem=typos-v$TOOL_VERSION-$target

if [[ $os == windows ]]; then
    archive=$file_stem.zip
    member=typos.exe
else
    archive=$file_stem.tar.gz
    member=./typos
fi

download_and_decompress "$base_url/$archive" "$member"

move_exe_to_path typos
