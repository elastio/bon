#!/usr/bin/env bash

set -euxo pipefail

# cargo t -p bon --test integration -- ui::ui trybuild=attr_with --nocapture

cargo check -p e2e-tests
