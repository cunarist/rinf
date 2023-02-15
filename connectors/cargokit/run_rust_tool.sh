#!/bin/sh

set -e

BASEDIR=$(dirname "$0")

# In case we're running in environment without PATH put the default location in there
PATH="$PATH:~/.cargo/bin"

if ! command -v cargo --version >/dev/null 2>&1
then
    echo "|"
    echo "| cargo not found."
    echo "|"
    echo "| We looked in $PATH"
    echo "|"
    echo "| Maybe you need to install Rust? It only takes a minute:"
    echo "|"
    echo "> curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    echo "|"
    exit 1
fi

cargo run --manifest-path=$BASEDIR/build_tool/Cargo.toml --bin build_tool --target-dir=$CARGOKIT_TOOL_TEMP_DIR -- $@