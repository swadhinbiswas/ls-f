#!/usr/bin/env python3
import re
import sys
import os

def get_terminal_width():
    try:
        return os.get_terminal_size().columns
    except OSError:
        return 80

def display_len(s):
    return len(re.sub(r'\x1b\[[0-9;]*m', '', s))

def main():
    items = [line.strip('\n') for line in sys.stdin]
    if not items:
        return

    layout = os.environ.get("LSF_LAYOUT", "columns").lower()
    if layout not in {"rows", "columns"}:
        layout = "columns"

    try:
        tabsize = int(os.environ.get("LSF_TABSIZE", "8"))
    except ValueError:
        tabsize = 8
    if tabsize <= 0:
        tabsize = 8

    width = get_terminal_width()
    if not items:
        return
    max_len = max(display_len(item) for item in items) if items else 0

    if max_len == 0:
        col_width = tabsize
    else:
        col_width = ((max_len + tabsize - 1) // tabsize) * tabsize
        if col_width <= max_len:
            col_width = max_len + tabsize

    cols = max(1, width // col_width)
    rows = (len(items) + cols - 1) // cols

    for i in range(rows):
        for j in range(cols):
            if layout == "rows":
                index = i * cols + j
            else:
                index = i + j * rows

            if index >= len(items):
                continue

            item = items[index]
            d_len = display_len(item)
            padding = ''

            has_next = False
            if layout == "rows":
                if j != cols - 1 and (index + 1) < len(items):
                    has_next = True
            else:
                if j != cols - 1 and (index + rows) < len(items):
                    has_next = True

            if has_next:
                offset = d_len % col_width
                if offset == 0:
                    padding_width = tabsize
                else:
                    padding_width = col_width - offset
                if padding_width < 1:
                    padding_width = tabsize
                padding = ' ' * padding_width

            sys.stdout.write(item + padding)
        sys.stdout.write('\n')

if __name__ == '__main__':
    main()
