# Installation Guide for ls-f

This guide covers all installation methods for **ls-f** on Linux, macOS, and other Unix-like systems.

---

## 📋 Prerequisites

Before installing `ls-f`, verify you have:

### Required
- **Bash 4+** or any POSIX shell (Bash, Zsh, Fish, etc.)
- **Python 3.x** (for column formatting)

### Optional (Recommended)
- **Nerd Fonts** installed (for proper icon display)

### Check Prerequisites

```bash
# Check Bash version (should be >= 4)
echo $BASH_VERSION

# Check Python availability
python3 --version

# Check for Nerd Fonts
fc-list | grep -i "Nerd Font"
```

If you don't have Nerd Fonts, download from [nerdfonts.com](https://www.nerdfonts.com/).

---

## 🚀 Installation Methods
git clone https://github.com/swadhinbiswas/ls-f.git
### Method 1: One-Liner Install (Recommended)

Quickest method - downloads both files and installs automatically:

```bash
# User-local installation (no sudo needed)
curl -fsSL https://raw.githubusercontent.com/swadhinbiswas/ls-f/main/install-standalone.sh | bash

# OR system-wide installation
curl -fsSL https://raw.githubusercontent.com/swadhinbiswas/ls-f/main/install-standalone.sh | bash -s system

# Verify installation
lsf --version
lsf -x
```

### Method 2: Git Clone

For development or customization:

```bash
# Clone the repository
git clone https://github.com/swadhinbiswas/ls-f.git
cd ls-f

# User-local installation (no sudo needed, default for non-root)
./install.sh --user

# OR system-wide installation (requires sudo)
sudo ./install.sh --system

# Verify installation
lsf --version
lsf -x
```
sudo ./install.sh --system
### Method 2: Direct Download

# Verify installation
lsf --version
 Download both scripts directly from GitHub:
```

### Method 2: Direct Download
sudo curl -fL https://raw.githubusercontent.com/swadhinbiswas/ls-f/main/lsf \
Download both scripts directly from GitHub:

```bash
sudo curl -fL https://raw.githubusercontent.com/swadhinbiswas/ls-f/main/column.py \
sudo curl -fL https://raw.githubusercontent.com/swadhin-biswas/ls-f/main/lsf \
  -o /usr/local/bin/lsf

# Download column.py helper
sudo curl -fL https://raw.githubusercontent.com/swadhin-biswas/ls-f/main/column.py \
  -o /usr/local/bin/column.py

# Make both executable
sudo chmod +x /usr/local/bin/lsf /usr/local/bin/column.py
git clone https://github.com/swadhinbiswas/ls-f.git && cd ls-f && ./install.sh --user
# Create config directory
mkdir -p ~/.config/lsf
git clone https://github.com/swadhinbiswas/ls-f.git && cd ls-f && sudo ./install.sh --system
# Verify installation
lsf --version
lsf -x
```

### Method 3: One-Liner Installation

Clone and install in a single command:
git clone https://github.com/swadhinbiswas/ls-f.git
```bash
# User installation (no sudo needed)
git clone https://github.com/swadhin-biswas/ls-f.git && cd ls-f && ./install.sh --user
# System-wide installation
git clone https://github.com/swadhin-biswas/ls-f.git && cd ls-f && sudo ./install.sh --system
```

> **Note:** The installer requires both `lsf` and `column.py` from the repository, so piping directly to bash is not recommended.

### Method 4: Manual Installation

For complete control:

```bash
# Clone and enter directory
git clone https://github.com/swadhinbiswas/ls-f.git
cd ls-f

# Create install directory (user or system)
mkdir -p ~/.local/bin              # User install
# OR
sudo mkdir -p /usr/local/bin       # System install

# Copy files
cp lsf column.py ~/.local/bin/     # User install
# OR
sudo cp lsf column.py /usr/local/bin/  # System install

# Make executable
chmod +x ~/.local/bin/{lsf,column.py}
# OR
sudo chmod +x /usr/local/bin/{lsf,column.py}

# Ensure user bin is in PATH (for user install)
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

---

## 🔧 Installation Options

The `install.sh` script supports several options for customization:

```bash
./install.sh [--user | --system] [--replace-ls] [--force] [--no-font-check] [-h|--help]
```

### Options

