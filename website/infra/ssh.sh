#!/usr/bin/env bash

set -euo pipefail

tf_output=$(terraform output -json)

os_user=$(echo "$tf_output" | jq -r '.server_os_user.value')
ip=$(echo "$tf_output" | jq -r '.server_ip.value')

echo "> ssh $os_user@$ip"

ssh -t "$os_user@$ip"
