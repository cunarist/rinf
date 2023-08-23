#!/bin/sh

set -e

BASEDIR=$(dirname "$0")

export PATH="~/.cargo/bin:$PATH"

if ! command -v rustup --version >/dev/null 2>&1
then
    echo "|"
    echo "| rustup not found. Expected in ~/.cargo/bin or \$PATH."
    echo "|"
    echo "| Maybe you need to install Rust? It only takes a minute:"
    echo "|"
    echo "> curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    echo "|"
    exit 1
fi

rustup run stable cargo run --manifest-path="$BASEDIR/build_tool/Cargo.toml" --bin build_tool --target-dir="$CARGOKIT_TOOL_TEMP_DIR" --quiet -- $@
