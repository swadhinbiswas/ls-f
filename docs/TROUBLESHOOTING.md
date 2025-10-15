# Troubleshooting Guide for ls-f

Solutions to common issues and problems with **ls-f**.

---

## 🔍 General Diagnostics

Before troubleshooting, run these diagnostic commands:

```bash
# Check if lsf is installed
which lsf

# Check version
lsf --version

# Test basic functionality
lsf

# Check for errors
lsf -a 2>&1 | head -20
```

---

## 🎨 Icons Not Displaying

### Symptom
Icons appear as boxes `□`, question marks `?`, or don't show at all.

### Causes & Solutions

#### 1. Missing or Incorrect Nerd Font

**Check installed fonts:**
```bash
fc-list | grep -i "Nerd Font"
```

**Expected output:**
```
/usr/share/fonts/...: Nerd Font Mono:...
```

**Solution:**
- Download from [nerdfonts.com](https://www.nerdfonts.com/)
- Extract to `~/.local/share/fonts/`
- Refresh cache: `fc-cache -fv`
- Restart terminal and set font to Nerd Font

#### 2. Terminal Font Not Set to Nerd Font

**Solution:**
1. Open terminal settings
2. Find "Font" or "Text" settings
3. Select a Nerd Font (e.g., "FiraCode Nerd Font", "JetBrains Mono Nerd Font")
4. Apply and restart terminal

#### 3. Icons Disabled (Bash < 4)

**Check Bash version:**
```bash
echo $BASH_VERSION
```

**Expected:** Version 4 or higher.

**If lower:**
- Install Bash 4+
- Use `lsf --no-icons` for now
- Or use `--user` installer flag

**Solution:**
```bash
# Install newer Bash (Ubuntu/Debian)
sudo apt-get install bash

# Or use Zsh instead
sudo apt-get install zsh
```

#### 4. Icons Explicitly Disabled

**Check if `--no-icons` is active:**
```bash
# Test without flags
lsf

# Test with icons forced
lsf --help  # check for --no-icons option
```

**Solution:**
- Remove `--no-icons` from alias if set
- Verify `.bashrc` or `.zshrc` doesn't have disabling flag

---

## 🔧 "Command Not Found" or "lsf: command not found"

### Symptom
`lsf` doesn't execute, even though it was installed.

### Causes & Solutions

#### 1. Not in PATH (User Install)

**Check:**
```bash
echo $PATH
which lsf
```

**Solution:**
Add to `~/.bashrc` or `~/.zshrc`:
```bash
export PATH="$HOME/.local/bin:$PATH"
```

Then reload:
```bash
source ~/.bashrc
exec $SHELL
```

#### 2. Installation Directory Not in PATH (System Install)

**Check:**
```bash
ls -la /usr/local/bin/lsf
```

**If not found:**
```bash
# Reinstall
sudo ./install.sh --system --force
```

#### 3. Shell Cache Not Updated

**Solution:**
```bash
# Clear shell hash table
hash -r

# Reload shell
exec $SHELL
```

---

## 📝 Alias Not Working

### Symptom
`alias ls='lsf'` is set but `ls` doesn't use `lsf`.

### Causes & Solutions

#### 1. Shell Config Not Sourced

**Check if alias is set:**
```bash
alias | grep ls
```

**If empty:**
```bash
# Reload shell config
source ~/.bashrc
# OR
source ~/.zshrc
```

#### 2. Typo in Alias

**Check:**
```bash
alias | grep lsf
```

**Common typos:**
- `alias ls=lsf` (missing quotes)
- `alias ls 'lsf'` (wrong quote type for Bash)
- Extra spaces: `alias ls = 'lsf'`

**Fix:**
```bash
# Correct syntax
alias ls='lsf'
```

#### 3. Alias Overridden Later

**Check if something is overriding the alias:**
```bash
# Search for later alias definitions
grep -n "alias ls" ~/.bashrc ~/.zshrc 2>/dev/null
```

**Solution:** Remove duplicate or conflicting alias.

#### 4. Interactive vs Non-Interactive Shell

**For non-interactive shells (scripts):**

Add to `~/.bashrc` (not `~/.bash_profile`):
```bash
alias ls='lsf'
```

Or use `~/.bash_aliases`:
```bash
# ~/.bash_aliases
alias ls='lsf'

# In ~/.bashrc:
if [ -f ~/.bash_aliases ]; then
    source ~/.bash_aliases
fi
```

---

## ⚠️ Permission Errors

### Symptom
"Permission denied" or "Cannot write to directory" during installation.

### Causes & Solutions

#### 1. Insufficient Permissions (System Install)

**Error:**
```
Error: You don't have write permissions for /usr/local/bin
```

**Solution:**
```bash
# Use sudo for system install
sudo ./install.sh --system
```

#### 2. File Already Exists (Permission Issue)

**Error:**
```
cp: cannot create regular file: Permission denied
```

**Solution:**
```bash
# Force overwrite with sudo
sudo ./install.sh --system --force

# OR remove existing file
sudo rm /usr/local/bin/lsf
./install.sh --system
```

#### 3. Cannot Create Config Directory

**Error:**
```
mkdir: cannot create directory: Permission denied
```

**Solution:**
```bash
# Create manually
mkdir -p ~/.config/lsf
chmod 755 ~/.config/lsf

# Then reinstall
./install.sh --user
```

---

## 🐍 Python or column.py Issues

### Symptom
Errors about `column.py` or Python not found.

### Causes & Solutions

#### 1. Python Not Installed

**Check:**
```bash
which python3
python3 --version
```

**Solution (Ubuntu/Debian):**
```bash
sudo apt-get install python3
```

**Solution (Fedora/RHEL):**
```bash
sudo dnf install python3
```

**Solution (macOS):**
```bash
brew install python3
```

#### 2. column.py Not Installed

**Check:**
```bash
ls -la ~/.local/bin/column.py
# OR
ls -la /usr/local/bin/column.py
```

**If missing:**
```bash
# Reinstall with column.py
cd ~/path/to/ls-f
./install.sh --force
```

#### 3. Python 2 vs Python 3

**Check default Python:**
```bash
python --version  # Might be Python 2
python3 --version  # Should be Python 3.x
```

**Solution:**
Ensure `column.py` has correct shebang:
```bash
head -1 ~/.local/bin/column.py
# Should output: #!/usr/bin/env python3
```

---

## 🎯 File Listing Issues

### Symptom
`lsf` shows no files, wrong files, or unusual output.

### Causes & Solutions

#### 1. Wrong Directory

**Check:**
```bash
pwd
lsf
```

**Solution:**
```bash
# List current directory explicitly
lsf .

# Or specify path
lsf /path/to/directory
```

#### 2. Permission Denied on Files

**Error:**
```
lsf: command not found
```

**Solution:**
```bash
# Use absolute path
/usr/local/bin/lsf

# OR check PATH
echo $PATH | tr ':' '\n'
```

#### 3. Hidden Files Not Showing

**Note:** By default, hidden files are not shown (standard `ls` behavior).

**Solution:**
```bash
# Show hidden files
lsf -a

# OR use with alias
ls -a
```

---

## 💻 Cross-Platform Issues

### macOS Specific

#### 1. Bash Version Too Old

**Check:**
```bash
bash --version
```

**Solution:**
```bash
# Install newer Bash
brew install bash

# Set as default shell
chsh -s /usr/local/bin/bash
```

#### 2. Nerd Fonts on macOS

**Solution:**
```bash
# Install via Homebrew
brew tap homebrew/cask-fonts
brew install font-fira-code-nerd-font
```

### WSL (Windows Subsystem for Linux)

#### 1. Fonts in WSL

**Solution:**
- Install Nerd Fonts on Windows (host)
- Configure Windows Terminal to use the font
- WSL will inherit terminal settings

---

## 🐛 Debug Mode

Enable detailed output to diagnose issues:

```bash
# Run with bash debug flag
bash -x /usr/local/bin/lsf -a 2>&1 | head -50

# Check for errors
lsf -a 2>&1

# Verify config is loaded
echo $CONFIG_DIR
cat ~/.config/lsf/lsf.conf
```

---

## 🆘 Getting Help

If you still have issues:

1. **Check existing issues:** https://github.com/swadhin-biswas/ls-f/issues
2. **Create new issue with:**
   - Your OS and version (`uname -a`)
   - Shell and version (`echo $SHELL $BASH_VERSION`)
   - Python version (`python3 --version`)
   - Error message (full output)
   - Steps to reproduce
3. **Include diagnostic output:**
   ```bash
   which lsf
   lsf --version
   echo $PATH
   alias | grep ls
   ```

---

## 📚 Related Documentation

- [Installation Guide](INSTALLATION.md)
- [Configuration Guide](CONFIGURATION.md)
- [README.md](../README.md)

---

**Happy listing! If you encounter issues, we're here to help.** 🚀
