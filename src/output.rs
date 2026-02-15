use std::collections::HashMap;
use std::io::{self, BufWriter, Write};
use std::path::Path;

use crate::cli::Args;
use crate::entry::{self, FileEntry};
use crate::format;
use crate::grid;
use crate::icons::{self, IconEntry};

/// Main output driver. Takes parsed args and runs the listing.
pub fn run(args: &Args) -> io::Result<()> {
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let icon_map = if args.no_icons {
        HashMap::new()
    } else {
        icons::build_icon_map()
    };

    let use_color = args.use_color();
    let show_icons = !args.no_icons;
    let term_width = grid::get_terminal_width(args.term_width);
    let terminator = if args.zero { '\0' } else { '\n' };

    let paths: Vec<&str> = args.paths.iter().map(|s| s.as_str()).collect();
    let multi = paths.len() > 1;

    // Separate files and directories
    let mut file_entries: Vec<FileEntry> = Vec::new();
    let mut dir_paths: Vec<&str> = Vec::new();

    for path_str in &paths {
        let path = Path::new(path_str);
        if !path.exists() && !path.is_symlink() {
            eprintln!(
                "lsf: cannot access '{}': No such file or directory",
                path_str
            );
            continue;
        }

        if path.is_file() || path.is_symlink() && !path.is_dir() || args.directory {
            let follow = args.dereference || args.dereference_command_line;
            if let Ok(fe) = FileEntry::from_path(path, follow) {
                file_entries.push(fe);
            }
        } else {
            dir_paths.push(path_str);
        }
    }

    // Print file arguments first
    if !file_entries.is_empty() {
        entry::sort_entries(&mut file_entries, args);
        print_entries(
            &mut out,
            &file_entries,
            args,
            &icon_map,
            use_color,
            show_icons,
            term_width,
            terminator,
        )?;
        if !dir_paths.is_empty() {
            write!(out, "\n")?;
        }
    }

    // Print directories
    let show_header = multi || !file_entries.is_empty() || args.recursive;

    for (idx, path_str) in dir_paths.iter().enumerate() {
        let path = Path::new(path_str);

        if show_header {
            if idx > 0 || !file_entries.is_empty() {
                write!(out, "\n")?;
            }
            if use_color {
                write!(out, "\x1b[1;34m{}:\x1b[0m\n", path_str)?;
            } else {
                write!(out, "{}:\n", path_str)?;
            }
        }

        match entry::read_directory(path, args) {
            Ok(mut entries) => {
                entry::sort_entries(&mut entries, args);

                if args.is_long() {
                    // Print total line
                    let total = format::total_blocks(&entries);
                    writeln!(out, "total {}", total)?;
                }

                print_entries(
                    &mut out, &entries, args, &icon_map, use_color, show_icons, term_width,
                    terminator,
                )?;

                // Recursive subdirectories
                if args.recursive {
                    let mut subdirs: Vec<FileEntry> = entries
                        .iter()
                        .filter(|e| e.is_dir && e.name != "." && e.name != "..")
                        .cloned()
                        .collect();
                    subdirs.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

                    if args.reverse {
                        subdirs.reverse();
                    }

                    for subdir in subdirs {
                        let subpath = subdir.path.to_string_lossy().to_string();

                        // Check depth
                        if let Some(max_depth) = args.max_depth {
                            let base = Path::new(path_str);
                            let sub = Path::new(&subpath);
                            let depth = sub
                                .strip_prefix(base)
                                .map(|p| p.components().count())
                                .unwrap_or(0);
                            if depth > max_depth {
                                continue;
                            }
                        }

                        print_recursive(
                            &mut out, &subpath, args, &icon_map, use_color, show_icons, term_width,
                            terminator, path_str,
                        )?;
                    }
                }
            }
            Err(e) => {
                eprintln!("lsf: cannot open directory '{}': {}", path_str, e);
            }
        }
    }

    // If we only had a single directory without header
    if !show_header && dir_paths.len() == 1 {
        // Already printed above, but without header
    }

    out.flush()?;
    Ok(())
}

/// Print entries for a single directory (not recursive, just the entries).
fn print_entries(
    out: &mut impl Write,
    entries: &[FileEntry],
    args: &Args,
    icon_map: &HashMap<&str, IconEntry>,
    use_color: bool,
    show_icons: bool,
    term_width: usize,
    terminator: char,
) -> io::Result<()> {
    if entries.is_empty() {
        return Ok(());
    }

    if args.is_long() {
        print_long(out, entries, args, icon_map, use_color, show_icons)?;
    } else if args.one_per_line {
        let items = format_items(entries, args, icon_map, use_color, show_icons);
        let output = grid::format_single_column(&items, terminator);
        write!(out, "{}", output)?;
    } else if args.comma {
        let items = format_items(entries, args, icon_map, use_color, show_icons);
        let output = grid::format_comma(&items, term_width);
        write!(out, "{}", output)?;
    } else {
        // Grid view
        let items = format_items(entries, args, icon_map, use_color, show_icons);
        if args.zero {
            let output = grid::format_single_column(&items, '\0');
            write!(out, "{}", output)?;
        } else {
            let output = grid::format_grid(&items, term_width, args.tabsize, args.across);
            write!(out, "{}", output)?;
        }
    }

    Ok(())
}

