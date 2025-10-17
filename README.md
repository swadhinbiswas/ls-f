# ✨ ls-f — A Beautiful `ls` with Icons

`ls-f` is a friendly drop-in wrapper around GNU `ls` that adds Nerd Font icons, color hints, and a few quality-of-life improvements while still letting your familiar flags pass straight through. Use it as a standalone command or alias it to `ls` for a richer directory view.

## Example Output
![Example output screenshot](docs/example.png)

```bash
$ ./lsf -x
 DOCUMENTATION_SUMMARY.md       IMPLEMENTATION_COMPLETE.md
 INSTALL.md                     LICENSE
 QUICK_REFERENCE.md             README.md
 column.py                      config
 docs                           index.html
 install.sh                     lsf
 test_flags.sh
```

## Quick Start

### Prerequisites
- A Nerd Font (or any font with the necessary glyphs) installed and selected in your terminal
- Bash 4.0+ (icons rely on associative arrays)
- Python 3 (recommended for column formatting; falls back gracefully otherwise)

### Install
```bash
# One-liner (downloads both lsf and column.py automatically)
curl -fsSL https://raw.githubusercontent.com/swadhinbiswas/ls-f/main/install-standalone.sh | bash

# Or clone and install
git clone https://github.com/swadhinbiswas/ls-f.git
cd ls-f
./install.sh --user      # or: sudo ./install.sh --system
```

Note: lsf uses a small Python helper (`column.py`) for grid formatting. The one-liner installer downloads both files automatically. The repository installer also installs both `lsf` and `column.py`. If installing manually, make sure both are present in your PATH.

### Optional: Alias `ls`
```bash
echo "alias ls='lsf'" >> ~/.zshrc   # or ~/.bashrc, ~/.config/fish/config.fish, ...
```
Reload your shell configuration afterwards (e.g. `source ~/.zshrc`).

## Feature Highlights
- Icons & colors that automatically match hundreds of common file types
- Works out-of-the-box with long listings (`-l`), recursion (`-R`), and hidden files (`-A`, `-a`)
- Escape-aware output (`-b`) so icon lookup still succeeds with quirky filenames
- Horizontal layout toggle (`-x`) and extension-based sorting (`-X`)
- Tab-size control (`-T`, `--tabsize=COLS`) that reaches into the bundled formatter
- Security-context display (`-Z`, `--context`) without losing icons
- `--zero` mode for null-terminated output when scripting
- `--no-icons` to fall back to the exact `ls` output

## Supported Flags (Native Integration)
| Flag(s) | Behavior |
|---------|----------|
| `-l` | Long format with icons preserved in the first column |
| `-R` | Recursive listings with colored directory headers |
| `-A`, `--almost-all` | Show hidden files while skipping `.` and `..` |
| `-b`, `--escape` | Treat escape sequences literally in icon lookups |
| `-x` | Row-major layout (matches `ls -x`) |
| `-X` | Sorts by extension while keeping icon formatting |
| `-T`, `--tabsize=COLS` | Custom tab stop width for the grid formatter |
| `-Z`, `--context` | Emit SELinux/AppArmor security contexts with icons |
| `--zero` | Terminate every entry with `\0` for safe piping |
| `--no-icons` | Bypass icon rendering and defer to raw `ls` output |
| `--help` | Display the built-in usage summary |
| `--version` | Print the current `ls-f` version |

> ℹ️ Most other GNU `ls` flags are forwarded untouched to the underlying command. Sorting, grouping, and metadata switches generally work today, but some formats still need first-class, icon-aware handling—see the roadmap below.

## Roadmap: GNU `ls` Compatibility
The table below inventories the remaining GNU `ls` flags and notes their current status inside `ls-f`. Entries marked "Pass-through" already work by delegating to `ls`, but we plan to make their output fully icon-aware. "Planned" items need additional logic before they behave identically to `ls`.

