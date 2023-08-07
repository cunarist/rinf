#!/bin/sh

set -e

BASEDIR=$(dirname "$0")

# Only use rustup cargo installation. This is to avoid picking up homebrew's cargo,
# which does not support rustup installed targets.
CARGO="$HOME/.cargo/bin/cargo"

if ! command -v $CARGO --version >/dev/null 2>&1
then
    echo "|"
    echo "| cargo not found. Expected in $CARGO."
    echo "|"
    echo "| Maybe you need to install Rust? It only takes a minute:"
    echo "|"
    echo "> curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    echo "|"
    exit 1
fi

$CARGO run --manifest-path=$BASEDIR/build_tool/Cargo.toml --bin build_tool --target-dir=$CARGOKIT_TOOL_TEMP_DIR --quiet -- $@
