use std::collections::HashMap;
use std::fs;
use std::os::unix::fs::{FileTypeExt, MetadataExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use crate::cli::Args;

/// Represents a single file/directory entry with all metadata needed for display.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FileEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub is_symlink: bool,
    pub is_executable: bool,
    pub is_hidden: bool,
    pub is_pipe: bool,
    pub is_socket: bool,
    pub is_block_device: bool,
    pub is_char_device: bool,
    pub size: u64,
    pub modified: Option<SystemTime>,
    pub accessed: Option<SystemTime>,
    pub created: Option<SystemTime>,
    pub mode: u32,
    pub nlink: u64,
    pub uid: u32,
    pub gid: u32,
    pub blocks: u64,
    pub inode: u64,
    pub symlink_target: Option<String>,
    pub extension: String,
}

impl FileEntry {
    /// Create a FileEntry from a path, reading metadata.
    pub fn from_path(path: &Path, follow_symlinks: bool) -> std::io::Result<Self> {
        let symlink_meta = fs::symlink_metadata(path)?;
        let is_symlink = symlink_meta.file_type().is_symlink();

        let meta = if follow_symlinks && is_symlink {
            fs::metadata(path).unwrap_or_else(|_| symlink_meta.clone())
        } else {
            symlink_meta.clone()
        };

        let file_type = meta.file_type();
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| path.to_string_lossy().into_owned());

        let is_dir = file_type.is_dir();
        let is_executable = !is_dir && (meta.permissions().mode() & 0o111 != 0);
        let is_hidden = name.starts_with('.');
        let is_pipe = file_type.is_fifo();
        let is_socket = file_type.is_socket();
        let is_block_device = file_type.is_block_device();
        let is_char_device = file_type.is_char_device();

        let symlink_target = if is_symlink {
            fs::read_link(path)
                .ok()
                .map(|t| t.to_string_lossy().into_owned())
        } else {
            None
        };

        let extension = path
            .extension()
            .map(|e| e.to_string_lossy().into_owned())
            .unwrap_or_default();

        Ok(FileEntry {
            name,
            path: path.to_path_buf(),
            is_dir,
            is_symlink,
            is_executable,
            is_hidden,
            is_pipe,
            is_socket,
            is_block_device,
            is_char_device,
            size: meta.len(),
            modified: meta.modified().ok(),
            accessed: meta.accessed().ok(),
            created: meta.created().ok(),
            mode: meta.mode(),
            nlink: meta.nlink(),
            uid: meta.uid(),
            gid: meta.gid(),
            blocks: meta.blocks(),
            inode: meta.ino(),
            symlink_target,
            extension,
        })
    }

    /// Create synthetic . and .. entries for a directory
    pub fn dot_entries(dir: &Path) -> Vec<FileEntry> {
        let mut entries = Vec::with_capacity(2);

        // "."
        if let Ok(mut entry) = FileEntry::from_path(dir, false) {
            entry.name = ".".to_string();
            entries.push(entry);
        }

        // ".."
        let parent = dir.parent().unwrap_or(dir);
        if let Ok(mut entry) = FileEntry::from_path(parent, false) {
            entry.name = "..".to_string();
            entries.push(entry);
        }

        entries
    }
}

/// Read directory entries, filtered according to the arguments.
pub fn read_directory(dir: &Path, args: &Args) -> std::io::Result<Vec<FileEntry>> {
    let mut entries = Vec::new();

    // Add . and .. if requested
    if args.show_dot_dirs() {
        entries.extend(FileEntry::dot_entries(dir));
    }

    let follow_symlinks = args.dereference;

    let rd = fs::read_dir(dir)?;
    for entry_result in rd {
        let entry = match entry_result {
            Ok(e) => e,
            Err(_) => continue,
        };

        let name = entry.file_name().to_string_lossy().into_owned();

        // Skip hidden files unless requested
        if name.starts_with('.') && !args.show_hidden() {
            continue;
        }

        // Skip backup files if requested
        if args.ignore_backups && name.ends_with('~') {
            continue;
        }

        // Skip files matching ignore pattern
        if let Some(ref pattern) = args.ignore_pattern {
            if matches_glob_simple(&name, pattern) {
                continue;
            }
        }

        // Skip files matching hide pattern (unless -a or -A)
        if !args.show_hidden() {
            if let Some(ref pattern) = args.hide_pattern {
                if matches_glob_simple(&name, pattern) {
                    continue;
                }
            }
        }

        let path = entry.path();
        match FileEntry::from_path(&path, follow_symlinks) {
            Ok(fe) => entries.push(fe),
            Err(_) => {
                // If we can't read metadata, create a minimal entry
                entries.push(FileEntry {
                    name,
                    path,
                    is_dir: false,
                    is_symlink: false,
                    is_executable: false,
                    is_hidden: true,
                    is_pipe: false,
                    is_socket: false,
                    is_block_device: false,
                    is_char_device: false,
                    size: 0,
                    modified: None,
                    accessed: None,
                    created: None,
                    mode: 0,
                    nlink: 0,
                    uid: 0,
                    gid: 0,
                    blocks: 0,
                    inode: 0,
                    symlink_target: None,
                    extension: String::new(),
                });
            }
        }
    }

    Ok(entries)
}

