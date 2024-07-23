#!/usr/bin/env bash
# The signal handling logic is inspired by http://veithen.io/2014/11/16/sigterm-propagation.html

. "$(dirname "${BASH_SOURCE[0]}")/log.sh"

# Taken from https://stackoverflow.com/a/9256709
trap_with_arg() {
    local trap_func=$1
    shift
    for sig ; do
        # We want the trap function to be interpolated before the trap is executed
        # shellcheck disable=SC2064
        trap "$trap_func $sig" "$sig"
    done
}

function forward_signals {
    local cmd
    if [[ "$1" == with_log ]]; then
        if [[ $2 == exec ]]; then
            cmd=( "${@:3:2}" )
        else
            cmd=( "${@:2:2}" )
        fi
    else
        cmd=( "${@:1:2}" )
    fi

    # The command is actually reachable, because it's used in a trap.
    # Shellcheck docs recommend disabling it if this is the case.
    # shellcheck disable=SC2317
    function signal_handler() {
        local signal=$1

        colorized_cmd=$(colorize_command "${cmd[@]}")

        echo -e "⛔ Sending \033[31;1mSIG$signal\033[0m to process $colorized_cmd (PID $pid, caught SIG$signal)" >&2
        kill "-$signal" "$pid" || true
    }

    trap_with_arg signal_handler TERM INT QUIT HUP

    "$@" &

    pid=$!

    # The double-wait is explained in the signal handling article linked in the comment at the top of the file
    # Need to temporarily disable `errexit` because even if the command exited with an error code we don't
    # want our script to terminate.
    set +o errexit
    wait $pid
    wait $pid

    # Save the exit code of the command, whether or not it was success
    exit_code=$?
    set -o errexit

    return $exit_code
}
