#!/usr/bin/env bash

set -euo pipefail

version=$(cargo pkgid bon | cut --delimiter=# --fields=2 | grep -oE '^[0-9]+\.[0-9]+')

echo "Setting version $version"

sed -i "s/bon = \".*\"/bon = \"$version\"/" \
    README.md
