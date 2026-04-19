#!/usr/bin/env bash
set -euo pipefail

PLUGIN_ROOT="$1"
PLATFORM="$2"

if [[ "$PLATFORM" == "ios" ]]; then
  PACKAGE_DIR="$PLUGIN_ROOT/ios/rinf"
  TARGETS=("arm64-apple-ios")
elif [[ "$PLATFORM" == "macos" ]]; then
  PACKAGE_DIR="$PLUGIN_ROOT/macos/rinf"
  TARGETS=("arm64-apple-macosx" "x86_64-apple-macosx")
else
  echo "Unsupported platform: $PLATFORM" >&2
  exit 1
fi

XCFRAMEWORK_DIR="$PACKAGE_DIR/Binaries/hub.xcframework"
BUILD_DIR="$PACKAGE_DIR/Binaries/build-placeholder"
HEADERS_DIR="$BUILD_DIR/Headers"
SRC_FILE="$BUILD_DIR/placeholder.c"
OBJ_DIR="$BUILD_DIR/objects"
LIB_DIR="$BUILD_DIR/libs"
LIB_NAME="libhub.a"

rm -rf "$BUILD_DIR" "$XCFRAMEWORK_DIR"
mkdir -p "$HEADERS_DIR" "$OBJ_DIR" "$LIB_DIR"

cat > "$SRC_FILE" <<'EOF'
void rinf_placeholder(void) {}
EOF

cat > "$HEADERS_DIR/hub.h" <<'EOF'
#ifndef HUB_H_
#define HUB_H_

void rinf_placeholder(void);

#endif  // HUB_H_
EOF

cat > "$HEADERS_DIR/module.modulemap" <<'EOF'
module hub {
  header "hub.h"
  export *
}
EOF

OBJECTS=()
for target in "${TARGETS[@]}"; do
  obj="$OBJ_DIR/${target}.o"
  clang -target "$target" -c "$SRC_FILE" -o "$obj"
  OBJECTS+=("$obj")
done

LIB_PATH="$LIB_DIR/$LIB_NAME"
if [[ ${#OBJECTS[@]} -eq 1 ]]; then
  libtool -static -o "$LIB_PATH" "${OBJECTS[0]}"
else
  lipo -create "${OBJECTS[@]}" -output "$OBJ_DIR/universal.o"
  libtool -static -o "$LIB_PATH" "$OBJ_DIR/universal.o"
fi

xcodebuild -create-xcframework \
  -library "$LIB_PATH" \
  -headers "$HEADERS_DIR" \
  -output "$XCFRAMEWORK_DIR"