| Option(s) | Description | Status |
|-----------|-------------|--------|
| *(baseline)* | List information about files, sorting alphabetically unless another sort flag is given. | ✅ Native |
| *(arguments)* | Long-option arguments should match their short-option counterparts. | 🛠️ Planned (documentation & validation) |
| `-a`, `--all` | Do not ignore entries starting with `.`. | 🟡 Pass-through (native polish planned) |
| `-A`, `--almost-all` | Do not list implied `.` and `..`. | ✅ Native |
| `--author` | With `-l`, show the author column. | 🟡 Pass-through |
| `-b`, `--escape` | Print C-style escapes for nongraphic characters. | ✅ Native |
| `--block-size=SIZE` | Scale sizes by `SIZE` when printing with `-l`. | 🟡 Pass-through |
| `-B`, `--ignore-backups` | Skip files ending with `~`. | 🟡 Pass-through |
| `-c` | Control ctime display and sorting. | 🟡 Pass-through |
| `-C` | Column-major layout. | 🟡 Pass-through |
| `--color[=WHEN]` | Honour color directives from `ls`. | 🛠️ Planned (currently disabled to avoid double coloring) |
| `-d`, `--directory` | List directories themselves, not their contents. | 🟡 Pass-through |
| `-D`, `--dired` | Emit GNU Emacs `dired` hints. | 🟡 Pass-through (needs validation) |
| `-f` | Same as `-a -U`. | 🟡 Pass-through |
| `-F`, `--classify[=WHEN]` | Append type indicators. | 🟡 Pass-through |
| `--file-type` | Append indicators, excluding `*`. | 🟡 Pass-through |
| `--format=WORD` | Choose layout: across, commas, long, etc. | 🛠️ Planned (current grid reformatting overrides `WORD`) |
| `--full-time` | Show full ISO timestamps. | 🟡 Pass-through |
| `-g` | Like `-l`, omit owner. | 🟡 Pass-through |
| `--group-directories-first` | Group directories before files. | 🟡 Pass-through |
| `-G`, `--no-group` | Suppress group column in long format. | 🟡 Pass-through |
| `-h`, `--human-readable` | Human-readable sizes. | 🟡 Pass-through |
| `--si` | Human-readable sizes base 1000. | 🟡 Pass-through |
| `-H`, `--dereference-command-line` | Follow command-line symlinks. | 🟡 Pass-through |
| `--dereference-command-line-symlink-to-dir` | Follow command-line symlinks to dirs. | 🟡 Pass-through |
| `--hide=PATTERN` | Hide entries matching `PATTERN`. | 🟡 Pass-through |
| `--hyperlink[=WHEN]` | Emit OSC 8 hyperlinks. | 🛠️ Planned (needs escape-safe icon handling) |
| `--indicator-style=WORD` | Control appended indicators. | 🟡 Pass-through |
| `-i`, `--inode` | Print inode numbers. | 🟡 Pass-through |
| `-I`, `--ignore=PATTERN` | Ignore entries matching `PATTERN`. | 🟡 Pass-through |
| `-k`, `--kibibytes` | Use 1024-byte blocks for sizes. | 🟡 Pass-through |
| `-l` | Long listing format. | ✅ Native |
| `-L`, `--dereference` | Follow symlinks when showing info. | 🟡 Pass-through |
| `-m` | Comma-separated output. | 🛠️ Planned (icons currently attach to the first entry only) |
| `-n`, `--numeric-uid-gid` | Show numeric IDs. | 🟡 Pass-through |
| `-N`, `--literal` | Output names without quoting. | 🟡 Pass-through |
| `-o` | Like `-l`, omit group. | 🟡 Pass-through |
| `-p`, `--indicator-style=slash` | Append `/` to directories. | 🟡 Pass-through |
| `-q`, `--hide-control-chars` | Replace nongraphic chars with `?`. | 🟡 Pass-through |
| `--show-control-chars` | Display nongraphic characters as-is. | 🟡 Pass-through |
| `-Q`, `--quote-name` | Quote entry names. | 🛠️ Planned (quotes break icon detection) |
| `--quoting-style=WORD` | Choose quoting style. | 🛠️ Planned (needs decode-aware lookup) |
| `-r`, `--reverse` | Reverse sort order. | 🟡 Pass-through |
| `-R`, `--recursive` | List subdirectories recursively. | ✅ Native |
| `-s`, `--size` | Show allocated blocks. | 🟡 Pass-through |
| `-S` | Sort by file size. | 🟡 Pass-through |
| `--sort=WORD` | Choose ordering (none, size, time, version, extension, name, width). | 🟡 Pass-through |
| `--time=WORD` | Select which timestamp to display/sort. | 🟡 Pass-through |
| `--time-style=TIME_STYLE` | Customize time/date formatting. | 🟡 Pass-through |
| `-t` | Sort by modification time. | 🟡 Pass-through |
| `-T`, `--tabsize=COLS` | Set tab stops. | ✅ Native |
| `-u` | Sort/show access time. | 🟡 Pass-through |
| `-U` | Do not sort directory entries. | 🟡 Pass-through |
| `-v` | Natural sort of numbers. | 🟡 Pass-through |
| `-w`, `--width=COLS` | Set output width. | 🛠️ Planned (formatter currently ignores this setting) |
| `-x` | Row-major layout. | ✅ Native |
| `-X` | Sort by extension. | ✅ Native |
| `-Z`, `--context` | Show security context. | ✅ Native |
| `--zero` | Delimit entries with `NUL`. | ✅ Native |
| `-1` | One entry per line. | 🛠️ Planned (column formatter reflows output) |
| `--help` | Show help and exit. | ✅ Native |
| `--version` | Output version information and exit. | ✅ Native |

