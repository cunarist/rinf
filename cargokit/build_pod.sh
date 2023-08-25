#!/bin/sh
set -e

BASEDIR=$(dirname "$0")

# Remove XCode SDK from path. Otherwise this breaks tool compilation when building iOS project
NEW_PATH=`echo $PATH | tr ":" "\n" | grep -v "Contents/Developer/" | tr "\n" ":"`

export PATH=${NEW_PATH%?} # remove trailing :

export CARGOKIT_PLATFORM_NAME=$PLATFORM_NAME
export CARGOKIT_ARCHS=$ARCHS
export CARGOKIT_CONFIGURATION=$CONFIGURATION
export CARGOKIT_SRCROOT=$PODS_TARGET_SRCROOT
export CARGOKIT_TEMP_DIR=$TARGET_TEMP_DIR
export CARGOKIT_PRODUCT_NAME=$PRODUCT_NAME
export CARGOKIT_TARGET_DIR=$PODS_CONFIGURATION_BUILD_DIR
export CARGOKIT_TOOL_TEMP_DIR=$TARGET_TEMP_DIR/rust_tool

"$BASEDIR/run_rust_tool.sh" build_pod $@

# Make a symlink from built framework to phony file, which will be used as input to
# build script. This should force rebuild (podspec currently doesn't support alwaysOutOfDate
# attribute on custom build phase)
ln -Fs "${BUILT_PRODUCTS_DIR}/${EXECUTABLE_PATH}" "${BUILT_PRODUCTS_DIR}/cargokit_phony"
ln -Fs "${BUILT_PRODUCTS_DIR}/${EXECUTABLE_PATH}" "${BUILT_PRODUCTS_DIR}/cargokit_phony_out"
