#!/usr/bin/env bash
# Fails unless the app at $1 holds both architectures, each with a valid ad-hoc signature.
set -euo pipefail

app=$1
executable=$(/usr/libexec/PlistBuddy -c 'Print :CFBundleExecutable' "$app/Contents/Info.plist")

for arch in arm64 x86_64; do
  lipo "$app/Contents/MacOS/$executable" -verify_arch "$arch"
  codesign --verify --strict --deep --arch "$arch" "$app"
  codesign --display --verbose=2 --arch "$arch" "$app" 2>&1 | grep -q '^Signature=adhoc$'
  echo "$arch: valid ad-hoc signature"
done
