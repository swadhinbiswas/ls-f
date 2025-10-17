# Changelog

All notable changes to this project will be documented in this file.

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

[4.5.0]: https://github.com/swadhinbiswas/ls-f/releases/tag/v4.5.0
