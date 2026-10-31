#!/usr/bin/env bash
# Fails, saying why, when the tag in $1 misses what a release needs. Runs first, so a release that
# cannot work costs no build. A key that doesn't match the public key only shows in the Mac run.
set -euo pipefail

tag=$1
config=src-tauri/tauri.conf.json

refuse() {
  echo "::error::$1"
  exit 1
}

[ -n "${TAURI_SIGNING_PRIVATE_KEY:-}" ] ||
  refuse "The secret TAURI_SIGNING_PRIVATE_KEY is not set, so the update cannot be signed."
[ -n "${TAURI_SIGNING_PRIVATE_KEY_PASSWORD:-}" ] ||
  refuse "The secret TAURI_SIGNING_PRIVATE_KEY_PASSWORD is not set, so the key cannot be opened."
[ -n "$(jq -r '.plugins.updater.pubkey' "$config")" ] ||
  refuse "plugins.updater.pubkey in $config is empty, so no app could check an update."

version=$(jq -r '.version' "$config")
[ "$tag" = "v$version" ] ||
  refuse "The tag $tag does not match version $version in $config: installed apps compare that version."
