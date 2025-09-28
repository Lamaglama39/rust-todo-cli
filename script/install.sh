#!/bin/bash

set -e

PROJECT_NAME="todo-cli"
INSTALL_DIR="$HOME/.local/bin"
GITHUB_REPO="mol/todo-cli"  # リポジトリ名を適切に変更してください

detect_platform() {
    local os=$(uname -s | tr '[:upper:]' '[:lower:]')
    local arch=$(uname -m)

    case "$os" in
        linux*)
            case "$arch" in
                x86_64) echo "linux-x86_64" ;;
                aarch64|arm64) echo "linux-aarch64" ;;
                *) echo "unsupported" ;;
            esac
            ;;
        darwin*)
            case "$arch" in
                x86_64) echo "macos-x86_64" ;;
                arm64) echo "macos-aarch64" ;;
                *) echo "unsupported" ;;
            esac
            ;;
        mingw*|msys*|cygwin*)
            case "$arch" in
                x86_64) echo "windows-x86_64.exe" ;;
                *) echo "unsupported" ;;
            esac
            ;;
        *)
            echo "unsupported"
            ;;
    esac
}

download_binary() {
    echo "Attempting to download pre-built binary..."

    local platform=$(detect_platform)
    if [ "$platform" = "unsupported" ]; then
        echo "Platform not supported for pre-built binaries. Falling back to building from source."
        return 1
    fi

    # 最新リリースのタグを取得
    local latest_tag
    if command -v curl &> /dev/null; then
        latest_tag=$(curl -s "https://api.github.com/repos/$GITHUB_REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"tag_name": "([^"]+)".*/\1/')
    elif command -v wget &> /dev/null; then
        latest_tag=$(wget -qO- "https://api.github.com/repos/$GITHUB_REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"tag_name": "([^"]+)".*/\1/')
    else
        echo "Error: curl or wget is required to download binaries."
        return 1
    fi

    if [ -z "$latest_tag" ]; then
        echo "Could not determine latest release. Falling back to building from source."
        return 1
    fi

    local binary_name="$PROJECT_NAME-$platform"
    local download_url="https://github.com/$GITHUB_REPO/releases/download/$latest_tag/$binary_name"

    echo "Downloading $binary_name from $download_url..."

    if command -v curl &> /dev/null; then
        if curl -L "$download_url" -o "/tmp/$PROJECT_NAME" --fail; then
            return 0
        else
            echo "Download failed. Falling back to building from source."
            return 1
        fi
    elif command -v wget &> /dev/null; then
        if wget "$download_url" -O "/tmp/$PROJECT_NAME"; then
            return 0
        else
            echo "Download failed. Falling back to building from source."
            return 1
        fi
    fi

    return 1
}

build_from_source() {
    echo "Building $PROJECT_NAME from source..."

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

    cp "target/release/$PROJECT_NAME" "/tmp/$PROJECT_NAME"
}

echo "Installing $PROJECT_NAME..."

# バイナリのダウンロードを試行、失敗した場合はソースからビルド
if ! download_binary; then
    build_from_source
fi

mkdir -p "$INSTALL_DIR"

echo "Installing binary to $INSTALL_DIR..."
chmod +x "/tmp/$PROJECT_NAME"
mv "/tmp/$PROJECT_NAME" "$INSTALL_DIR/"

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