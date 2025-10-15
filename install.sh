#!/usr/bin/env bash

set -euo pipefail

SCRIPT_NAME="lsf"
VERSION="1"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"

usage() {
    cat <<EOF
Usage: $0 [--user | --system] [--replace-ls] [--force] [--no-font-check]

Options:
  --user         Install to user local bin (~/.local/bin). Default when not root.
  --system       Install system-wide to /usr/local/bin. Requires sudo or root.
  --replace-ls   Replace system 'ls' command by creating a wrapper at /usr/local/bin/ls (backups created).
  --force        Overwrite existing files without prompt.
  --no-font-check  Skip checking for Nerd Fonts.
  -h, --help     Show this help message.

This installer will copy the bundled 'lsf' script and 'column.py' helper into the chosen bin directory
and create a config directory at ~/.config/lsf (if not present).
EOF
}

INSTALL_SYSTEM=false
REPLACE_LS=false
FORCE=false
FONT_CHECK=true

while [ "$#" -gt 0 ]; do
    case "$1" in
        --user)
            INSTALL_SYSTEM=false
            shift
            ;;
        --system)
            INSTALL_SYSTEM=true
            shift
            ;;
        --replace-ls)
            REPLACE_LS=true
            shift
            ;;
        --force)
            FORCE=true
            shift
            ;;
        --no-font-check)
            FONT_CHECK=false
            shift
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            echo "Unknown arg: $1"
            usage
            exit 2
            ;;
    esac
done

# Decide default install dir
if [ "$INSTALL_SYSTEM" = true ] || [ "$(id -u)" -eq 0 ]; then
    BIN_DIR="/usr/local/bin"
else
    BIN_DIR="$HOME/.local/bin"
fi

CONFIG_DIR="$HOME/.config/lsf"
CONFIG_FILE="$CONFIG_DIR/lsf.conf"

if [ ! -f "$SCRIPT_DIR/lsf" ]; then
    echo "Error: unable to locate 'lsf' next to install.sh. Please run this installer from an ls-f checkout." >&2
    exit 1
fi

echo "Installing to: $BIN_DIR"

if [ "$FONT_CHECK" = true ]; then
    if ! command -v fc-list >/dev/null 2>&1; then
        echo "Warning: 'fc-list' not available; skipping Nerd Font check. Use --no-font-check to suppress this check." >&2
    else
        if ! fc-list | grep -i "Nerd Font" >/dev/null 2>&1; then
            echo "Warning: Nerd Fonts not detected. Icons may not render correctly." >&2
            echo "Visit https://www.nerdfonts.com/ for installation instructions." >&2
        fi
    fi
fi

# Create config dir and default config
mkdir -p "$CONFIG_DIR"
if [ ! -f "$CONFIG_FILE" ]; then
    cat > "$CONFIG_FILE" <<'EOL'
# Config file for lsf
# You can override the default icons and colors here.
# Example:
# ICONS["dir"]=""
# COLORS["dir"]="\e[31m"
EOL
    echo "Created default config at $CONFIG_FILE"
fi

# Ensure bin dir exists
mkdir -p "$BIN_DIR"

install_file() {
    local src="$1"
    local dest="$BIN_DIR/$(basename "$src")"
    if [ -e "$dest" ] && [ "$FORCE" = false ]; then
        read -r -p "$dest exists. Overwrite? [y/N] " ans || ans=n
        if [[ ! "$ans" =~ ^[Yy]$ ]]; then
            echo "Skipping $dest"
            return
        fi
    fi
    cp "$src" "$dest"
    chmod +x "$dest"
    echo "Installed $dest"
}

# Install main script and helper (column.py) if present
install_file "$SCRIPT_DIR/lsf"
if [ -f "$SCRIPT_DIR/column.py" ]; then
    install_file "$SCRIPT_DIR/column.py"
fi

if [ "$REPLACE_LS" = true ]; then
    # Only allow replace-ls when installing system-wide
    if [ "$BIN_DIR" != "/usr/local/bin" ]; then
        echo "--replace-ls requires system installation (run with --system or sudo)." >&2
    else
        LS_WRAPPER="/usr/local/bin/ls"
        if [ -e "$LS_WRAPPER" ] && [ "$FORCE" = false ]; then
            read -r -p "$LS_WRAPPER exists. Backup and replace it? [y/N] " ans || ans=n
            if [[ ! "$ans" =~ ^[Yy]$ ]]; then
                echo "Skipping replacement of ls."
            else
                mv "$LS_WRAPPER" "$LS_WRAPPER.bak.$(date +%s)"
            fi
        fi

        cat > "$LS_WRAPPER" <<'EOF'
#!/usr/bin/env bash
# Wrapper to prefer lsf; falls back to original ls if needed
if command -v lsf >/dev/null 2>&1; then
    lsf "$@"
else
    /bin/ls "$@"
fi
EOF
        chmod +x "$LS_WRAPPER"
        echo "Installed ls wrapper at $LS_WRAPPER (original backed up if present)."
    fi
fi

echo "Installation complete."
if [[ ":$PATH:" != *":$BIN_DIR:"* ]]; then
    echo "Note: $BIN_DIR is not in your PATH. Add it to be able to run 'lsf' without a full path." >&2
fi
echo "To use lsf as your default 'ls', add the following to your shell rc (e.g. ~/.bashrc or ~/.zshrc):"
echo "  alias ls='lsf'"

if [ -f "$SCRIPT_DIR/config/lsf_alias_setup.sh" ]; then
    echo "If you want me to set the alias automatically, run:"
    echo "  source $SCRIPT_DIR/config/lsf_alias_setup.sh"
fi

echo "Verify the installation with:"
echo "  lsf --version"
echo "  lsf -x"

exit 0
