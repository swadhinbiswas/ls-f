use std::collections::HashMap;
use std::time::SystemTime;

use chrono::{DateTime, Local};

use crate::entry::FileEntry;

/// Format the permission mode as an `ls -l` style string (e.g., drwxr-xr-x).
pub fn format_permissions(mode: u32, is_dir: bool, is_symlink: bool) -> String {
    let mut s = String::with_capacity(10);

    // File type character
    if is_symlink {
        s.push('l');
    } else if is_dir {
        s.push('d');
    } else if mode & 0o170000 == 0o010000 {
        s.push('p'); // FIFO
    } else if mode & 0o170000 == 0o020000 {
        s.push('c'); // char device
    } else if mode & 0o170000 == 0o060000 {
        s.push('b'); // block device
    } else if mode & 0o170000 == 0o140000 {
        s.push('s'); // socket
    } else {
        s.push('-');
    }

    // Owner
    s.push(if mode & 0o400 != 0 { 'r' } else { '-' });
    s.push(if mode & 0o200 != 0 { 'w' } else { '-' });
    s.push(if mode & 0o4000 != 0 {
        if mode & 0o100 != 0 {
            's'
        } else {
            'S'
        }
    } else {
        if mode & 0o100 != 0 {
            'x'
        } else {
            '-'
        }
    });

    // Group
    s.push(if mode & 0o040 != 0 { 'r' } else { '-' });
    s.push(if mode & 0o020 != 0 { 'w' } else { '-' });
    s.push(if mode & 0o2000 != 0 {
        if mode & 0o010 != 0 {
            's'
        } else {
            'S'
        }
    } else {
        if mode & 0o010 != 0 {
            'x'
        } else {
            '-'
        }
    });

    // Others
    s.push(if mode & 0o004 != 0 { 'r' } else { '-' });
    s.push(if mode & 0o002 != 0 { 'w' } else { '-' });
    s.push(if mode & 0o1000 != 0 {
        if mode & 0o001 != 0 {
            't'
        } else {
            'T'
        }
    } else {
        if mode & 0o001 != 0 {
            'x'
        } else {
            '-'
        }
    });

    s
}

/// Format a file size for display.
pub fn format_size(size: u64, human: bool, si: bool) -> String {
    if !human {
        return size.to_string();
    }

    let base: f64 = if si { 1000.0 } else { 1024.0 };
    let units = if si {
        &["B", "kB", "MB", "GB", "TB", "PB"][..]
    } else {
        &["", "K", "M", "G", "T", "P"][..]
    };

    if size == 0 {
        return if si {
            "0B".to_string()
        } else {
            "0".to_string()
        };
    }

    let mut val = size as f64;
    let mut unit_idx = 0;

    while val >= base && unit_idx < units.len() - 1 {
        val /= base;
        unit_idx += 1;
    }

    if unit_idx == 0 {
        format!("{}{}", size, units[0])
    } else if val < 10.0 {
        format!("{:.1}{}", val, units[unit_idx])
    } else {
        format!("{:.0}{}", val, units[unit_idx])
    }
}

/// Format a timestamp for display (like ls).
pub fn format_time(time: Option<SystemTime>) -> String {
    let time = match time {
        Some(t) => t,
        None => return "            ".to_string(), // 12 chars
    };

    let datetime: DateTime<Local> = time.into();
    let now = Local::now();
    let six_months_ago = now - chrono::Duration::days(182);

    if datetime < six_months_ago || datetime > now + chrono::Duration::hours(1) {
        // Show year for old/future files
        datetime.format("%b %e  %Y").to_string()
    } else {
        // Show time for recent files
        datetime.format("%b %e %H:%M").to_string()
    }
}