/// Format items for grid / one-per-line / comma views.
fn format_items(
    entries: &[FileEntry],
    args: &Args,
    icon_map: &HashMap<&str, IconEntry>,
    use_color: bool,
    show_icons: bool,
) -> Vec<String> {
    entries
        .iter()
        .map(|entry| {
            let mut item = String::with_capacity(64);

            // Inode
            if args.inode {
                item.push_str(&format!("{} ", entry.inode));
            }

            // Blocks
            if args.show_size {
                let blocks = entry.blocks / 2;
                item.push_str(&format!("{} ", blocks));
            }

            // Icon
            if show_icons {
                let icon = icons::get_icon(
                    icon_map,
                    &entry.name,
                    entry.is_dir,
                    entry.is_hidden,
                    entry.is_symlink,
                    entry.is_executable,
                    entry.is_pipe,
                    entry.is_socket,
                    entry.is_block_device,
                    entry.is_char_device,
                );
                if use_color {
                    item.push_str(&icon.colored());
                } else {
                    item.push_str(&icon.plain());
                }
                item.push(' ');
            }

            // Filename (with color)
            item.push_str(&format::colorize_filename(entry, use_color));

            // Classify indicator
            if args.classify {
                item.push_str(format::classify_indicator(entry, true));
            } else if args.slash_dirs && entry.is_dir {
                item.push('/');
            }

            // Symlink target in non-long modes
            if entry.is_symlink && args.one_per_line {
                if let Some(ref target) = entry.symlink_target {
                    item.push_str(" -> ");
                    if use_color {
                        item.push_str(&format!("\x1b[36m{}\x1b[0m", target));
                    } else {
                        item.push_str(target);
                    }
                }
            }

            item
        })
        .collect()
}

/// Print entries in long format.
fn print_long(
    out: &mut impl Write,
    entries: &[FileEntry],
    args: &Args,
    icon_map: &HashMap<&str, IconEntry>,
    use_color: bool,
    show_icons: bool,
) -> io::Result<()> {
    let show_owner = !args.long_no_owner;
    let show_group = !args.long_no_group && !args.no_group;
    let show_inode = args.inode;
    let show_blocks = args.show_size;
    let show_author = args.author;
    let human_readable = args.human_readable;
    let si = args.si;
    let numeric_ids = args.numeric_uid_gid;

    let mut user_cache: HashMap<u32, String> = HashMap::new();
    let mut group_cache: HashMap<u32, String> = HashMap::new();

    // Pre-calculate column widths for alignment
    let nlink_width = entries
        .iter()
        .map(|e| e.nlink.to_string().len())
        .max()
        .unwrap_or(1);

    let owner_width = if show_owner {
        entries
            .iter()
            .map(|e| {
                if numeric_ids {
                    e.uid.to_string().len()
                } else {
                    entry::get_username(e.uid, &mut user_cache).len()
                }
            })
            .max()
            .unwrap_or(1)
    } else {
        0
    };

    let group_width = if show_group {
        entries
            .iter()
            .map(|e| {
                if numeric_ids {
                    e.gid.to_string().len()
                } else {
                    entry::get_groupname(e.gid, &mut group_cache).len()
                }
            })
            .max()
            .unwrap_or(1)
    } else {
        0
    };

    let size_width = entries
        .iter()
        .map(|e| format::format_size(e.size, human_readable, si).len())
        .max()
        .unwrap_or(1);

    let inode_width = if show_inode {
        entries
            .iter()
            .map(|e| e.inode.to_string().len())
            .max()
            .unwrap_or(1)
    } else {
        0
    };

    let blocks_width = if show_blocks {
        entries
            .iter()
            .map(|e| (e.blocks / 2).to_string().len())
            .max()
            .unwrap_or(1)
    } else {
        0
    };

    for entry in entries {
        let icon_str = if show_icons {
            let icon = icons::get_icon(
                icon_map,
                &entry.name,
                entry.is_dir,
                entry.is_hidden,
                entry.is_symlink,
                entry.is_executable,
                entry.is_pipe,
                entry.is_socket,
                entry.is_block_device,
                entry.is_char_device,
            );
            if use_color {
                icon.colored()
            } else {
                icon.plain()
            }
        } else {
            String::new()
        };

        let mut line = format::format_long_entry(
            entry,
            &icon_str,
            show_icons,
            show_owner,
            show_group,
            show_inode,
            show_blocks,
            show_author,
            human_readable,
            si,
            numeric_ids,
            use_color,
            &mut user_cache,
            &mut group_cache,
            nlink_width,
            owner_width,
            group_width,
            size_width,
            inode_width,
            blocks_width,
        );

        // Classify indicator
        if args.classify {
            line.push_str(format::classify_indicator(entry, true));
        } else if args.slash_dirs && entry.is_dir {
            line.push('/');
        }

        writeln!(out, "{}", line)?;
    }

    Ok(())
}

