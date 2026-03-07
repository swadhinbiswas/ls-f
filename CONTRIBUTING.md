# Contributing to ls-f

Thanks for your interest in contributing! ls-f is a modern `ls` replacement written in Rust with Nerd Font icons, git status, and smart formatting.

## Ways to Contribute

- Report bugs and edge cases
- Improve documentation or examples
- Implement new flags or features
- Add tests and improve code quality
- Improve performance

## Getting Started

### Prerequisites

- **Rust** (stable, MSRV 1.74) — install via [rustup](https://rustup.rs/)
- **A Nerd Font** — for icon display ([nerdfonts.com](https://www.nerdfonts.com/))
- **Git** — required for `--git` flag functionality

### Dev Setup

1. Fork and clone the repo:
   ```sh
   git clone https://github.com/<your-username>/ls-f.git
   cd ls-f
   ```

2. Build and run:
   ```sh
   cargo build
   cargo run -- --help
   cargo run -- -l --git src/
   ```

3. Run checks before submitting:
   ```sh
   cargo fmt --all -- --check
   cargo clippy --all-targets -- -D warnings
   cargo test
   ```

## Branch, Commit, and PRs

- Create a feature branch from `main`
- Follow Conventional Commits for messages:
  - `feat: add --git flag with per-file status markers`
  - `fix: handle symlinks in tree view`
  - `docs: update installation instructions`
  - `refactor:`, `test:`, `chore:`, `perf:`, `ci:`, `build:` are also common
- Keep commits focused and small when possible
- Open a Pull Request with:
  - A clear description of the intent and behavior change
  - Notes on any user-facing changes (flags, output, defaults)
  - Screenshots or terminal output examples when helpful
- All PRs must pass CI checks (fmt, clippy, test, build, audit) before merge

## Coding Guidelines

- Follow standard `rustfmt` formatting (see `rustfmt.toml`)
- Keep clippy clean: `cargo clippy -- -D warnings`
- Prefer `std::process::Command` for external tool integration (e.g., git) over heavy crate dependencies
- Use proper error handling — avoid `.unwrap()` in library/production code
- Keep functions focused; use structs to group related parameters when argument count grows

## Tests

- Run the full suite with `cargo test`
- When adding new flags or features:
  - Add unit tests where practical
  - Include smoke tests (binary runs without crashing with the new flag)
- If you change output formatting, include before/after samples in the PR description

## Documentation

- Update `README.md` if you add or change flag behavior
- Add or edit docs in `docs/` when appropriate (INSTALLATION, CONFIGURATION, TROUBLESHOOTING)

## CI Requirements

All PRs must pass these checks before merge:

| Check | Command |
|-------|---------|
| Format | `cargo fmt --all -- --check` |
| Lint | `cargo clippy --all-targets -- -D warnings` |
| Test | `cargo test --verbose` |
| Build | `cargo build --release` |
| Audit | `cargo audit` |
| MSRV | `cargo check` with Rust 1.74 |

## License and DCO

By contributing, you agree that your contributions will be licensed under the project's MIT License (see `LICENSE`).

## Code of Conduct

Please follow our [Code of Conduct](CODE_OF_CONDUCT.md). We aim for a welcoming, respectful community.
