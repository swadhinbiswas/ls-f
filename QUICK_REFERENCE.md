# Quick Reference Guide — ls-f

A one-page cheat sheet for **ls-f** users and developers.

---

## 🚀 Installation (One Minute)

```bash
# User install (no sudo)
./install.sh --user

# System install (requires sudo)
sudo ./install.sh --system

# Or one-liner
curl -fsSL https://raw.githubusercontent.com/swadhin-biswas/ls-f/main/install.sh | bash
```

---

## 📖 Quick Commands

```bash
lsf              # List files with icons (grid view)
lsf -a           # List all files (including hidden)
lsf -A           # Hidden files without . and ..
lsf -b           # Show names with C-style escapes
lsf -x           # List entries by lines (horizontal)
lsf -T 4         # Align columns with tab width of 4
lsf -Z           # Display SELinux/AppArmor security context
lsf --zero       # Output entries delimited by NUL (for scripting)
lsf -l           # Long format
lsf -la          # Long + all files
lsf -R           # Recursive listing
lsf -t           # Sort by time
lsf -S           # Sort by size
lsf -X           # Sort by extension
lsf --help       # Show help
lsf --version    # Show version
lsf --no-icons   # Disable icons
```

---

## 🎨 Customize Icons

Edit `~/.config/lsf/lsf.conf`:

```bash
# Python icon
ICONS[".py"]=""
COLORS[".py"]="92"  # Bright green

# Directory icon
ICONS["dir"]=""
COLORS["dir"]="94"  # Bright blue

# JSON icon
ICONS[".json"]=""
COLORS[".json"]="36"  # Cyan
```

**Color Codes:** 30-37 (normal), 90-97 (bright), 38;5;N (256-color), 38;2;R;G;B (RGB)

---

## ⚙️ Installation Options

| Option | Purpose |
|--------|---------|
| `--user` | Install to `~/.local/bin` |
| `--system` | Install to `/usr/local/bin` (requires sudo) |
| `--replace-ls` | Replace system `ls` (creates backup) |
| `--force` | Overwrite without prompting |
| `--no-font-check` | Skip Nerd Font detection |
| `--help` | Show all options |

---

## 🔗 Set Up Alias

### Bash (~/.bashrc)
```bash
alias ls='lsf'
source ~/.bashrc
```

### Zsh (~/.zshrc)
```bash
alias ls='lsf'
source ~/.zshrc
```

### Fish (~/.config/fish/config.fish)
```fish
alias ls='lsf'
```

---

## 🔧 Troubleshooting

| Problem | Solution |
|---------|----------|
| Icons show as boxes | Install Nerd Fonts from nerdfonts.com |
| `lsf: command not found` | Add `~/.local/bin` to PATH |
| Alias not working | Reload shell: `source ~/.bashrc` |
| Permission denied | Use `sudo ./install.sh --system` |
| Python not found | Install Python 3: `apt install python3` |

---

## 📚 Documentation

- **Website:** Open `index.html`
- **Installation:** See `docs/INSTALLATION.md`
- **Configuration:** See `docs/CONFIGURATION.md`
- **Troubleshooting:** See `docs/TROUBLESHOOTING.md`
- **Overview:** See `README.md`

---

## 🎯 Common Use Cases

### Replace system ls
```bash
sudo ./install.sh --system --replace-ls
alias ls='lsf'  # Add to ~/.bashrc or ~/.zshrc
```

### Install for current user only
```bash
./install.sh --user
export PATH="$HOME/.local/bin:$PATH"  # Add to ~/.bashrc
```

### Customize all Python files purple
```bash
# Edit ~/.config/lsf/lsf.conf
ICONS[".py"]=""
COLORS[".py"]="35"  # Magenta/Purple
```

### Use without icons
```bash
lsf --no-icons
# OR disable Nerd Fonts permanently:
./install.sh --no-font-check
```

---

## 🛠️ Uninstall

```bash
# Remove files
rm ~/.local/bin/lsf ~/.local/bin/column.py        # User
sudo rm /usr/local/bin/lsf /usr/local/bin/column.py  # System

# Remove config
rm -rf ~/.config/lsf/

# Remove alias
# Edit ~/.bashrc or ~/.zshrc and remove: alias ls='lsf'
```

---

## 📊 File Type Categories Supported

- **Programming:** `.py`, `.js`, `.ts`, `.rs`, `.go`, `.c`, `.java`, etc.
- **Config:** `.json`, `.yaml`, `.toml`, `.xml`, `.env`, `.conf`
- **Docs:** `.md`, `.txt`, `.pdf`, `.doc`
- **Media:** `.png`, `.jpg`, `.mp4`, `.mp3`, `.zip`
- **Special:** `package.json`, `Dockerfile`, `LICENSE`, `.gitignore`
- **And 100+ more!**

---

## 🚨 Diagnostics

```bash
# Check installation
which lsf
lsf --version

# Check fonts
fc-list | grep -i "Nerd Font"

# Check bash version
echo $BASH_VERSION

# Check Python
python3 --version

# Check PATH
echo $PATH | tr ':' '\n'

# Check alias
alias | grep ls
```

---

## 🌐 Platform Support

| OS | Support | Install |
|----|---------|---------|
| Linux (Ubuntu, Fedora, Arch, etc.) | ✅ Full | `./install.sh` |
| macOS | ✅ Full | `./install.sh` (needs Bash 4+) |
| WSL (Windows) | ✅ Full | `./install.sh` |
| BSD/Unix | ✅ Full | `./install.sh` |

---

## 💡 Tips

1. **Keep it simple:** Start with default icons, customize later
2. **Use colors consistently:** Pick a color scheme and stick with it
3. **Test changes:** Run `lsf` after editing config
4. **Back up config:** Save working configurations
5. **Check permissions:** Use `ls -la ~/.local/bin/` to verify

---

## 🆘 Getting Help

- **GitHub Issues:** https://github.com/swadhin-biswas/ls-f/issues
- **Documentation:** See `docs/` folder
- **FAQ:** Open `index.html` in browser

---

## 📄 License

MIT License — Use freely, modify, and redistribute.

---

**Last updated: October 16, 2025**
For latest docs, visit: https://github.com/swadhin-biswas/ls-f
