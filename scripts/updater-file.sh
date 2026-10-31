#!/usr/bin/env bash
# Prints the file the updater reads, for the tag in $1 and the archive's signature file in $2. The
# one universal archive serves both architectures.
set -euo pipefail

tag=$1
signature=$(cat "$2")
url="https://github.com/sm24jv/trygga-mac/releases/download/$tag/trygga.app.tar.gz"

jq -n --arg version "${tag#v}" --arg url "$url" --arg signature "$signature" '
  {url: $url, signature: $signature} as $archive
  | {version: $version, platforms: {"darwin-aarch64": $archive, "darwin-x86_64": $archive}}'
