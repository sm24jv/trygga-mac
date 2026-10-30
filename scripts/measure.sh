#!/usr/bin/env bash
# Runs the app in the disk image $1 for $2 minutes, with its logs in $3, and always exits 0: a
# machine that cannot run a window is an answer, not a failure.
set -uo pipefail

dmg=$1
minutes=$2
out=$3

mkdir -p "$out"
sw_vers > "$out/macos.txt"
mount=$(hdiutil attach "$dmg" -nobrowse | grep -o '/Volumes/.*$')

app="$mount/trygga.app"
executable=$(/usr/libexec/PlistBuddy -c 'Print :CFBundleExecutable' "$app/Contents/Info.plist")

TRYGGA_MEASURE_DIR=$out "$app/Contents/MacOS/$executable" \
  > "$out/app-stdout.log" 2>&1 &
pid=$!

sleep $((minutes * 60))

if kill -0 "$pid" 2>/dev/null; then
  echo "running" > "$out/app-state.txt"
  kill "$pid"
else
  wait "$pid"
  echo "exited with $?" > "$out/app-state.txt"
fi
exit 0