| Option | Description |
|--------|-------------|
| `--user` | Install to `~/.local/bin` (default for non-root users) |
| `--system` | Install to `/usr/local/bin` (requires sudo) |
| `--replace-ls` | Replace system `ls` with wrapper (system-only; creates backup) |
| `--force` | Overwrite existing files without prompting |
| `--no-font-check` | Skip Nerd Font detection |
| `-h, --help` | Show help message |

### Examples

```bash
# User-local installation
./install.sh --user

# System-wide with Nerd Font check skipped
sudo ./install.sh --system --no-font-check

# Replace system ls (backup created)
sudo ./install.sh --system --replace-ls

# Force install without prompts
./install.sh --force

# Help
./install.sh --help
```

---

## 🔗 Post-Installation Setup

### Verify Installation

```bash
which lsf

# Test basic functionality
lsf --version
lsf --help

# Try listing files with icons
lsf
lsf -x
lsf -l
```

### Set Up Alias (Optional)

To use `lsf` as your default `ls`:

#### Bash

Add to `~/.bashrc`:
```bash
alias ls='lsf'
```

Then reload:
```bash
source ~/.bashrc
```

#### Zsh

Add to `~/.zshrc`:
```bash
alias ls='lsf'
```

Then reload:
```bash
source ~/.zshrc
```

#### Fish

Add to `~/.config/fish/config.fish`:
```fish
alias ls='lsf'
```

Then reload:
```bash
source ~/.config/fish/config.fish
```

#### Tcsh/Csh

Add to `~/.cshrc`:
```csh
alias ls 'lsf'
```

Then reload:
```bash
source ~/.cshrc
```

### Automatic Alias Setup

Run the included setup script (if available):

```bash
# From the ls-f repository directory
source config/lsf_alias_setup.sh
```

> **Note:** This requires you to be in the cloned `ls-f` directory.

---

## 📍 Troubleshooting Installation

### "Permission Denied" Error

**Problem:** Can't write to installation directory.

**Solution:** Use sudo for system install:
sudo ./install.sh --system
```

Or use user install:
```bash
./install.sh --user
```

### "Command Not Found" After Installation

**Problem:** `lsf` is installed but not found in PATH.

**Solution (User Install):**

Add `~/.local/bin` to your PATH:
```bash
# For Bash - add to ~/.bashrc
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# For Zsh - add to ~/.zshrc
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

**Solution (System Install):**
```bash
# Restart your terminal or reload shell config
exec $SHELL
```

### Python or column.py Not Found

**Problem:** Error about missing `column.py` or Python.

**Solution 1:** Ensure both files were installed:
```bash
which lsf column.py
```

**Solution 2:** Reinstall from git to get both files:
```bash
git clone https://github.com/swadhin-biswas/ls-f.git
cd ls-f
./install.sh --user --force
```

**Solution 3:** Install Python 3 if missing:
```bash
# Ubuntu/Debian
sudo apt install python3

# Fedora/RHEL

# macOS
brew install python3

**Problem:** Icons show as boxes or question marks.

**Solution:** Install Nerd Fonts:
1. Download from [nerdfonts.com](https://www.nerdfonts.com/)
2. Install to `~/.local/share/fonts/`
3. Set your terminal font to a Nerd Font
4. Use `fc-cache -fv` to refresh font cache

Or skip icon checks during install:
```bash
./install.sh --no-font-check
```

---

## 🔄 Updating

To update `lsf` to the latest version:

```bash
# If cloned from git
cd ls-f
git pull origin main
./install.sh --user --force  # or sudo ./install.sh --system --force

# Verify the update
lsf --version
```

---

## 🗑️ Uninstallation

### Remove Binary

```bash
# User install
rm ~/.local/bin/lsf ~/.local/bin/column.py

# System install
sudo rm /usr/local/bin/lsf /usr/local/bin/column.py
```

### Remove Configuration

```bash
rm -rf ~/.config/lsf/
```

### Remove Alias

Remove or comment out the alias from your shell config:
```bash
# Edit ~/.bashrc, ~/.zshrc, etc.
# Remove or comment out: alias ls='lsf'
```

### Restore Original `ls`

If you used `--replace-ls`:
```bash
# Remove wrapper
sudo rm /usr/local/bin/ls

# Restore from backup
sudo mv /usr/local/bin/ls.bak.* /usr/local/bin/ls
```

---

## 🆘 Getting Help

- **Issues:** https://github.com/swadhinbiswas/ls-f/issues
- **Documentation:** See [README.md](../README.md)
- **Configuration:** See [CONFIGURATION.md](CONFIGURATION.md)

Happy listing! 🚀
