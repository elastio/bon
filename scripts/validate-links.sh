#!/usr/bin/env bash
#
# Validate for broken links (mostly broken anchors)

set -euo pipefail

cd website

npm run build

FORCE_COLOR=2 node \
    ./validate-links.mts