/// Sort entries according to the arguments.
pub fn sort_entries(entries: &mut Vec<FileEntry>, args: &Args) {
    if args.no_sort() {
        return;
    }

    // Determine the primary sort
    if args.sort_size {
        entries.sort_by(|a, b| b.size.cmp(&a.size));
    } else if args.sort_time {
        entries.sort_by(|a, b| {
            let ta = a.modified.unwrap_or(SystemTime::UNIX_EPOCH);
            let tb = b.modified.unwrap_or(SystemTime::UNIX_EPOCH);
            tb.cmp(&ta)
        });
    } else if args.sort_extension {
        entries.sort_by(|a, b| {
            let ea = a.extension.to_lowercase();
            let eb = b.extension.to_lowercase();
            ea.cmp(&eb)
                .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
        });
    } else if args.version_sort {
        entries.sort_by(|a, b| version_compare(&a.name, &b.name));
    } else {
        // Default: alphabetical (case-insensitive)
        entries.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    }

    // Group directories first if requested
    if args.group_directories_first {
        entries.sort_by(|a, b| b.is_dir.cmp(&a.is_dir));
    }

    // Reverse if requested
    if args.reverse {
        entries.reverse();
    }
}

/// Simple glob matching (supports * and ? wildcards)
fn matches_glob_simple(name: &str, pattern: &str) -> bool {
    let mut name_chars = name.chars().peekable();
    let mut pat_chars = pattern.chars().peekable();

    fn matches_inner(
        name: &mut std::iter::Peekable<std::str::Chars>,
        pat: &mut std::iter::Peekable<std::str::Chars>,
    ) -> bool {
        loop {
            match (pat.peek().copied(), name.peek().copied()) {
                (None, None) => return true,
                (None, Some(_)) => return false,
                (Some('*'), _) => {
                    pat.next();
                    if pat.peek().is_none() {
                        return true;
                    }
                    // Try matching * against zero or more chars
                    loop {
                        let mut pat_clone = pat.clone();
                        let mut name_clone = name.clone();
                        if matches_inner(&mut name_clone, &mut pat_clone) {
                            return true;
                        }
                        if name.next().is_none() {
                            return false;
                        }
                    }
                }
                (Some('?'), Some(_)) => {
                    pat.next();
                    name.next();
                }
                (Some(p), Some(n)) if p == n => {
                    pat.next();
                    name.next();
                }
                _ => return false,
            }
        }
    }

    matches_inner(&mut name_chars, &mut pat_chars)
}

/// Natural/version sort comparison
fn version_compare(a: &str, b: &str) -> std::cmp::Ordering {
    let mut ai = a.chars().peekable();
    let mut bi = b.chars().peekable();

    loop {
        match (ai.peek(), bi.peek()) {
            (None, None) => return std::cmp::Ordering::Equal,
            (None, Some(_)) => return std::cmp::Ordering::Less,
            (Some(_), None) => return std::cmp::Ordering::Greater,
            (Some(&ac), Some(&bc)) => {
                if ac.is_ascii_digit() && bc.is_ascii_digit() {
                    // Compare numeric segments
                    let mut an = String::new();
                    let mut bn = String::new();
                    while let Some(&c) = ai.peek() {
                        if c.is_ascii_digit() {
                            an.push(c);
                            ai.next();
                        } else {
                            break;
                        }
                    }
                    while let Some(&c) = bi.peek() {
                        if c.is_ascii_digit() {
                            bn.push(c);
                            bi.next();
                        } else {
                            break;
                        }
                    }
                    let na: u64 = an.parse().unwrap_or(0);
                    let nb: u64 = bn.parse().unwrap_or(0);
                    match na.cmp(&nb) {
                        std::cmp::Ordering::Equal => continue,
                        other => return other,
                    }
                } else {
                    let al = ac.to_lowercase().next().unwrap_or(ac);
                    let bl = bc.to_lowercase().next().unwrap_or(bc);
                    match al.cmp(&bl) {
                        std::cmp::Ordering::Equal => {
                            // If lowercase-equal, compare original case
                            match ac.cmp(&bc) {
                                std::cmp::Ordering::Equal => {
                                    ai.next();
                                    bi.next();
                                }
                                other => {
                                    ai.next();
                                    bi.next();
                                    return other;
                                }
                            }
                        }
                        other => return other,
                    }
                }
            }
        }
    }
}

/// Gather entries for a single path argument.
/// If `-d` is set, lists the directory itself rather than its contents.
#[allow(dead_code)]
pub fn gather_entries(path: &Path, args: &Args) -> std::io::Result<Vec<FileEntry>> {
    let follow = args.dereference || args.dereference_command_line;

    if args.directory {
        let entry = FileEntry::from_path(path, follow)?;
        return Ok(vec![entry]);
    }

    if path.is_dir() {
        read_directory(path, args)
    } else {
        let entry = FileEntry::from_path(path, follow)?;
        Ok(vec![entry])
    }
}

/// Map of user IDs to user names (cached).
pub fn get_username(uid: u32, cache: &mut HashMap<u32, String>) -> String {
    if let Some(name) = cache.get(&uid) {
        return name.clone();
    }
    let name = uzers::get_user_by_uid(uid)
        .map(|u| u.name().to_string_lossy().into_owned())
        .unwrap_or_else(|| uid.to_string());
    cache.insert(uid, name.clone());
    name
}

/// Map of group IDs to group names (cached).
pub fn get_groupname(gid: u32, cache: &mut HashMap<u32, String>) -> String {
    if let Some(name) = cache.get(&gid) {
        return name.clone();
    }
    let name = uzers::get_group_by_gid(gid)
        .map(|g| g.name().to_string_lossy().into_owned())
        .unwrap_or_else(|| gid.to_string());
    cache.insert(gid, name.clone());
    name
}
