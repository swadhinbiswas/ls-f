# Configuration Guide for ls-f

Learn how to customize **ls-f** icons, colors, and behavior to match your style.

---

## 📁 Configuration File

The configuration file is located at: `~/.config/lsf/lsf.conf`

This is a Bash script that sets up custom icons and colors. If it doesn't exist, `install.sh` will create a default one.

---

## 🎨 Customizing Icons and Colors

### Basic Structure

The configuration file uses associative arrays:

```bash
# ~/.config/lsf/lsf.conf

# Override icon for a file type
ICONS[".py"]="🐍"

# Override color for a file type (ANSI codes)
COLORS[".py"]="92"  # Bright green
```

### Icon Values

Icons can be:
- **Unicode characters**: `ICONS[".py"]="🐍"`
- **Unicode escapes**: `ICONS[".py"]="\U0001F40D"`
- **Nerd Font glyphs**: `ICONS[".py"]=""`
- **Plain text**: `ICONS[".readme"]="README"`

### Color Values

Colors are **ANSI escape codes**. Common values:

| Color | Code | Bright | Code |
|-------|------|--------|------|
| Black | 30 | 90 |
| Red | 31 | 91 |
| Green | 32 | 92 |
| Yellow | 33 | 93 |
| Blue | 34 | 94 |
| Magenta | 35 | 95 |
| Cyan | 36 | 96 |
| White | 37 | 97 |

For 256-color or true color support, use:
```bash
# 256-color: 38;5;<color_number>
COLORS[".py"]="38;5;33"  # Dark blue

# True color (RGB): 38;2;<r>;<g>;<b>
COLORS[".py"]="38;2;100;150;200"  # Custom RGB blue
```

---

## 📝 Configuration Examples

### Example 1: Custom Python Icon

```bash
# ~/.config/lsf/lsf.conf
ICONS[".py"]=""
COLORS[".py"]="92"
```

### Example 2: Custom Directory Icon

```bash
# ~/.config/lsf/lsf.conf
ICONS["dir"]=""
COLORS["dir"]="94"  # Bright blue

# Hidden directories
ICONS["dir_hidden"]="."
COLORS["dir_hidden"]="90"  # Bright black
```

### Example 3: Custom Config File Icons

```bash
# ~/.config/lsf/lsf.conf
ICONS[".json"]=""
COLORS[".json"]="93"  # Bright yellow

ICONS[".yaml"]=""
COLORS[".yaml"]="91"  # Bright red

ICONS[".toml"]=""
COLORS[".toml"]="35"  # Magenta
```

### Example 4: Custom Executable Icons

```bash
# ~/.config/lsf/lsf.conf
ICONS[".sh"]=""
COLORS[".sh"]="32"  # Green

ICONS[".py"]=""
COLORS[".py"]="92"  # Bright green
```

### Example 5: Full Customization

```bash
# ~/.config/lsf/lsf.conf

# Directories
ICONS["dir"]=""
COLORS["dir"]="94"

ICONS["dir_hidden"]="."
COLORS["dir_hidden"]="34"

# Programming Languages
ICONS[".py"]=""
COLORS[".py"]="92"

ICONS[".js"]=""
COLORS[".js"]="93"

ICONS[".rs"]=""
COLORS[".rs"]="91"

# Config Files
ICONS[".json"]=""
COLORS[".json"]="36"

ICONS[".yaml"]=""
COLORS[".yaml"]="31"

# Documents
ICONS[".pdf"]=""
COLORS[".pdf"]="31"

ICONS[".md"]=""
COLORS[".md"]="36"
```

---

## 🔍 Available File Type Keys

The following keys can be customized:

### Directories
- `dir` — Regular directory
- `dir_hidden` — Hidden directory (starts with `.`)

### Configuration Files
- `.gitignore`, `.gitconfig`, `.editorconfig`
- `.json`, `.yaml`, `.toml`, `.xml`
- `.env`, `.conf`, `.cfg`

### Programming Languages
- `.py`, `.js`, `.ts`, `.rs`, `.go`, `.c`, `.cpp`, `.java`, `.rb`, `.php`
- `.sh`, `.bash`, `.zsh`, `.fish`
- And many more (see the source code for complete list)

### Documents
- `.md`, `.txt`, `.pdf`, `.doc`, `.docx`

### Media
- `.png`, `.jpg`, `.gif`, `.mp4`, `.mp3`, `.zip`

### Special Files
- `package.json`, `requirements.txt`, `Dockerfile`, `LICENSE`, `README.md`

For a complete list, check `lsf` source code around line 80-370.

---

## 🔧 Advanced Configuration

### Conditional Colors (True Color)

Use RGB values for precise colors:

```bash
# ~/.config/lsf/lsf.conf

# 24-bit true color
ICONS[".py"]=""
COLORS[".py"]="38;2;70;130;180"  # Steel blue RGB

ICONS[".js"]=""
COLORS[".js"]="38;2;240;180;30"  # Gold RGB
```

### No Color (Monochrome)

Disable colors entirely:

```bash
# ~/.config/lsf/lsf.conf

# Override all colors with default (white/no color)
ICONS["default"]=""
COLORS["default"]="37"
```

### Override Default Icon

```bash
# ~/.config/lsf/lsf.conf

# Set default icon for unknown file types
ICONS["default"]="?"
COLORS["default"]="37"
```

---

## 🛠️ Testing Configuration

After modifying `~/.config/lsf/lsf.conf`, test your changes:

```bash
# Reload shell to ensure config is read
source ~/.bashrc

# List files to see your custom icons
lsf

# Force reread config
rm ~/.config/lsf/lsf.conf  # remove cache if any
lsf
```

---

## 🚨 Troubleshooting Configuration

### Icons Not Changing

1. **Ensure Nerd Fonts are installed:**
   ```bash
   fc-list | grep -i "Nerd Font"
   ```

2. **Check config file syntax:**
   ```bash
   bash -n ~/.config/lsf/lsf.conf
   ```

3. **Reload shell:**
   ```bash
   exec $SHELL
   ```

### Colors Look Wrong

1. **Check terminal ANSI color support:**
   ```bash
   # In terminal: should show color bars
   for i in {30..37}; do echo -e "\e[${i}m Color $i \e[0m"; done
   ```

2. **Verify color codes in config:**
   ```bash
   cat ~/.config/lsf/lsf.conf | grep COLORS
   ```

### Configuration Not Applied

1. **Verify `lsf` reads the config:**
   ```bash
   lsf --version
   ```

2. **Check file permissions:**
   ```bash
   ls -la ~/.config/lsf/
   ```

3. **Ensure PATH includes config dir:**
   ```bash
   echo $HOME
   lsf  # should use ~/.config/lsf/lsf.conf
   ```

---

## 📚 Resources

- **ANSI Color Codes:** https://en.wikipedia.org/wiki/ANSI_escape_code#Colors
- **Nerd Fonts:** https://www.nerdfonts.com/
- **Unicode Characters:** https://unicode-table.com/

---

## 💡 Tips

1. **Keep it simple:** Start with just a few custom icons, then expand.
2. **Use consistent colors:** Pick a color scheme and stick with it.
3. **Test changes:** Run `lsf` after each change to verify.
4. **Back up config:** Keep a backup of working configurations.

---

Need help? Open an issue: https://github.com/swadhin-biswas/ls-f/issues
