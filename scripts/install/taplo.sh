#!/usr/bin/env bash

set -euo pipefail

. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

: "${TOOL_VERSION:=0.9.2}"

base_url=https://github.com/tamasfe/taplo/releases/download/$TOOL_VERSION

file_stem="taplo-linux-$arch_rust"

download_and_decompress "$base_url/$file_stem.gz"

in_temp_dir mv "$file_stem" taplo

move_exe_to_path taplo