/// Format a long-listing line for a single entry.
pub fn format_long_entry(
    entry: &FileEntry,
    icon_str: &str,
    show_icons: bool,
    show_owner: bool,
    show_group: bool,
    show_inode: bool,
    show_blocks: bool,
    show_author: bool,
    human_readable: bool,
    si: bool,
    numeric_ids: bool,
    use_color: bool,
    user_cache: &mut HashMap<u32, String>,
    group_cache: &mut HashMap<u32, String>,
    // Width hints for alignment
    nlink_width: usize,
    owner_width: usize,
    group_width: usize,
    size_width: usize,
    inode_width: usize,
    blocks_width: usize,
) -> String {
    let mut line = String::with_capacity(128);

    // Inode
    if show_inode {
        line.push_str(&format!("{:>width$} ", entry.inode, width = inode_width));
    }

    // Blocks
    if show_blocks {
        let blocks = entry.blocks / 2; // Convert 512-byte blocks to 1K blocks
        line.push_str(&format!("{:>width$} ", blocks, width = blocks_width));
    }

    // Icon
    if show_icons {
        line.push_str(icon_str);
        line.push(' ');
    }

    // Permissions
    let perms = format_permissions(entry.mode, entry.is_dir, entry.is_symlink);
    if use_color {
        line.push_str(&colorize_permissions(&perms));
    } else {
        line.push_str(&perms);
    }

    // Hard link count
    line.push_str(&format!(" {:>width$}", entry.nlink, width = nlink_width));

    // Owner
    if show_owner {
        let owner = if numeric_ids {
            entry.uid.to_string()
        } else {
            crate::entry::get_username(entry.uid, user_cache)
        };
        line.push_str(&format!(" {:<width$}", owner, width = owner_width));
    }

    // Group
    if show_group {
        let group = if numeric_ids {
            entry.gid.to_string()
        } else {
            crate::entry::get_groupname(entry.gid, group_cache)
        };
        line.push_str(&format!(" {:<width$}", group, width = group_width));
    }

    // Author (same as owner for now)
    if show_author {
        let author = if numeric_ids {
            entry.uid.to_string()
        } else {
            crate::entry::get_username(entry.uid, user_cache)
        };
        line.push_str(&format!(" {:<width$}", author, width = owner_width));
    }

    // Size
    let size_str = format_size(entry.size, human_readable, si);
    line.push_str(&format!(" {:>width$}", size_str, width = size_width));

    // Timestamp
    let time_str = format_time(entry.modified);
    line.push_str(&format!(" {}", time_str));

    // Filename (with color if applicable)
    line.push(' ');
    line.push_str(&colorize_filename(entry, use_color));

    // Symlink target
    if entry.is_symlink {
        if let Some(ref target) = entry.symlink_target {
            line.push_str(" -> ");
            if use_color {
                line.push_str(&format!("\x1b[36m{}\x1b[0m", target));
            } else {
                line.push_str(target);
            }
        }
    }

    // Classify indicator
    // (handled in the caller)

    line
}

/// Colorize a filename based on its type.
pub fn colorize_filename(entry: &FileEntry, use_color: bool) -> String {
    if !use_color {
        return entry.name.clone();
    }

    if entry.is_dir {
        format!("\x1b[1;34m{}\x1b[0m", entry.name)
    } else if entry.is_symlink {
        format!("\x1b[36m{}\x1b[0m", entry.name)
    } else if entry.is_executable {
        format!("\x1b[1;32m{}\x1b[0m", entry.name)
    } else if entry.is_pipe {
        format!("\x1b[33m{}\x1b[0m", entry.name)
    } else if entry.is_socket {
        format!("\x1b[1;35m{}\x1b[0m", entry.name)
    } else if entry.is_block_device || entry.is_char_device {
        format!("\x1b[1;33m{}\x1b[0m", entry.name)
    } else {
        // Check extension for known media/archive types
        let ext = entry.extension.to_lowercase();
        match ext.as_str() {
            // Archives
            "zip" | "tar" | "gz" | "bz2" | "xz" | "7z" | "rar" | "tgz" | "tbz2" | "txz" | "zst"
            | "deb" | "rpm" | "iso" | "dmg" => {
                format!("\x1b[1;31m{}\x1b[0m", entry.name)
            }
            // Images
            "png" | "jpg" | "jpeg" | "gif" | "svg" | "webp" | "bmp" | "ico" | "tiff" | "tif"
            | "psd" | "heic" | "avif" => {
                format!("\x1b[35m{}\x1b[0m", entry.name)
            }
            // Audio
            "mp3" | "wav" | "flac" | "aac" | "ogg" | "m4a" | "wma" | "opus" => {
                format!("\x1b[36m{}\x1b[0m", entry.name)
            }
            // Video
            "mp4" | "mov" | "avi" | "mkv" | "webm" | "flv" | "wmv" | "m4v" | "mpg" | "mpeg" => {
                format!("\x1b[1;35m{}\x1b[0m", entry.name)
            }
            _ => entry.name.clone(),
        }
    }
}

/// Colorize permission string.
fn colorize_permissions(perms: &str) -> String {
    let mut result = String::with_capacity(perms.len() * 5);
    for c in perms.chars() {
        match c {
            'd' => result.push_str(&format!("\x1b[1;34m{}\x1b[0m", c)),
            'l' => result.push_str(&format!("\x1b[1;36m{}\x1b[0m", c)),
            'r' => result.push_str(&format!("\x1b[33m{}\x1b[0m", c)),
            'w' => result.push_str(&format!("\x1b[31m{}\x1b[0m", c)),
            'x' | 's' | 't' => result.push_str(&format!("\x1b[1;32m{}\x1b[0m", c)),
            'S' | 'T' => result.push_str(&format!("\x1b[32m{}\x1b[0m", c)),
            '-' => result.push_str(&format!("\x1b[90m{}\x1b[0m", c)),
            _ => result.push(c),
        }
        // Don't add separator
    }
    result
}

/// Get the classify indicator for a file.
pub fn classify_indicator(entry: &FileEntry, full: bool) -> &'static str {
    if entry.is_dir {
        "/"
    } else if entry.is_symlink {
        "@"
    } else if entry.is_pipe {
        "|"
    } else if entry.is_socket {
        "="
    } else if full && entry.is_executable {
        "*"
    } else {
        ""
    }
}

/// Calculate the total blocks for a list of entries (for long-format "total" line).
pub fn total_blocks(entries: &[FileEntry]) -> u64 {
    entries.iter().map(|e| e.blocks).sum::<u64>() / 2 // 512-byte -> 1K blocks
}
