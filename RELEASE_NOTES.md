# Release Notes — v4.5.1

Documentation improvements and installation convenience update.

## New Features
- **Standalone installer**: One-liner installation via `curl ... | bash`
- **Development roadmap**: Comprehensive plan through v5.0.0 with milestones
- **Automated releases**: GitHub Actions workflow builds artifacts on tags

## Fixes
- Clarify installation requires both `lsf` and `column.py`
- Restore convenient pipe-to-bash installation method

## How to Install
```bash
# One-liner (recommended)
curl -fsSL https://raw.githubusercontent.com/swadhinbiswas/ls-f/main/install-standalone.sh | bash

# Or clone and install
git clone https://github.com/swadhinbiswas/ls-f.git && cd ls-f && ./install.sh --user
```

## Coming Next
See the roadmap in README.md for upcoming features like `-1`, `-w`, `-m` flag support and performance improvements.

## Checksums
SHA256 sums are provided with the release archives for verification.
