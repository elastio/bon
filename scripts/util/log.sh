#!/usr/bin/env bash

# This output stream is used by subshells to send their output to the
# global build process stdout. This is needed e.g. for writing special commands to
# let the github actions runner read them
global_stdout=3
eval "exec $global_stdout>&1"

# Log a message at the info level
function info {
    local message=$1

    echo -e "\033[32;1m[INFO]\033[0m $message" >&2
}

# Log the command and execute it
function with_log {
    local cmd="$1"
    if [[ $cmd == exec ]]; then
        cmd=("${@:2}")
    else
        cmd=("${@:1}")
    fi

    colorized_cmd=$(colorize_command "${cmd[@]}")

    >&$global_stdout echo -e "\033[32;1m❱\033[0m $colorized_cmd" >&2

    "$@"
}

# Log the command and execute it, but collapse the output on CI
function with_collapsed_log {
    start_group "$@"

    # Temporarily disable the "exit script on error" behavior
    set +o errexit

    "$@"
    local exit_code=$?

    # put exit on error back up
    set -o errexit

    end_group

    return $exit_code
}

function group_header {
    command=$(colorize_command "$@")

    echo -e "\033[32;1m👉 ❱❱❱❱ $command"
}

# Begin a collapsible group. You'll need to click on the logs to expand them on CI
#
# Beware that it's not possible to nest groups. Two consecutive start_group calls are wrong.
function start_group {
    local group
    group=$(group_header "${@}")

    if [ "${GITHUB_ACTIONS:-false}" == "true" ]; then
        >&$global_stdout echo -e "::group::${group}"
    else
        >&$global_stdout echo -e "${group}"
    fi
}

# Finish the previously started collapsible group.
#
# Beware that it's not possible to nest groups, this closes all groups
function end_group {
    if [ "${GITHUB_ACTIONS:-false}" == "true" ]; then
        >&$global_stdout echo "::endgroup::"
    fi
}

# Write down some info to the job's summary
function write_to_summary {
    if [ "${GITHUB_ACTIONS:-false}" == "true" ]; then
        echo "${@}" >> "$GITHUB_STEP_SUMMARY"
    fi
}

# Returns a command with syntax highlighting
function colorize_command {
    local program=$1
    shift

    local args=()
    for arg in "$@"; do
        if [[ $arg =~ ^- ]]; then
            args+=("\033[34;1m${arg}\033[0m")
        else
            args+=("\033[0;33m${arg}\033[0m")
        fi
    done

    # On old versions of bash, for example 4.2.46 if the `args` array
    # is empty, then an `unbound variable` is thrown.
    #
    # Luckily, we don't pass commands without positional arguments to this function.
    # If this ever becomes a problem, you know the why and you'll hopefully fix it 😓.
    echo -e "\033[1;32m${program}\033[0m ${args[*]}"
}
