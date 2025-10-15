# Release Notes — v4.5.0

This release turns ls-f into a production-ready, icon-enhanced wrapper around GNU ls with strong compatibility and a polished install path.

## Highlights
- Native support: -x, -X, -T/--tabsize, -Z/--context, --zero, -b/--escape
- Smarter formatting powered by column.py (row/column layouts, tabsize)
- Installer with user/system modes and post-install verification
- Comprehensive docs and website with demo screenshot

## What's New
- Horizontal by-lines layout (-x)
- Sort by extension (-X)
- Security context display (-Z/--context)
- NUL-terminated output (--zero)
- C-style escapes for reliable icon lookup (-b/--escape)
- Environment-driven formatter (LSF_LAYOUT, LSF_TABSIZE)

## Installation
- Recommended: git clone and run ./install.sh --user
- Or download both lsf and column.py to your bin directory

## Compatibility
Most GNU ls flags pass through unchanged. See README for the compatibility roadmap with statuses.

## Checksums
SHA256 sums are provided alongside release archives for verification.

## Thanks
Thanks to contributors and users for testing and feedback.
