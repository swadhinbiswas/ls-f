# 📂 ls-f

[![GitHub release (latest by date)](https://img.shields.io/github/v/release/swadhinbiswas/ls-f?color=orange&logo=rust)](https://github.com/swadhinbiswas/ls-f/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

**ls-f** is a modern, blazing-fast `ls` replacement. Originally a bash wrapper, it has been completely rewritten in **Rust (v5.0.0)** to provide superior performance, a native tree view, and zero runtime dependencies.

![Example output screenshot](docs/example.png)

---

## ✨ Key Features

- 🚀 **Performance:** Native Rust implementation, matching GNU `ls` speed even with icons and colors.
- 🎨 **Nerd Font Icons:** Rich set of icons for hundreds of file types and extensions.
- 🌳 **Tree View:** Recursive tree visualization built-in via `--tree`.
- 🌿 **Git Integration:** View file git status (staged, modified, etc.) with `--git`.
- 🌈 **ANSI Colors:** Intelligent colorization for file types, permissions, and metadata.
- ⚙️ **GNU Compatibility:** Supports standard flags: `-l`, `-a`, `-R`, `-h`, `-t`, `-S`, `-v`, and more.
- 📦 **Zero Dependencies:** Single static binary—no Bash or Python required for the Rust version.

---

## 📊 Benchmarks (v5.0.0)

`lsf` is optimized for speed. Benchmarked against GNU `ls` (coreutils 9.1) with `--color=always`.

| Scenario | GNU ls | lsf | Winner |
|---|---|---|---|
| Small dir (grid) | 0.75 ms | **0.65 ms** | 🏆 lsf |
| Large dir (7.3k files) | **10.2 ms** | 12.9 ms | 🏆 GNU ls |
| Long format (large dir) | 18.4 ms | **18.3 ms** | 🏆 lsf |
| Tree View (recursive) | N/A | **17.2 ms** | 🏆 lsf |

> *Icons add ~0% measurable overhead to lsf. See [BENCHMARKS.md](BENCHMARKS.md) for details.*

---

## 🚀 Installation

### Prerequisites
- A **[Nerd Font](https://www.nerdfonts.com/)** must be set in your terminal for icons to render.

### 1. Build from Source (Rust - Recommended)
The Rust version (v5.0.0) provides the best performance and features.
```bash
git clone https://github.com/swadhinbiswas/ls-f.git
cd ls-f
make install  # Installs to /usr/local/bin
```
*Or via cargo:* `cargo install --path .`

---

## 📖 Usage

Simply use `lsf` where you would use `ls`.

```bash
lsf              # Grid view with icons
lsf -la          # Long format, hidden files
lsf --tree       # Recursive tree view
lsf --git        # Show git status indicators
lsf --depth 2    # Limit tree/recursive depth
```

### Setting up the Alias
Add this to your shell profile (`.bashrc`, `.zshrc`, or `.config/fish/config.fish`):
```bash
alias ls='lsf'
```

---

## 🔧 Troubleshooting

- **Icons look like boxes?** You need a [Nerd Font](https://www.nerdfonts.com/). Check terminal font settings.
- **Command not found?** Ensure `/usr/local/bin` or `~/.local/bin` is in your `$PATH`.
- **Colors missing?** Ensure your terminal supports ANSI colors. Try `lsf --color=always`.

*Detailed guides:*
- [Installation Guide](docs/INSTALLATION.md)
- [Troubleshooting Guide](docs/TROUBLESHOOTING.md)
- [Configuration (Legacy)](docs/CONFIGURATION.md)

---

## 🤝 Contributing

Contributions are welcome!
1. Fork the repo.
2. Create your branch (`git checkout -b feature/AmazingFeature`).
3. Commit your changes (`git commit -m 'Add AmazingFeature'`).
4. Push to the branch (`git push origin feature/AmazingFeature`).
5. Open a Pull Request.

---

## 📄 License

Distributed under the MIT License. See `LICENSE` for details.

---

**Created by [swadhinbiswas](https://github.com/swadhinbiswas)**
