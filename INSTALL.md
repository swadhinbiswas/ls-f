# Installation Guide for ls-f

## Prerequisites

Before installing `ls-f`, ensure your system meets these requirements:

1. **Nerd Fonts**: You need to have [Nerd Fonts](https://www.nerdfonts.com/) installed to see the icons correctly
2. **Shell Compatibility**: Bash, Zsh, Fish, or other POSIX-compatible shells
3. **Root Access**: For system-wide installation, you'll need `sudo` access

### Checking Prerequisites

To verify Nerd Fonts are installed:
```bash
fc-list | grep -i "Nerd Font"
```

If this command returns results, you have Nerd Fonts installed. If not, visit [Nerd Fonts website](https://www.nerdfonts.com/) to install them.

## Installation Methods

### Method 1: Git Clone (Recommended)

```bash
# Clone the repository
git clone https://github.com/your-username/ls-f.git
cd ls-f

# Install using the script
sudo ./install.sh
```

### Method 2: Direct Download

```bash
# Download the lsf script directly
sudo curl -L https://github.com/your-username/ls-f/releases/latest/download/lsf -o /usr/local/bin/lsf
sudo chmod +x /usr/local/bin/lsf
```

### Method 3: Using the Raw Install Script

```bash
curl -fsSL https://raw.githubusercontent.com/your-username/ls-f/main/install.sh | sudo bash
```

## Post-Installation Setup

After installation, you can use `lsf` immediately:

```bash
lsf
lsf -la
lsf --help
```

### Setting up Aliases

To replace the default `ls` command with `lsf`, set up an alias in your shell configuration.

#### For Bash users

Add to `~/.bashrc`:
```bash
alias ls='lsf'
```

Then reload:
```bash
source ~/.bashrc
```

#### For Zsh users

Add to `~/.zshrc`:
```bash
alias ls='lsf'
```

Then reload:
```bash
source ~/.zshrc
```

#### For Fish users

Add to `~/.config/fish/config.fish`:
```fish
alias ls='lsf'
```

Then reload:
```bash
source ~/.config/fish/config.fish
```

#### For Tcsh/Csh users

Add to `~/.cshrc` or `~/.tcshrc`:
```csh
alias ls 'lsf'
```

## Verifying Installation

After installation and alias setup, verify everything works:

```bash
# Test the command exists
which lsf

# Test basic functionality
lsf
lsf -la

# If alias is set, test that ls now uses lsf
ls
```

## Troubleshooting

### Icons Not Displaying

If icons appear as question marks or boxes, ensure:
1. Nerd Fonts are properly installed
2. Your terminal supports Nerd Fonts
3. Your terminal font is set to a Nerd Font

### Alias Not Working

If the alias doesn't work after adding it:
1. Make sure to reload your shell configuration (`source ~/.bashrc` or `source ~/.zshrc`)
2. Check for syntax errors in the configuration file
3. Verify the alias was added correctly by checking the file contents

### Permission Errors

If you encounter permission errors during installation, ensure you're using `sudo` for system-wide installation.

## Uninstallation

To uninstall `lsf`, simply remove the binary:

```bash
sudo rm /usr/local/bin/lsf
```

To also remove the configuration:
```bash
rm -rf ~/.config/lsf/
```

Don't forget to remove the alias from your shell configuration file if you added one.