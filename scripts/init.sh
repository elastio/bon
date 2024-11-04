#!/usr/bin/env bash
# Script to initialize the repo for development.

set -euxo pipefail

# Install prettier
npm ci

# Install the pre-commit hook
ln -s ../../.githooks/pre-commit .git/hooks/pre-commit
