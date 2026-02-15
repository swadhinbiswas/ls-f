mod cli;
mod entry;
mod format;
mod grid;
mod icons;
mod output;

use std::io::{self, BufWriter, Write};
use std::path::Path;
use std::process;

use clap::Parser;

fn main() {
    let args = cli::Args::parse();

    // Handle custom help/version flags
    if args.help {
        use clap::CommandFactory;
        let mut cmd = cli::Args::command();
        cmd.print_help().ok();
        println!();
        return;
    }
    if args.print_version {
        println!("lsf {}", env!("CARGO_PKG_VERSION"));
        return;
    }

    // Handle tree mode separately
    if args.tree {
        let stdout = io::stdout();
        let mut out = BufWriter::new(stdout.lock());
        let icon_map = if args.no_icons {
            std::collections::HashMap::new()
        } else {
            icons::build_icon_map()
        };
        let use_color = args.use_color();
        let show_icons = !args.no_icons;

        for path_str in &args.paths {
            let path = Path::new(path_str);
            if !path.exists() {
                eprintln!(
                    "lsf: cannot access '{}': No such file or directory",
                    path_str
                );
                continue;
            }
            if let Err(e) = output::print_tree(
                &mut out, path, &args, &icon_map, use_color, show_icons, "", true, 0,
            ) {
                eprintln!("lsf: {}: {}", path_str, e);
            }
        }

        if let Err(e) = out.flush() {
            if e.kind() != io::ErrorKind::BrokenPipe {
                eprintln!("lsf: write error: {}", e);
            }
        }
        return;
    }

    // Normal listing
    if let Err(e) = output::run(&args) {
        if e.kind() == io::ErrorKind::BrokenPipe {
            // Silently exit on broken pipe (e.g., piping to `head`)
            process::exit(0);
        }
        eprintln!("lsf: {}", e);
        process::exit(1);
    }
}
