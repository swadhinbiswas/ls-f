# Contributing to ls-f

Thanks for your interest in contributing! This project aims to be a friendly, production-quality wrapper around GNU `ls` with icons, colors, and smart formatting.

## Ways to Contribute
- Report bugs and edge cases
- Improve documentation or examples
- Implement or polish GNU `ls` flags (see the compatibility table in `README.md`)
- Add tests and tooling

## Getting Started (Dev Setup)
1. Fork and clone the repo
2. Use a recent Bash (4+) and Python 3
3. Run a quick local check:
   - `./lsf --version`
   - `./lsf -x`
   - `./test_flags.sh` (basic sanity tests)

No build step is required. The main script is `lsf`; `column.py` is a helper used for grid formatting.

## Branch, Commit, and PRs
- Create a feature branch from `main`
- Follow Conventional Commits for messages:
  - `feat: add -x layout by lines`
  - `fix: handle escaped filenames in grid view`
  - `docs: update installation instructions`
  - `refactor:, test:, chore:, perf:, ci:, build:` are also common
- Keep commits focused and small when possible
- Open a Pull Request with:
  - A clear description of the intent and behavior change
  - Notes on any user-facing changes (flags, output, defaults)
  - Screenshots or examples when helpful

## Coding Guidelines
- Bash
  - Prefer portability and clarity over cleverness
  - Keep exit codes and error messages helpful
  - Be careful with quoting and spaces in filenames
  - Avoid unnecessary subshells and external utilities on hot paths
- Python (`column.py`)
  - Keep it dependency-free and fast (stdlib only)
  - Support row/column layouts via `LSF_LAYOUT`
  - Respect `LSF_TABSIZE` for alignment

## Tests
- Add or update quick checks in `test_flags.sh`
- Include at least:
  - Happy path
  - One or two edge cases (hidden files, escaped names, long names)
- If you change output formatting, include before/after samples in the PR description

## Documentation
- Update `README.md` tables if you add/adjust flag behavior
- Add or edit docs in `docs/` when appropriate (INSTALLATION, CONFIGURATION, TROUBLESHOOTING)

## License and DCO
By contributing, you agree that your contributions will be licensed under the project’s MIT License (see `LICENSE`).

## Code of Conduct
Please follow our [Code of Conduct](CODE_OF_CONDUCT.md). We aim for a welcoming, respectful community.
