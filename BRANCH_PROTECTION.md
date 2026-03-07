# Branch Protection Rules

This document describes the recommended branch protection settings for `main`.

## Required Status Checks

Go to **Settings → Branches → Branch protection rules → Add rule** for `main`:

### Required checks before merging

Enable **"Require status checks to pass before merging"** and add these required checks:

| Status Check | What it does |
|---|---|
| `CI Pass` | Meta-job that gates on all other checks (single required check) |

The `CI Pass` job in `.github/workflows/test.yml` depends on all of:
- **Rustfmt** — enforces consistent code formatting
- **Clippy** — catches common mistakes and enforces lint rules
- **Test (ubuntu-latest)** — runs `cargo test` on Linux
- **Test (macos-latest)** — runs `cargo test` on macOS
- **Build (ubuntu-latest)** — verifies release build on Linux
- **Build (macos-latest)** — verifies release build on macOS
- **Security Audit** — checks dependencies for known vulnerabilities
- **MSRV Check** — verifies code compiles with minimum supported Rust 1.74

### Other settings

- [x] **Require a pull request before merging**
  - Required approvals: 1 (adjust for team size)
- [x] **Require conversation resolution before merging**
- [x] **Require signed commits** (optional, recommended)
- [x] **Require linear history** (optional, prevents merge commits)
- [x] **Do not allow bypassing the above settings** (even for admins)
- [ ] **Allow force pushes** — disabled
- [ ] **Allow deletions** — disabled

## How to Set Up

1. Go to your repo: https://github.com/swadhinbiswas/ls-f/settings/branches
2. Click **"Add branch protection rule"**
3. Branch name pattern: `main`
4. Check the boxes as described above
5. Under "Require status checks", search for `CI Pass` and select it
6. Click **"Create"**

## For Contributors

Before opening a PR, run these locally:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
```

All checks are automated in CI and must pass before your PR can be merged.