Legend: `✅ Native` = icon-aware handling built into `ls-f`; `🟡 Pass-through` = delegated to `ls` today but slated for native polish; `🛠️ Planned` = not yet behaving like GNU `ls` when icons are enabled.

## Next Steps: Development Roadmap

### v4.6.0 - Enhanced Formatting & Display
**Priority: High** | **Target: Q1 2025**

- **`-1` (one per line)**: Override column formatter to respect single-column output
- **`-w`, `--width=COLS`**: Pass terminal width to column.py for proper wrapping
- **`-m` (comma-separated)**: Icon-aware comma formatting for compact output
- **`--color` integration**: Smart color passthrough without conflicting with icons
- **Directory coloring**: Enhance folder icon colors based on permissions/type

### v4.7.0 - Advanced GNU ls Parity
**Priority: Medium** | **Target: Q2 2025**

- **`-Q`, `--quote-name`**: Quote-aware icon detection and placement
- **`--quoting-style=WORD`**: Support all GNU ls quoting styles with icons
- **`-F`, `--classify` native**: Icon + indicator combinations (e.g., 🐍script.py*)
- **`--hyperlink`**: OSC 8 hyperlink support with icon preservation
- **`-g`, `-o`, `-G`**: Native long format variants with proper icon alignment

### v4.8.0 - Performance & Efficiency
**Priority: Medium** | **Target: Q3 2025**

- **Caching layer**: Icon lookup cache for large directories
- **Async processing**: Parallel icon resolution for recursive listings
- **Memory optimization**: Reduce overhead for very large directory trees
- **Config preprocessing**: Compile icon rules for faster matching

### v5.0.0 - Extended Features
**Priority: Low** | **Target: Q4 2025**

- **Custom icon themes**: Theme system (material, nerd, emoji, ascii)
- **Plugin architecture**: External icon providers and formatters
- **Terminal capability detection**: Auto-disable features on limited terminals
- **Network file support**: Icons for remote/mounted filesystems
- **Integration helpers**: Git status indicators, file age colors, size categories

### Beyond v5.0 - Future Vision

- **Shell integration**: Native completions for bash/zsh/fish
- **GUI file manager**: GTK/Qt frontend using ls-f as backend
- **Cross-platform**: Windows PowerShell module, macOS Finder integration
- **Language bindings**: Python/Node.js APIs for programmatic use
- **Cloud support**: S3, Google Drive, Dropbox icon providers

### Contributing Priorities

**🔥 Hot Areas** (easy wins, high impact):
1. Fix `-1` and `-w` flag handling in column.py
2. Add more file type icons (current: ~100, target: 500+)
3. Improve error messages and help text
4. Add regression test coverage for new flags

**💡 Medium Effort**:
1. Quote-style and classifier integration
2. Color scheme management and themes
3. Performance profiling and optimization
4. Windows/macOS platform testing

**🚀 Ambitious Projects**:
1. Plugin system design and implementation
2. GUI frontend development
3. Shell completion generators
4. Language binding APIs

Want to contribute? See [CONTRIBUTING.md](CONTRIBUTING.md) for setup and [the issues page](https://github.com/swadhinbiswas/ls-f/issues) for specific tasks!

## Customization
Override any icon or color in `~/.config/lsf`:
```bash
# nano lsf or nvim lsf
ICONS["dir"]=""
COLORS["dir"]="34"
ICONS[".py"]="🐍"
COLORS[".py"]="33"
```
Reload your shell (or re-run `lsf`) to apply changes. The configuration file is ordinary shell syntax, so you can use conditionals, helper functions, and any POSIX-compatible features.

## Documentation Hub
- `README.md` — you are here (feature overview & roadmap)
- `docs/INSTALLATION.md` — deep-dive installation instructions
- `docs/CONFIGURATION.md` — icon and color customization reference
- `docs/TROUBLESHOOTING.md` — diagnostics and common fixes
- `index.html` — shareable one-page site with highlights and quick commands

## Contributing
Issues and pull requests are welcome! Please document new flags in the tables above and add regression tests or example commands when possible.

- See [CONTRIBUTING.md](CONTRIBUTING.md) for how to set up your environment, commit style, and PR guidelines.
- Please follow our [Code of Conduct](CODE_OF_CONDUCT.md).
- Security issues? Read our [Security Policy](SECURITY.md) and report privately.

## License
Licensed under the MIT License. See [LICENSE](LICENSE) for details.
