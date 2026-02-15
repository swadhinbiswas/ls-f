# Changelog

All notable changes to this project will be documented in this file.

## [5.0.1] - 2026-02-15
### Fixed
- **Security Vulnerabilities**:
  - Replaced unmaintained `users` crate with `uzers` v0.12 to address RUSTSEC-2023-0005.
  - Removed unused `toml` and `serde` dependencies to reduce attack surface and resolve transitive vulnerabilities.
### Changed
- **Dependencies**:
  - Updated `clap` to v4.5.
  - General update of all transitive dependencies to latest compatible versions.

## [5.0.0] - 2026-02-15
### Added
- **Complete Rewrite in Rust**: Transitioned from a Bash/Python hybrid to a native Rust implementation for massive performance gains.
- **Tree View**: Added native tree visualization via the `--tree` flag.
- **Git Integration**: Built-in support for displaying git status indicators using the `--git` flag.
- **Zero Dependencies**: The tool is now a single static binary with no runtime dependencies on Bash or Python.
- **Improved Benchmarks**: Competitive with and often faster than GNU `ls` for colorized output.
- **Native Recursive Sorting**: Improved handling of recursive listings and depth control.

### Changed
- **Performance**: Significant reduction in startup time and listing speed for large directories.
- **Documentation**: Overhauled README, Installation, and Troubleshooting guides to reflect the Rust transition.
- **Installation**: New `make install` and `cargo install` methods added to the preferred workflow.

### Fixed
- Fixed icon misalignment issues in long listing formats.
- Resolved various shell-specific escaping bugs from the legacy version.

## [4.5.1] - 2025-10-17
### Added
- Standalone installer (install-standalone.sh) for convenient pipe-to-bash installation
- Comprehensive development roadmap with versioned milestones through v5.0.0
- GitHub Actions workflow to build and upload release artifacts on tags
- Makefile `release` target to build local artifacts

### Fixed
- Documentation: correct GitHub owner to `swadhinbiswas` across README, site, and docs
- Installation: clarify that both `lsf` and `column.py` are required; restore pipe-to-bash convenience

### Changed
- README: embed example screenshot; improve customization examples
- Primary installation method is now one-liner: `curl ... | bash`

## [4.5.0] - 2025-10-16
### Added
- Horizontal layout (-x) with row-major grid formatting
- Extension sorting (-X) with icon preservation
- Tab-size control (-T, --tabsize=COLS) passed to formatter
- Security context display (-Z, --context)
- NUL-terminated output (--zero)
- C-style escape handling (-b, --escape)
- Python column formatter (column.py) with LSF_LAYOUT and LSF_TABSIZE
- Installer (install.sh) with --user/--system modes and verification
- Configuration alias script (config/lsf_alias_setup.sh)
- Docs: INSTALLATION, CONFIGURATION, COMPATIBILITY, TROUBLESHOOTING, QUICK_REFERENCE
- One-page site (index.html) and example screenshot

### Changed
- Refactor complex view to preserve icons with -Z and -l
- Add decode_c_escapes() and print_output_line() utilities
- Improved parse_args for combined short options (e.g., -lx, -bZ)
- LICENSE year updated to 2025

### Removed
- Deprecated ls-material.sh script

[5.0.1]: https://github.com/swadhinbiswas/ls-f/compare/v5.0.0...v5.0.1
[5.0.0]: https://github.com/swadhinbiswas/ls-f/compare/v4.5.1...v5.0.0
[4.5.1]: https://github.com/swadhinbiswas/ls-f/compare/v4.5.0...v4.5.1
[4.5.0]: https://github.com/swadhinbiswas/ls-f/releases/tag/v4.5.0
