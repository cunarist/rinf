#!/usr/bin/env bash
set -euo pipefail

APP_ROOT="$1"
PLUGIN_ROOT="$2"
PLATFORM_NAME="$3"
CONFIGURATION="$4"
ARCHS="$5"

if [[ "$PLATFORM_NAME" == "macosx" ]]; then
  PACKAGE_DIR="$PLUGIN_ROOT/macos/rinf"
elif [[ "$PLATFORM_NAME" == "iphoneos" || "$PLATFORM_NAME" == "iphonesimulator" ]]; then
  PACKAGE_DIR="$PLUGIN_ROOT/ios/rinf"
else
  echo "Unsupported Apple platform: $PLATFORM_NAME" >&2
  exit 1
fi

MANIFEST_DIR="$APP_ROOT/native/hub"
MANIFEST_PATH="$MANIFEST_DIR/Cargo.toml"
if [[ ! -f "$MANIFEST_PATH" ]]; then
  echo "Missing Rust crate manifest: $MANIFEST_PATH" >&2
  exit 1
fi

CONFIGURATION_LOWER="$(printf '%s' "$CONFIGURATION" | cut -d- -f1 | tr '[:upper:]' '[:lower:]')"
if [[ "$CONFIGURATION_LOWER" == "debug" ]]; then
  CARGO_PROFILE="debug"
  CARGO_PROFILE_FLAG=""
else
  CARGO_PROFILE="release"
  CARGO_PROFILE_FLAG="--release"
fi

rust_target() {
  local platform="$1"
  local arch="$2"
  case "$platform:$arch" in
    macosx:arm64) echo "aarch64-apple-darwin" ;;
    macosx:x86_64) echo "x86_64-apple-darwin" ;;
    iphoneos:arm64) echo "aarch64-apple-ios" ;;
    iphonesimulator:arm64) echo "aarch64-apple-ios-sim" ;;
    iphonesimulator:x86_64) echo "x86_64-apple-ios" ;;
    *)
      echo "Unsupported Apple target: $platform $arch" >&2
      exit 1
      ;;
  esac
}

XCFRAMEWORK_DIR="$PACKAGE_DIR/Binaries/hub.xcframework"
BUILD_ROOT="$PACKAGE_DIR/Binaries/build"
HEADERS_DIR="$BUILD_ROOT/Headers"
TARGET_DIR="$BUILD_ROOT/target"
LIB_NAME="libhub.a"

mkdir -p "$HEADERS_DIR" "$TARGET_DIR"

cat > "$HEADERS_DIR/hub.h" <<'EOF'
#ifndef HUB_H_
#define HUB_H_

// Rinf links the Rust static library but does not expose C headers from it.

#endif  // HUB_H_
EOF

cat > "$HEADERS_DIR/module.modulemap" <<'EOF'
module hub {
  header "hub.h"
  export *
}
EOF

LIBRARY_ARGS=()
BUILT_LIBRARIES=()
for arch in $ARCHS; do
  TARGET="$(rust_target "$PLATFORM_NAME" "$arch")"
  rustup target add "$TARGET"
  cargo build \
    --manifest-path "$MANIFEST_PATH" \
    --locked \
    -p hub \
    --target "$TARGET" \
    --target-dir "$TARGET_DIR" \
    ${CARGO_PROFILE_FLAG}
  BUILT_LIB="$TARGET_DIR/$TARGET/$CARGO_PROFILE/$LIB_NAME"
  BUILT_LIBRARIES+=("$BUILT_LIB")
done

MERGED_LIB="$BUILD_ROOT/$LIB_NAME"
if [[ ${#BUILT_LIBRARIES[@]} -eq 1 ]]; then
  cp "${BUILT_LIBRARIES[0]}" "$MERGED_LIB"
else
  lipo -create "${BUILT_LIBRARIES[@]}" -output "$MERGED_LIB"
fi

rm -rf "$XCFRAMEWORK_DIR"
xcodebuild -create-xcframework -library "$MERGED_LIB" -headers "$HEADERS_DIR" -output "$XCFRAMEWORK_DIR"
