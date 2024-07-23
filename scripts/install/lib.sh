#!/usr/bin/env bash

. "$(dirname "${BASH_SOURCE[0]}")/../../scripts/util/lib.sh"

step curl --version
step grep --version
step tar --version

curl_version=$(curl --version | grep -o 'curl [0-9]\+\.[0-9]\+\.[0-9]\+')
curl_major=$(echo "$curl_version" | grep -o ' [0-9]\+\.'  | grep -o '[0-9]\+')
curl_minor=$(echo "$curl_version" | grep -o '\.[0-9]\+\.' | grep -o '[0-9]\+')

# We don't need such a condition in windows installation script because the
# version of curl on windows 2019 is quite recent (8.2.1). However the GitHub
# OS image for ubuntu-20.04 uses curl 7.68.0, and this is the environment where
# this if condition is needed.
if [[ "$curl_major" -lt 7 || $curl_major -eq 7 && "$curl_minor" -lt 71 ]]; then
    echo -e "
\033[33;1m[WARN] Installed curl version is $curl_major.$curl_minor, but \
--retry-all-errors option is supported only since curl 7.71, so this option \
will not be set. This means that if the download fails due to an error HTTP status \
code, it won't be retried. The script will retry only 'connection refused' errors.\033[0m
" >&2
    retry_all_errors="--retry-connrefused"
else
    retry_all_errors="--retry-all-errors"
fi

# Determine the OS and arch

case "$OSTYPE" in
    linux*)  export os=linux ;;
    darwin*) export os=darwin ;;
    msys)    export os=windows ;;
    *)       echo "Unknown OS: $OSTYPE" && exit 1 ;;
esac

case $os in
    linux)   export triple_rust=unknown-linux-gnu ;;
    darwin)  export triple_rust=apple-darwin ;;
    windows) export triple_rust=pc-windows-msvc ;;
esac

# Get a Rust-style arch name, e.g. x86_64, aarch64, etc.
arch_rust=$(uname -m | sed "s/arm64/aarch64/")
export arch_rust

# The target triple for the current machine using the Rust convention
export triple_rust="$arch_rust-$triple_rust"

if [[ $os == "windows" ]]; then
    export exe=.exe
else
    export exe=
fi

info "Running on $os ($triple_rust)"

info "Using temporary directory ${SCRIPT_TEMP:-/tmp}"

function download_and_decompress {
    with_backoff try_download_and_decompress "$@"
}

function try_download_and_decompress {
    local hash_algo=""
    while [[ "$#" -gt 0 ]]; do
        case $1 in
        --check-hash)
            hash_algo="$2"
            shift 2
            ;;
        *)
            break
            ;;
        esac
    done

    local url="$1"
    shift

    # Switch to the temporary directory. All file operations must be placed after this command.
    pushd "${SCRIPT_TEMP:-/tmp}" > /dev/null

    local archive
    archive=$(basename "$url")

    curl_with_retry "$url" --remote-name

    # Check the hash of the downloaded file if it was requested
    if [[ "$hash_algo" != "" ]]
    then
        hash=$(curl_with_retry "$url.$hash_algo")
        echo "$hash $archive" | step "${hash_algo}sum" --check
    fi

    if [[ $url == *.tar.* || $url == *.tgz ]]
    then
        step tar --extract --file "$archive" "$@"
    elif [[ $url == *.gz ]]
    then
        step gzip --decompress --stdout "$archive" > "$(basename "$url" .gz)"
    elif [[ $url == *.zip ]]
    then
        step unzip "$archive" "$@"
    else
        echo "Unknown file type: $url"
        exit 1
    fi

    rm "$archive"

    # Return to the original directory. Must be the last command in the function.
    popd > /dev/null
}

function curl_with_retry {
    step curl \
        --location \
        --silent \
        --fail \
        --show-error \
        --retry 5 \
        $retry_all_errors \
        "$@"
}

function move_exe_to_path {
    local exe_path="$1$exe"
    : "${TOOL_OUT_DIR:="${CARGO_HOME-$HOME/.cargo}/bin"}"

    step mkdir -p "$TOOL_OUT_DIR"

    # We need the absolute path to the output directory
    # because we will change the current directory and relative paths will not work
    # Use trick with cd because we can't use readlink -f on macos
    TOOL_OUT_DIR_ABS=$(cd "$TOOL_OUT_DIR"; pwd)

    # Switch to the temporary directory before file operations
    pushd "${SCRIPT_TEMP:-/tmp}" > /dev/null

    step chmod +x "$exe_path"
    step mv "$exe_path" "$TOOL_OUT_DIR_ABS"

    # Return to the original directory. Must be the last command in the function.
    popd > /dev/null
}
