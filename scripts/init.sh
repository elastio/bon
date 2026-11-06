#!/usr/bin/env bash
# Script to initialize the repo for development.

set -euxo pipefail

# Installs prettier (see top-level package.json)
npm ci

scripts/install/typos.sh
scripts/install/taplo.sh

# Install the pre-commit hook
ln -sf ../../.githooks/pre-commit .git/hooks/pre-commit
