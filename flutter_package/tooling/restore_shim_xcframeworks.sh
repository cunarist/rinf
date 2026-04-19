#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$1"

if [[ -d "$REPO_ROOT/.git" ]]; then
  git -C "$REPO_ROOT" restore --source=HEAD -- \
    flutter_package/ios/rinf/Binaries/hub.xcframework \
    flutter_package/macos/rinf/Binaries/hub.xcframework
  exit 0
fi

echo "Expected a git checkout at $REPO_ROOT for shim restore" >&2
exit 1
