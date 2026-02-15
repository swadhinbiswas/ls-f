# Troubleshooting Guide for lsf

Solutions to common issues with **lsf v5.0.0** (Rust rewrite).

---

## 🔍 General Diagnostics

Before troubleshooting, run these diagnostic commands:

```bash
# Check if lsf is installed
which lsf

# Check version (should be 5.0.0+)
lsf --version

# Test basic functionality
lsf

# Check for errors
lsf . 2>&1 | head -n 20
```

---

## 🎨 Icons Not Displaying

### Symptom
Icons appear as boxes `□`, question marks `?`, or don't show at all.

### Causes & Solutions

#### 1. Missing or Incorrect Nerd Font

**Check installed fonts:**
```bash
# Linux
fc-list | grep -i "Nerd Font"

# macOS
system_profiler SPFontsDataType | grep -i "Nerd"
```

**Solution:**
- Download from [nerdfonts.com](https://www.nerdfonts.com/)
- Extract to `~/.local/share/fonts/` (Linux) or install via Font Book (macOS)
- Refresh cache (Linux): `fc-cache -fv`
- **Crucial:** Restart your terminal and set the font to a Nerd Font (e.g., "JetBrainsMono Nerd Font").

#### 2. Icons Explicitly Disabled

**Check if `--no-icons` is active:**
- Check your shell aliases: `alias | grep ls`
- Try running `lsf` without any flags or aliases: `command lsf`

---

## 🔧 "lsf: command not found"

### Symptom
`lsf` doesn't execute, even though it was installed.

### Causes & Solutions

#### 1. Not in PATH (Local Install)

If you installed to `~/.local/bin/` or `~/.cargo/bin/`, ensure it's in your `$PATH`.

**Check:**
```bash
echo $PATH
```

**Solution:**
Add to `~/.bashrc` or `~/.zshrc`:
```bash
export PATH="$HOME/.local/bin:$PATH"
# or for cargo
export PATH="$HOME/.cargo/bin:$PATH"
```

#### 2. Shell Cache Not Updated

**Solution:**
```bash
# Clear shell hash table
hash -r   # Bash/Zsh
rehash    # Zsh/Fish
```

---

## 📝 Alias Not Working

### Symptom
`alias ls='lsf'` is set but `ls` doesn't use `lsf`.

### Causes & Solutions

#### 1. Shell Config Not Sourced
Reload your config: `source ~/.bashrc` or `source ~/.zshrc`.

#### 2. Overridden by Oh My Zsh
Oh My Zsh often sets its own `ls` aliases. Ensure your `alias ls='lsf'` is at the **very bottom** of `~/.zshrc`, or after the line `source $ZSH/oh-my-zsh.sh`.

#### 3. Typo in Alias
Ensure the syntax is correct for your shell:
- Bash/Zsh: `alias ls='lsf'`
- Fish: `alias ls lsf` or `abbr --add ls lsf`

---

## ⚠️ Permission Errors

### Symptom
"Permission denied" during installation.

### Causes & Solutions

#### 1. System-wide Installation
If installing to `/usr/local/bin`, you must use `sudo`:
```bash
sudo cp target/release/lsf /usr/local/bin/
```

#### 2. Binary Not Executable
Ensure the binary has execution permissions:
```bash
chmod +x /path/to/lsf
```

---

## 🌈 Colors Look Wrong

### Symptom
Colors are missing, dull, or unreadable.

### Causes & Solutions

#### 1. Terminal Color Support
Check if your terminal supports true color:
```bash
curl -s https://raw.githubusercontent.com/gnachman/iTerm2/master/tests/24-bit-color.sh | bash
```

#### 2. Color Mode Forced
Try forcing color output to see if it's an auto-detection issue:
```bash
lsf --color=always
```

---

## 💻 Cross-Platform Issues

### macOS Specific

#### Homebrew PATH
If installed via Homebrew or Cargo, ensure `/opt/homebrew/bin` or `~/.cargo/bin` is in your PATH.

#### Nerd Fonts on macOS
Recommended way to install:
```bash
brew tap homebrew/cask-fonts
brew install --cask font-jetbrains-mono-nerd-font
```

### WSL (Windows Subsystem for Linux)

#### Fonts in WSL
You must install the Nerd Font on the **Windows host system**, then configure the Windows Terminal (or whatever terminal you use) to use that font.

---

## 🆘 Getting Help

If you still have issues:

1. **Check existing issues:** [https://github.com/swadhinbiswas/ls-f/issues](https://github.com/swadhinbiswas/ls-f/issues)
2. **Open a new issue** with:
   - OS & Version (`uname -a`)
   - Terminal emulator name
   - `lsf --version` output
   - A screenshot of the problem (especially for icon/color issues)

---

## 📚 Related Documentation

- [Installation Guide](INSTALLATION.md)
- [Configuration Guide](CONFIGURATION.md)
- [README.md](../README.md)
