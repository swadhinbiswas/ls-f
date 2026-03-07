use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Represents the git status of a single file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GitStatus {
    /// File has been modified in the working tree
    Modified,
    /// File is staged (added to index)
    Staged,
    /// File is both staged and has unstaged changes
    StagedModified,
    /// File is untracked
    Untracked,
    /// File has been renamed
    Renamed,
    /// File has been deleted
    Deleted,
    /// File has merge conflicts
    Conflicted,
    /// File is ignored by git
    Ignored,
    /// File is tracked and clean (no changes)
    Clean,
}

impl GitStatus {
    /// Return a short colored marker string for display.
    pub fn marker(&self, use_color: bool) -> &'static str {
        match self {
            GitStatus::Modified => {
                if use_color {
                    "\x1b[33m[M]\x1b[0m"
                } else {
                    "[M]"
                }
            }
            GitStatus::Staged => {
                if use_color {
                    "\x1b[32m[S]\x1b[0m"
                } else {
                    "[S]"
                }
            }
            GitStatus::StagedModified => {
                if use_color {
                    "\x1b[33m[SM]\x1b[0m"
                } else {
                    "[SM]"
                }
            }
            GitStatus::Untracked => {
                if use_color {
                    "\x1b[31m[?]\x1b[0m"
                } else {
                    "[?]"
                }
            }
            GitStatus::Renamed => {
                if use_color {
                    "\x1b[36m[R]\x1b[0m"
                } else {
                    "[R]"
                }
            }
            GitStatus::Deleted => {
                if use_color {
                    "\x1b[31m[D]\x1b[0m"
                } else {
                    "[D]"
                }
            }
            GitStatus::Conflicted => {
                if use_color {
                    "\x1b[1;31m[U]\x1b[0m"
                } else {
                    "[U]"
                }
            }
            GitStatus::Ignored => {
                if use_color {
                    "\x1b[90m[I]\x1b[0m"
                } else {
                    "[I]"
                }
            }
            GitStatus::Clean => "",
        }
    }

    /// Return the display width of the marker (excluding ANSI codes).
    #[allow(dead_code)]
    pub fn marker_width(&self) -> usize {
        match self {
            GitStatus::Clean => 0,
            GitStatus::StagedModified => 4, // "[SM]"
            _ => 3,                         // "[X]"
        }
    }
}

/// A cache of git statuses for all files in a repository.
#[derive(Debug, Clone)]
pub struct GitRepo {
    /// Map from absolute file path to git status
    statuses: HashMap<PathBuf, GitStatus>,
    /// The root of the git repository (working tree)
    #[allow(dead_code)]
    repo_root: PathBuf,
}

impl GitRepo {
    /// Get the git status for a specific file path.
    pub fn status_for(&self, path: &Path) -> GitStatus {
        // Try the canonical/absolute path first
        let abs = if path.is_absolute() {
            path.to_path_buf()
        } else {
            std::env::current_dir()
                .map(|cwd| cwd.join(path))
                .unwrap_or_else(|_| path.to_path_buf())
        };

        if let Some(status) = self.statuses.get(&abs) {
            return *status;
        }

        // For directories, check if any child has a non-clean status
        if path.is_dir() {
            let abs_str = abs.to_string_lossy().to_string();
            let prefix = if abs_str.ends_with('/') {
                abs_str
            } else {
                format!("{}/", abs_str)
            };

            let mut has_staged = false;
            let mut has_modified = false;
            let mut has_untracked = false;
            let mut has_any = false;

            for (file_path, status) in &self.statuses {
                let file_str = file_path.to_string_lossy();
                if file_str.starts_with(&prefix) {
                    match status {
                        GitStatus::Clean | GitStatus::Ignored => {}
                        GitStatus::Staged => {
                            has_staged = true;
                            has_any = true;
                        }
                        GitStatus::Untracked => {
                            has_untracked = true;
                            has_any = true;
                        }
                        _ => {
                            has_modified = true;
                            has_any = true;
                        }
                    }
                }
            }

            if !has_any {
                return GitStatus::Clean;
            }

            if has_staged && has_modified {
                return GitStatus::StagedModified;
            } else if has_modified {
                return GitStatus::Modified;
            } else if has_staged {
                return GitStatus::Staged;
            } else if has_untracked {
                return GitStatus::Untracked;
            }
        }

        GitStatus::Clean
    }
}

/// Discover the git repository root for a given directory.
fn find_repo_root(dir: &Path) -> Option<PathBuf> {
    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .current_dir(dir)
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let root = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if root.is_empty() {
        return None;
    }

    Some(PathBuf::from(root))
}

/// Parse the two-character status code from `git status --porcelain=v1`.
fn parse_status(index: char, worktree: char) -> GitStatus {
    // Conflict states
    if (index == 'U' || worktree == 'U')
        || (index == 'A' && worktree == 'A')
        || (index == 'D' && worktree == 'D')
    {
        return GitStatus::Conflicted;
    }

    // Untracked
    if index == '?' && worktree == '?' {
        return GitStatus::Untracked;
    }

    // Ignored
    if index == '!' && worktree == '!' {
        return GitStatus::Ignored;
    }

    let staged = matches!(index, 'M' | 'A' | 'D' | 'R' | 'C');
    let modified = matches!(worktree, 'M' | 'D');

    if staged && modified {
        GitStatus::StagedModified
    } else if staged {
        if index == 'R' {
            GitStatus::Renamed
        } else if index == 'D' {
            GitStatus::Deleted
        } else {
            GitStatus::Staged
        }
    } else if modified {
        if worktree == 'D' {
            GitStatus::Deleted
        } else {
            GitStatus::Modified
        }
    } else {
        GitStatus::Clean
    }
}

/// Load git status for all files under a directory.
/// Returns None if the directory is not inside a git repository.
pub fn load_git_status(dir: &Path) -> Option<GitRepo> {
    let target_dir = if dir.is_dir() { dir } else { dir.parent()? };
    let repo_root = find_repo_root(target_dir)?;

    let output = Command::new("git")
        .args([
            "status",
            "--porcelain=v1",
            "-uall",     // show individual files in untracked dirs
            "--ignored", // show ignored files too
            "-z",        // NUL-terminated entries
        ])
        .current_dir(&repo_root)
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let raw = String::from_utf8_lossy(&output.stdout);
    let mut statuses: HashMap<PathBuf, GitStatus> = HashMap::new();

    // -z output: entries are separated by NUL, rename entries have an extra NUL-separated field
    let entries: Vec<&str> = raw.split('\0').collect();
    let mut i = 0;
    while i < entries.len() {
        let entry = entries[i];
        if entry.len() < 4 {
            i += 1;
            continue;
        }

        let chars: Vec<char> = entry.chars().collect();
        let index_char = chars[0];
        let worktree_char = chars[1];
        // chars[2] should be a space
        let file_path_str: String = chars[3..].iter().collect();

        let status = parse_status(index_char, worktree_char);

        if status != GitStatus::Clean {
            let abs_path = repo_root.join(&file_path_str);
            statuses.insert(abs_path, status);
        }

        // If this is a rename/copy, the next entry is the original path
        if index_char == 'R' || index_char == 'C' {
            i += 1; // skip the "from" path
        }

        i += 1;
    }

    Some(GitRepo {
        statuses,
        repo_root,
    })
}
