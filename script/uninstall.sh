#!/bin/bash

PROJECT_NAME="todo-cli"
INSTALL_DIR="$HOME/.local/bin"
BINARY_PATH="$INSTALL_DIR/$PROJECT_NAME"

echo "Uninstalling $PROJECT_NAME..."

if [ -f "$BINARY_PATH" ]; then
    echo "Removing binary from $INSTALL_DIR..."
    rm "$BINARY_PATH"
    echo "$PROJECT_NAME has been successfully uninstalled."
else
    echo "Warning: $PROJECT_NAME binary not found at $BINARY_PATH"
    echo "It may have already been removed or installed in a different location."
fi

echo ""
echo "Note: This script only removes the binary file."
echo "If you added $INSTALL_DIR to your PATH in your shell profile"
echo "(~/.bashrc, ~/.zshrc, etc.), you may want to remove that line manually"
echo "if you're not using any other binaries from that directory."
echo ""
echo "Uninstallation complete!"