# ls-f

A simple wrapper for `ls` that adds icons. It works just like the `ls` you already know, but looks better.

![Example output screenshot](docs/example.png)

## Why use this?
- **Familiar:** It uses your system's `ls` under the hood. All your favorite flags (`-la`, `-R`, `-h`) just work.
- **Icons:** Automatically adds Nerd Font icons based on file extensions.
- **Simple:** No complex configuration needed. It's just a bash script and a tiny python helper.

## Prerequisites
- **A Nerd Font:** You need a font that supports icons (like FiraCode Nerd Font, JetBrainsMono, etc.) set in your terminal.
- **Python 3:** Used for the pretty grid layout.

## Installation

The easiest way is to use the one-liner:

```bash
curl -fsSL https://raw.githubusercontent.com/swadhinbiswas/ls-f/main/install-standalone.sh | bash
```

This will download `lsf` and its helper to your machine and make them ready to use.

### Manual Install
If you prefer doing it yourself:
1. Clone the repo: `git clone https://github.com/swadhinbiswas/ls-f.git`
2. Enter the folder: `cd ls-f`
3. Run the install script: `./install.sh --user`

## Setting up the Alias
To make `ls` show icons automatically, add an alias to your shell config file.

**For Bash (`~/.bashrc`):**
```bash
echo "alias ls='lsf'" >> ~/.bashrc
source ~/.bashrc
```

**For Zsh (`~/.zshrc`):**
```bash
echo "alias ls='lsf'" >> ~/.zshrc
source ~/.zshrc
```

**For Fish (`~/.config/fish/config.fish`):**
```bash
echo "alias ls='lsf'" >> ~/.config/fish/config.fish
```

## Usage
Just use it like `ls`:
- `ls` (shows icons in a grid)
- `ls -la` (long format with icons)
- `ls -R` (recursive with icons)
- `ls --no-icons` (if you want the plain old boring output)

## Customizing Icons
If you want to change an icon, you can edit the `ICONS` mapping inside the `lsf` script or create a config at `~/.config/lsf`.

## License
MIT
