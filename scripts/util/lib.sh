#!/usr/bin/env bash
# This file is meant to be sourced by other scripts, not executed directly.
# It contains a bunch of helper functions for writing bash scripts.

. "$(dirname "${BASH_SOURCE[0]}")/log.sh"
. "$(dirname "${BASH_SOURCE[0]}")/signal.sh"

# Create a temporary directory for the script.
if [[ -z "${SCRIPT_TEMP:-}" ]]; then
    if [[ -n "${RUNNER_TEMP:-}" ]]; then
        SCRIPT_TEMP="${RUNNER_TEMP}"
    else
        SCRIPT_TEMP="$(mktemp --directory)"
    fi
fi
export SCRIPT_TEMP

# Retry a command a with backoff.
#
# The retry count is given by ATTEMPTS (default 5), the
# initial backoff timeout is given by TIMEOUT in seconds
# (default 1.)
#
# Successive backoffs double the timeout.
#
# Beware of set -e killing your whole script!
#
# Shamelessly copied from https://coderwall.com/p/--eiqg/exponential-backoff-in-bash
function with_backoff {
    local max_attempts=${ATTEMPTS-5}
    local timeout=${TIMEOUT-1}
    local attempt=0
    local exit_code=0

    while [[ $attempt -lt $max_attempts ]]
    do
        if [[ $attempt == 0 ]]; then
            start_group "${@}"
        else
            start_group "[Try $((attempt + 1))/$max_attempts] ${*}"
        fi

        # Temporarily disable the "exit script on error" behavior
        set +o errexit

        "$@"
        exit_code=$?

        # put exit on error back up
        set -o errexit

        end_group

        if [[ $exit_code == 0 ]]; then
            break
        fi

        echo "Failure! Retrying in $timeout seconds.." 1>&2
        sleep "$timeout"
        attempt=$(( attempt + 1 ))
        timeout=$(( timeout * 2 ))
    done

    if [[ $exit_code != 0 ]]; then
        echo "You've failed me for the last time! (exit code $exit_code) ($*)" 1>&2
    fi

    return $exit_code
}

# A function to run a command in a temporary directory and then return to the
# original directory after the command has finished.
function in_temp_dir {
    pushd "${SCRIPT_TEMP:-/tmp}" > /dev/null

    # Temporarily disable the "exit script on error" behavior
    set +o errexit

    "$@"
    exit_code=$?

    # put exit on error back up
    set -o errexit

    popd > /dev/null

    if [[ $exit_code != 0 ]]; then
        echo "Command has failed! (exit code $exit_code) ($*)" 1>&2
        exit $exit_code
    fi
}


# A generic step of execution in the script that should be logged,
# and also forward signals to the child process
function step {
    # forward_signals spawns a background process. We want `with_log` in that
    # process to be replaced with the invoked command so that it receives the
    # signals directly.
    forward_signals with_log exec "$@"
}
