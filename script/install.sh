#!/bin/bash

set -e

PROJECT_NAME="todo-cli"
INSTALL_DIR="$HOME/.local/bin"

echo "Installing $PROJECT_NAME..."

if ! command -v cargo &> /dev/null; then
    echo "Error: Rust/Cargo is not installed. Please install Rust first."
    echo "Visit https://rustup.rs/ to install Rust."
    exit 1
fi

echo "Building $PROJECT_NAME in release mode..."
cargo build --release

if [ ! -f "target/release/$PROJECT_NAME" ]; then
    echo "Error: Build failed or binary not found."
    exit 1
fi

mkdir -p "$INSTALL_DIR"

echo "Installing binary to $INSTALL_DIR..."
cp "target/release/$PROJECT_NAME" "$INSTALL_DIR/"

if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
    echo ""
    echo "Installation complete!"
    echo ""
    echo "Note: $INSTALL_DIR is not in your PATH."
    echo "To use $PROJECT_NAME from anywhere, add the following line to your shell profile:"
    echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
    echo ""
    echo "For bash, add it to ~/.bashrc"
    echo "For zsh, add it to ~/.zshrc"
    echo ""
    echo "Then restart your terminal or run: source ~/.bashrc (or ~/.zshrc)"
else
    echo ""
    echo "Installation complete!"
    echo "You can now use '$PROJECT_NAME' from anywhere in your terminal."
fi

echo ""
echo "To verify the installation, run: $PROJECT_NAME --help"