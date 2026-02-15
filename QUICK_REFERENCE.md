# Quick Reference Guide — lsf v5.0.0

A one-page cheat sheet for **lsf** (Rust) users and developers.

---

## 🚀 Installation

```bash
# Build from source (Rust)
make install

# Or via cargo
cargo install --path .
```

---

## 📖 Quick Commands

```bash
lsf              # List files with icons (grid view)
lsf -la          # Long format (all files)
lsf --tree       # Recursive tree view
lsf --git        # Show git status
lsf -R           # Recursive listing (classic)
lsf -t           # Sort by time
lsf -S           # Sort by size
lsf -X           # Sort by extension
lsf -v           # Natural version sort
lsf --depth 2    # Limit tree depth
lsf --help       # Show help
lsf --version    # Show version
lsf --no-icons   # Disable icons
```

---

## ⚙️ Sorting Options

| Flag | Sort By |
|------|---------|
| `-t` | Time (newest first) |
| `-S` | Size (largest first) |
| `-X` | Extension (alphabetical) |
| `-v` | Version (natural sort) |
| `-r` | Reverse sort order |
| `-U` | Unsorted (directory order) |

---

## 🔗 Set Up Alias

### Bash / Zsh
```bash
alias ls='lsf'
```

### Fish
```fish
alias ls lsf
```

---

## 🔧 Troubleshooting

| Problem | Solution |
|---------|----------|
| Icons show as boxes | Install a [Nerd Font](https://www.nerdfonts.com/) |
| `lsf: command not found` | Add `/usr/local/bin` to PATH |
| Alias not working | Reload shell config: `source ~/.bashrc` |
| No colors | Use `lsf --color=always` |

---

## 📚 Documentation

- **Installation:** See `docs/INSTALLATION.md`
- **Troubleshooting:** See `docs/TROUBLESHOOTING.md`
- **Benchmarks:** See `BENCHMARKS.md`
- **Overview:** See `README.md`

---

## 🌐 Platform Support

| OS | Support |
|----|---------|
| Linux | ✅ Full |
| macOS | ✅ Full |
| WSL (Windows) | ✅ Full |
| BSD | ✅ Full |

---

**Created by [swadhinbiswas](https://github.com/swadhinbiswas)**