/// Recursively print a subdirectory.
fn print_recursive(
    out: &mut impl Write,
    dir_path: &str,
    args: &Args,
    icon_map: &HashMap<&str, IconEntry>,
    use_color: bool,
    show_icons: bool,
    term_width: usize,
    terminator: char,
    base_path: &str,
) -> io::Result<()> {
    let path = Path::new(dir_path);

    write!(out, "\n")?;
    if use_color {
        write!(out, "\x1b[1;34m{}:\x1b[0m\n", dir_path)?;
    } else {
        write!(out, "{}:\n", dir_path)?;
    }

    match entry::read_directory(path, args) {
        Ok(mut entries) => {
            entry::sort_entries(&mut entries, args);

            if args.is_long() {
                let total = format::total_blocks(&entries);
                writeln!(out, "total {}", total)?;
            }

            print_entries(
                out, &entries, args, icon_map, use_color, show_icons, term_width, terminator,
            )?;

            // Recurse into subdirectories
            let mut subdirs: Vec<FileEntry> = entries
                .iter()
                .filter(|e| e.is_dir && e.name != "." && e.name != "..")
                .cloned()
                .collect();
            subdirs.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

            if args.reverse {
                subdirs.reverse();
            }

            for subdir in subdirs {
                let subpath = subdir.path.to_string_lossy().to_string();

                // Check depth
                if let Some(max_depth) = args.max_depth {
                    let base = Path::new(base_path);
                    let sub = Path::new(&subpath);
                    let depth = sub
                        .strip_prefix(base)
                        .map(|p| p.components().count())
                        .unwrap_or(0);
                    if depth > max_depth {
                        continue;
                    }
                }

                print_recursive(
                    out, &subpath, args, icon_map, use_color, show_icons, term_width, terminator,
                    base_path,
                )?;
            }
        }
        Err(e) => {
            eprintln!("lsf: cannot open directory '{}': {}", dir_path, e);
        }
    }

    Ok(())
}

/// Print entries in tree view.
pub fn print_tree(
    out: &mut impl Write,
    path: &Path,
    args: &Args,
    icon_map: &HashMap<&str, IconEntry>,
    use_color: bool,
    show_icons: bool,
    prefix: &str,
    is_last: bool,
    depth: usize,
) -> io::Result<()> {
    let follow = args.dereference;
    let entry = FileEntry::from_path(path, follow)?;

    // Print this entry
    let connector = if depth == 0 {
        ""
    } else if is_last {
        "└── "
    } else {
        "├── "
    };

    let icon_str = if show_icons {
        let icon = icons::get_icon(
            icon_map,
            &entry.name,
            entry.is_dir,
            entry.is_hidden,
            entry.is_symlink,
            entry.is_executable,
            entry.is_pipe,
            entry.is_socket,
            entry.is_block_device,
            entry.is_char_device,
        );
        if use_color {
            format!("{} ", icon.colored())
        } else {
            format!("{} ", icon.plain())
        }
    } else {
        String::new()
    };

    let colored_name = format::colorize_filename(&entry, use_color);
    writeln!(out, "{}{}{}{}", prefix, connector, icon_str, colored_name)?;

    // Recurse into directories
    if entry.is_dir {
        if let Some(max_depth) = args.max_depth {
            if depth >= max_depth {
                return Ok(());
            }
        }

        if let Ok(mut children) = entry::read_directory(path, args) {
            entry::sort_entries(&mut children, args);

            let child_prefix = if depth == 0 {
                String::new()
            } else {
                format!("{}{}", prefix, if is_last { "    " } else { "│   " })
            };

            let len = children.len();
            for (i, child) in children.iter().enumerate() {
                if child.name == "." || child.name == ".." {
                    continue;
                }
                let child_is_last = i == len - 1;
                print_tree(
                    out,
                    &child.path,
                    args,
                    icon_map,
                    use_color,
                    show_icons,
                    &child_prefix,
                    child_is_last,
                    depth + 1,
                )?;
            }
        }
    }

    Ok(())
}
