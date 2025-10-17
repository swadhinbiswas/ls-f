#!/usr/bin/env bash
set -euo pipefail

# Standalone installer for ls-f
# Downloads both lsf and column.py, then installs them

REPO_URL="https://raw.githubusercontent.com/swadhinbiswas/ls-f/main"
INSTALL_MODE="${1:-user}"

echo "🚀 ls-f Standalone Installer"
echo "Downloading lsf and column.py from GitHub..."

# Determine install directory
if [[ "$INSTALL_MODE" == "system" || "$EUID" -eq 0 ]]; then
    BIN_DIR="/usr/local/bin"
    echo "📍 Installing to: $BIN_DIR (system-wide)"
    SUDO_CMD="sudo"
else
    BIN_DIR="$HOME/.local/bin"
    echo "📍 Installing to: $BIN_DIR (user-local)"
    SUDO_CMD=""
fi

# Create directory
$SUDO_CMD mkdir -p "$BIN_DIR"

# Download files
echo "⬇️  Downloading lsf..."
$SUDO_CMD curl -fL "$REPO_URL/lsf" -o "$BIN_DIR/lsf"

echo "⬇️  Downloading column.py..."
$SUDO_CMD curl -fL "$REPO_URL/column.py" -o "$BIN_DIR/column.py"

# Make executable
echo "🔧 Making files executable..."
$SUDO_CMD chmod +x "$BIN_DIR/lsf" "$BIN_DIR/column.py"

# Create config directory
echo "📁 Creating config directory..."
mkdir -p "$HOME/.config/lsf"

# Check PATH
if [[ "$INSTALL_MODE" != "system" && ":$PATH:" != *":$BIN_DIR:"* ]]; then
    echo "⚠️  Warning: $BIN_DIR is not in your PATH"
    echo "   Add this to your shell config (~/.bashrc, ~/.zshrc, etc.):"
    echo "   export PATH=\"\$HOME/.local/bin:\$PATH\""
fi

# Verify installation
echo "✅ Installation complete!"
echo ""
echo "🧪 Testing installation..."
if command -v "$BIN_DIR/lsf" >/dev/null 2>&1; then
    echo "Version: $("$BIN_DIR/lsf" --version 2>/dev/null || echo "lsf installed")"
    echo ""
    echo "🎉 Try it out:"
    echo "   lsf --help"
    echo "   lsf -x"
    echo ""
    echo "💡 Optional: Set up alias:"
    echo "   echo \"alias ls='lsf'\" >> ~/.bashrc"
else
    echo "❌ Installation verification failed"
    exit 1
fi