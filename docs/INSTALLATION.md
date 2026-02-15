# Installation Guide for lsf

**lsf v5.0.0** is a native Rust binary. No runtime dependencies -- no Bash 4+, no Python, no GNU coreutils required. Just a single binary.

---

## Prerequisites

### Required
- A **Nerd Font** installed and set in your terminal for icon display

### Check for Nerd Fonts

```bash
# Linux
fc-list | grep -i "Nerd Font"

# macOS
system_profiler SPFontsDataType | grep -i "Nerd"
```

If you don't have Nerd Fonts, download from [nerdfonts.com](https://www.nerdfonts.com/).

Popular choices: **JetBrainsMono Nerd Font**, **FiraCode Nerd Font**, **Hack Nerd Font**.

---

## Installation Methods

### Method 1: Build from Source (Recommended)

Requires [Rust toolchain](https://rustup.rs/) (1.70+):

```bash
git clone https://github.com/swadhinbiswas/ls-f.git
cd ls-f
cargo build --release

# The binary is at ./target/release/lsf
# Copy it to your PATH:
cp target/release/lsf ~/.local/bin/
# OR system-wide:
sudo cp target/release/lsf /usr/local/bin/
```

### Method 2: Cargo Install (from crates.io)

```bash
cargo install lsf

# Binary is installed to ~/.cargo/bin/lsf
# Make sure ~/.cargo/bin is in your PATH
```

### Method 3: Download Prebuilt Binary

```bash
# Download the latest release binary for your platform from GitHub Releases
# https://github.com/swadhinbiswas/ls-f/releases

# Example for Linux x86-64:
curl -fsSL https://github.com/swadhinbiswas/ls-f/releases/latest/download/lsf-linux-x86_64 -o lsf
chmod +x lsf
sudo mv lsf /usr/local/bin/
```

### Verify Installation

```bash
lsf --version    # Should print: lsf 5.0.0
lsf --help       # Show all available flags
lsf              # List current directory with icons
lsf -la          # Long format with all files
```

---

## Shell Alias Setup

You can use `lsf` as a drop-in replacement for `ls` by setting up aliases. Below are instructions for **every major shell**, showing how to alias both `lsf` (as a shortcut) and `ls` (as a full replacement).

### Bash

Edit `~/.bashrc` (or `~/.bash_profile` on macOS):

```bash
# Replace ls with lsf
alias ls='lsf'

# Optional: common ls shortcuts using lsf
alias ll='lsf -lah'
alias la='lsf -A'
alias l='lsf -1'
alias lt='lsf --tree --depth 2'
```

Apply changes:

```bash
source ~/.bashrc
```

### Zsh

Edit `~/.zshrc`:

```zsh
# Replace ls with lsf
alias ls='lsf'

# Optional: common shortcuts
alias ll='lsf -lah'
alias la='lsf -A'
alias l='lsf -1'
alias lt='lsf --tree --depth 2'
```

Apply changes:

```bash
source ~/.zshrc
```

**Oh My Zsh users:** Oh My Zsh defines its own `ls` aliases in the `lib/directories.zsh` plugin. To override them, put your aliases **after** the `source $ZSH/oh-my-zsh.sh` line in `~/.zshrc`, or add them to a custom plugin:

```bash
# Create custom plugin
mkdir -p ~/.oh-my-zsh/custom/plugins/lsf
cat > ~/.oh-my-zsh/custom/plugins/lsf/lsf.plugin.zsh << 'EOF'
alias ls='lsf'
alias ll='lsf -lah'
alias la='lsf -A'
alias l='lsf -1'
alias lt='lsf --tree --depth 2'
EOF
```

Then add `lsf` to your plugins list in `~/.zshrc`:

```zsh
plugins=(git lsf)
```

### Fish

Edit `~/.config/fish/config.fish`:

```fish
# Replace ls with lsf
alias ls 'lsf'

# Optional: common shortcuts
alias ll 'lsf -lah'
alias la 'lsf -A'
alias l 'lsf -1'
alias lt 'lsf --tree --depth 2'
```

Or use Fish's `abbr` for expandable abbreviations (shows the expanded command):

```fish
abbr --add ls lsf
abbr --add ll 'lsf -lah'
abbr --add la 'lsf -A'
abbr --add lt 'lsf --tree --depth 2'
```

Apply changes:

```bash
source ~/.config/fish/config.fish
```

### Tcsh / Csh

Edit `~/.cshrc` (or `~/.tcshrc` for tcsh):

```csh
# Replace ls with lsf
alias ls 'lsf'

# Optional shortcuts
alias ll 'lsf -lah'
alias la 'lsf -A'
alias l 'lsf -1'
alias lt 'lsf --tree --depth 2'
```

Apply changes:

```bash
source ~/.cshrc
```

### Ksh (KornShell)

Edit `~/.kshrc` (make sure `ENV=~/.kshrc` is set in `~/.profile`):

```ksh
# Replace ls with lsf
alias ls='lsf'

# Optional shortcuts
alias ll='lsf -lah'
alias la='lsf -A'
alias l='lsf -1'
alias lt='lsf --tree --depth 2'
```

Apply changes:

```bash
source ~/.kshrc
```

### Dash / POSIX sh

Edit `~/.profile` (or `~/.shrc` if your system supports `ENV`):

```sh
# Replace ls with lsf
alias ls='lsf'
alias ll='lsf -lah'
alias la='lsf -A'
```

Apply changes:

```bash
. ~/.profile
```

### PowerShell (pwsh)

Edit your PowerShell profile (`$PROFILE`):

```powershell
# Find your profile path
# $PROFILE

# Replace ls with lsf
Set-Alias -Name ls -Value lsf -Option AllScope
Set-Alias -Name lsf -Value lsf -Option AllScope

# Optional: functions for shortcuts with arguments
function ll { lsf -lah @args }
function la { lsf -A @args }
function lt { lsf --tree --depth 2 @args }
```

Apply changes:

```powershell
. $PROFILE
```

### Nushell

Edit `~/.config/nushell/config.nu`:

```nu
# Replace ls with lsf (as external command)
alias ls = ^lsf
alias ll = ^lsf -lah
alias la = ^lsf -A
alias lt = ^lsf --tree --depth 2
```

### Elvish

Edit `~/.config/elvish/rc.elv`:

```elvish
# Replace ls with lsf
fn ls {|@args| e:lsf $@args }
fn ll {|@args| e:lsf -lah $@args }
fn la {|@args| e:lsf -A $@args }
fn lt {|@args| e:lsf --tree --depth 2 $@args }
```

### Xonsh

Edit `~/.xonshrc`:

```python
# Replace ls with lsf
aliases['ls'] = 'lsf'
aliases['ll'] = 'lsf -lah'
aliases['la'] = 'lsf -A'
aliases['lt'] = 'lsf --tree --depth 2'
```

---

## Quick One-Liner Setup

Copy-paste the right command for your shell:

```bash
# Bash
echo 'alias ls="lsf"' >> ~/.bashrc && source ~/.bashrc

# Zsh
echo 'alias ls="lsf"' >> ~/.zshrc && source ~/.zshrc

# Fish
echo 'alias ls lsf' >> ~/.config/fish/config.fish && source ~/.config/fish/config.fish

# Tcsh
echo 'alias ls lsf' >> ~/.cshrc && source ~/.cshrc

# Ksh
echo 'alias ls="lsf"' >> ~/.kshrc && source ~/.kshrc
```

---

## Accessing Real ls

After aliasing, you may occasionally need the original `ls`. Use any of these methods:

```bash
# Method 1: Backslash (bypasses aliases in bash/zsh/ksh)
\ls

# Method 2: Full path
/usr/bin/ls
# or
/sbin/ls

# Method 3: command builtin (bypasses aliases and functions)
command ls

# Method 4: env (runs from PATH, bypasses shell builtins)
env ls
```

---

## PATH Setup

If `lsf` is not found after installation, ensure its location is in your PATH:

```bash
# For ~/.local/bin (user install)
# Bash/Zsh/Ksh:
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc  # or ~/.zshrc
source ~/.bashrc

# Fish:
fish_add_path ~/.local/bin

# For ~/.cargo/bin (cargo install)
# Bash/Zsh/Ksh:
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc  # or ~/.zshrc
source ~/.bashrc

# Fish:
fish_add_path ~/.cargo/bin
```

---

## Uninstallation

```bash
# Remove binary
rm ~/.local/bin/lsf          # user install
# OR
sudo rm /usr/local/bin/lsf   # system install
# OR
cargo uninstall lsf           # cargo install

# Remove aliases from your shell config
# Edit ~/.bashrc, ~/.zshrc, etc. and remove the alias lines
```

---

## Troubleshooting

### Icons show as boxes or question marks

Install a [Nerd Font](https://www.nerdfonts.com/) and set it as your terminal font.

### "command not found" after install

Check that the binary location is in your PATH:

```bash
which lsf
echo $PATH
```

### Alias not working

1. Make sure you edited the correct config file for your shell
2. Make sure you reloaded the config (`source ~/.bashrc`, etc.)
3. Check for conflicting aliases: `alias | grep ls`
4. For Oh My Zsh, place aliases **after** the `source $ZSH/oh-my-zsh.sh` line

### Colors not showing

```bash
# Force color output
lsf --color=always

# Check terminal color support
echo -e "\e[31mRed\e[0m \e[32mGreen\e[0m \e[34mBlue\e[0m"
```

---

## More Information

- [BENCHMARKS.md](../BENCHMARKS.md) -- Performance comparison with GNU ls
- [README.md](../README.md) -- Feature overview
- [TROUBLESHOOTING.md](TROUBLESHOOTING.md) -- Detailed diagnostics
