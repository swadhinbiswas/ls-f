use unicode_width::UnicodeWidthStr;

/// Calculate the display width of a string, ignoring ANSI escape sequences.
pub fn display_width(s: &str) -> usize {
    let stripped = strip_ansi(s);
    UnicodeWidthStr::width(stripped.as_str())
}

/// Strip ANSI escape sequences from a string.
pub fn strip_ansi(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut in_escape = false;
    let mut chars = s.chars();

    while let Some(c) = chars.next() {
        if c == '\x1b' {
            in_escape = true;
            continue;
        }
        if in_escape {
            if c == 'm' {
                in_escape = false;
            }
            continue;
        }
        result.push(c);
    }

    result
}

/// Format entries into a multi-column grid.
/// `items` - pre-formatted strings (with ANSI colors).
/// `term_width` - terminal width in columns.
/// `tabsize` - tab stop width.
/// `by_rows` - if true, fill rows first (like `ls -x`); if false, fill columns first.
pub fn format_grid(items: &[String], term_width: usize, tabsize: usize, by_rows: bool) -> String {
    if items.is_empty() {
        return String::new();
    }

    let tabsize = if tabsize == 0 { 8 } else { tabsize };

    // Find the maximum display width
    let max_len = items.iter().map(|s| display_width(s)).max().unwrap_or(0);

    // Calculate column width (align to tab stops)
    let col_width = if max_len == 0 {
        tabsize
    } else {
        let w = ((max_len + tabsize - 1) / tabsize) * tabsize;
        if w <= max_len {
            max_len + tabsize
        } else {
            w
        }
    };

    let cols = (term_width / col_width).max(1);
    let rows = (items.len() + cols - 1) / cols;

    let mut output = String::with_capacity(items.len() * (col_width + 2));

    for i in 0..rows {
        for j in 0..cols {
            let index = if by_rows { i * cols + j } else { i + j * rows };

            if index >= items.len() {
                continue;
            }

            let item = &items[index];
            let d_len = display_width(item);

            // Determine if there's a next item in this row
            let has_next = if by_rows {
                j != cols - 1 && (index + 1) < items.len()
            } else {
                j != cols - 1 && (index + rows) < items.len()
            };

            output.push_str(item);

            if has_next {
                let offset = d_len % col_width;
                let padding = if offset == 0 {
                    tabsize
                } else {
                    let p = col_width - offset;
                    if p < 1 {
                        tabsize
                    } else {
                        p
                    }
                };
                for _ in 0..padding {
                    output.push(' ');
                }
            }
        }
        output.push('\n');
    }

    output
}

/// Format entries as one-per-line.
pub fn format_single_column(items: &[String], terminator: char) -> String {
    let mut output = String::with_capacity(items.len() * 40);
    for item in items {
        output.push_str(item);
        output.push(terminator);
    }
    output
}

/// Format entries as comma-separated.
pub fn format_comma(items: &[String], term_width: usize) -> String {
    if items.is_empty() {
        return String::new();
    }

    let mut output = String::with_capacity(items.len() * 20);
    let mut line_len: usize = 0;

    for (i, item) in items.iter().enumerate() {
        let d_len = display_width(item);
        let separator = if i < items.len() - 1 { ", " } else { "" };
        let needed = d_len + separator.len();

        if line_len > 0 && line_len + needed > term_width {
            output.push('\n');
            line_len = 0;
        }

        output.push_str(item);
        output.push_str(separator);
        line_len += needed;
    }

    if !output.ends_with('\n') {
        output.push('\n');
    }

    output
}

/// Get terminal width, with optional override.
pub fn get_terminal_width(override_width: Option<usize>) -> usize {
    if let Some(w) = override_width {
        return w;
    }
    terminal_size::terminal_size()
        .map(|(w, _)| w.0 as usize)
        .unwrap_or(80)
}
