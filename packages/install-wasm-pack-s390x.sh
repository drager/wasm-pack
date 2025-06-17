#!/bin/bash
# wasm-pack s390x installation script

set -e

PACKAGE_FILE="wasm-pack-0.13.1-s390x-unknown-linux-gnu.tar.gz"
INSTALL_DIR="/usr/local/bin"

echo "🚀 Installing wasm-pack for s390x..."

if [ ! -f "$PACKAGE_FILE" ]; then
    echo "❌ Package file $PACKAGE_FILE not found!"
    echo "Please download it first."
    exit 1
fi

echo "📦 Extracting package..."
tar -xzf "$PACKAGE_FILE"

echo "📁 Installing to $INSTALL_DIR..."
sudo cp wasm-pack-0.13.1-s390x-unknown-linux-gnu/wasm-pack "$INSTALL_DIR/"
sudo chmod +x "$INSTALL_DIR/wasm-pack"

echo "🧪 Testing installation..."
wasm-pack --version

echo "✅ wasm-pack installed successfully!"
echo "💡 You can now use: wasm-pack new my-project"
