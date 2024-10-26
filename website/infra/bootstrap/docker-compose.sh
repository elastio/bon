#!/usr/bin/env bash

set -euo pipefail

echo "Running: docker compose $@"

CURRENT_UID=$(id -u):$(id -g) docker compose $@
