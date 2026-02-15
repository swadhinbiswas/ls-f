use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(
    name = "lsf",
    about = "A beautiful ls replacement with Nerd Font icons and colors",
    version,
    disable_help_flag = true,
    disable_version_flag = true,
    after_help = "lsf is a fast, native ls replacement written in Rust.\nRequires a Nerd Font installed in your terminal for icons."
)]
pub struct Args {
    /// Files or directories to list
    #[arg(default_value = ".")]
    pub paths: Vec<String>,

    /// Use a long listing format
    #[arg(short = 'l', long = "long")]
    pub long: bool,

    /// List all entries including . and ..
    #[arg(short = 'a', long = "all")]
    pub all: bool,

    /// Do not list implied . and ..
    #[arg(short = 'A', long = "almost-all")]
    pub almost_all: bool,

    /// List subdirectories recursively
    #[arg(short = 'R', long = "recursive")]
    pub recursive: bool,

    /// Disable icons
    #[arg(long = "no-icons")]
    pub no_icons: bool,

    /// Sort by file size, largest first
    #[arg(short = 'S')]
    pub sort_size: bool,

    /// Sort by time, newest first
    #[arg(short = 't')]
    pub sort_time: bool,

    /// Sort alphabetically by extension
    #[arg(short = 'X')]
    pub sort_extension: bool,

    /// Do not sort; list entries in directory order
    #[arg(short = 'U')]
    pub unsorted: bool,

    /// Natural sort of (version) numbers within text
    #[arg(short = 'v')]
    pub version_sort: bool,

    /// Reverse order while sorting
    #[arg(short = 'r', long = "reverse")]
    pub reverse: bool,

    /// List entries by lines instead of by columns
    #[arg(short = 'x')]
    pub across: bool,

    /// List one file per line
    #[arg(short = '1')]
    pub one_per_line: bool,

    /// Fill width with a comma separated list of entries
    #[arg(short = 'm')]
    pub comma: bool,

    /// List directories themselves, not their contents
    #[arg(short = 'd', long = "directory")]
    pub directory: bool,

    /// Append indicator (one of */=>@|) to entries
    #[arg(short = 'F', long = "classify")]
    pub classify: bool,

    /// Append / indicator to directories
    #[arg(short = 'p')]
    pub slash_dirs: bool,

    /// Print the allocated number of blocks for each file
    #[arg(short = 's', long = "size")]
    pub show_size: bool,

    /// Print the inode number of each file
    #[arg(short = 'i', long = "inode")]
    pub inode: bool,

    /// With -l: show numeric user and group IDs
    #[arg(short = 'n', long = "numeric-uid-gid")]
    pub numeric_uid_gid: bool,

    /// Like -l, but list owner
    #[arg(short = 'o')]
    pub long_no_group: bool,

    /// Like -l, but do not list owner
    #[arg(short = 'g')]
    pub long_no_owner: bool,

    /// In a long listing, don't print group names
    #[arg(short = 'G', long = "no-group")]
    pub no_group: bool,

    /// Print sizes in human readable format (e.g., 1K 234M 2G)
    #[arg(short = 'h', long = "human-readable")]
    pub human_readable: bool,

    /// Like -h but use powers of 1000
    #[arg(long = "si")]
    pub si: bool,

    /// Follow symbolic links listed on the command line
    #[arg(short = 'H', long = "dereference-command-line")]
    pub dereference_command_line: bool,

    /// Follow all symbolic links
    #[arg(short = 'L', long = "dereference")]
    pub dereference: bool,

    /// Group directories before files
    #[arg(long = "group-directories-first")]
    pub group_directories_first: bool,

    /// End each output line with NUL, not newline
    #[arg(long = "zero")]
    pub zero: bool,

    /// Assume tab stops at each COLS instead of 8
    #[arg(short = 'T', long = "tabsize", default_value = "8")]
    pub tabsize: usize,

    /// Assume the terminal is COLS columns wide
    #[arg(short = 'w', long = "width")]
    pub term_width: Option<usize>,

    /// Do not list entries matching shell PATTERN
    #[arg(short = 'I', long = "ignore")]
    pub ignore_pattern: Option<String>,

    /// Do not list entries ending with ~
    #[arg(short = 'B', long = "ignore-backups")]
    pub ignore_backups: bool,

    /// With -l, print the author of each file
    #[arg(long = "author")]
    pub author: bool,

    /// Colorize the output [auto, always, never]
    #[arg(long = "color", default_value = "auto", hide_default_value = true)]
    pub color: String,

    /// Do not colorize the output (same as --color=never)
    #[arg(long = "no-color")]
    pub no_color: bool,

    /// Print tree view
    #[arg(long = "tree")]
    pub tree: bool,

    /// Set max depth for tree/recursive (0 = unlimited)
    #[arg(long = "depth")]
    pub max_depth: Option<usize>,

    /// Show git status for files (requires git)
    #[arg(long = "git")]
    pub git: bool,

    /// Print help information
    #[arg(long = "help")]
    pub help: bool,

    /// Print version information
    #[arg(short = 'V', long = "version")]
    pub print_version: bool,

    /// Print C-style escapes for nongraphic characters
    #[arg(short = 'b', long = "escape")]
    pub escape: bool,

    /// Do not sort; list entries in directory order (same as -U)
    #[arg(short = 'f')]
    pub no_sort_all: bool,

    /// Emit SELinux / security context
    #[arg(short = 'Z', long = "context")]
    pub context: bool,

    /// Show allocated blocks in -s with this scale
    #[arg(long = "block-size")]
    pub block_size: Option<String>,

    /// Hide entries matching pattern (overridden by -a or -A)
    #[arg(long = "hide")]
    pub hide_pattern: Option<String>,
}

impl Args {
    /// Whether to show hidden files
    pub fn show_hidden(&self) -> bool {
        self.all || self.almost_all || self.no_sort_all
    }

    /// Whether to show . and ..
    pub fn show_dot_dirs(&self) -> bool {
        self.all || self.no_sort_all
    }

    /// Whether any long-format flag is set
    pub fn is_long(&self) -> bool {
        self.long || self.long_no_group || self.long_no_owner || self.numeric_uid_gid
    }

    /// Whether to use color output
    pub fn use_color(&self) -> bool {
        if self.no_color {
            return false;
        }
        match self.color.as_str() {
            "always" | "yes" | "force" => true,
            "never" | "no" | "none" => false,
            _ => {
                // "auto" - check if stdout is a tty
                atty_check()
            }
        }
    }

    /// Whether sorting is disabled
    pub fn no_sort(&self) -> bool {
        self.unsorted || self.no_sort_all
    }
}

fn atty_check() -> bool {
    unsafe { libc::isatty(libc::STDOUT_FILENO) != 0 }
}
