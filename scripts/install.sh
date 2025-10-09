#!/bin/bash

# This script builds the Rustlator project and installs the binary to ~/.local/bin
# It also creates a default config file if it does not exist

set -e

echo "Building Rustlator..."
cargo build --release

BIN_PATH="target/release/rl"
INSTALL_DIR="$HOME/.local/bin"
CONFIG_DIR="$HOME/.rustlator"
CONFIG_FILE="$CONFIG_DIR/config.json"

echo "Installing binary to $INSTALL_DIR"
mkdir -p "$INSTALL_DIR"
cp "$BIN_PATH" "$INSTALL_DIR"

if [ ! -f "$CONFIG_FILE" ]; then
    echo "Creating default config file at $CONFIG_FILE"
    mkdir -p "$CONFIG_DIR"
    cat > "$CONFIG_FILE" <<EOF
{
  "api_url": "http://localhost:5000/",
  "from": "en",
  "to": "es",
  "alternatives": 3
}
EOF
else
    echo "Config file already exists at $CONFIG_FILE"
fi

echo "Installation complete. Make sure $INSTALL_DIR is in your PATH."