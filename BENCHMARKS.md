# Benchmarks: lsf vs GNU ls

**lsf v5.0.0** (Rust rewrite) vs **GNU coreutils ls 9.10**

Benchmarked with [hyperfine](https://github.com/sharkdp/hyperfine) using `--shell=none` (`-N`) for accurate sub-millisecond measurement. All tests run with `--color=always` to enable full ANSI color output on both tools.

**System:** Linux x86-64

---

## Results Summary

| Scenario | GNU ls | lsf | Ratio | Winner |
|---|---|---|---|---|
| Small dir, grid (7 files) | 0.75 ms | 0.65 ms | **0.87x** | lsf |
| Large dir, grid (7300 files) | 10.2 ms | 12.9 ms | **1.27x** | GNU ls |
| Long format, small dir | 1.7 ms | 1.8 ms | **1.06x** | ~tied |
| Long format, large dir (7300 files) | 18.4 ms | 18.3 ms | **1.00x** | ~tied |
| Single file (`-l Cargo.toml`) | 1.8 ms | 1.7 ms | **0.94x** | lsf |
| Recursive (`-R /usr/share/doc/`) | 17.1 ms | 27.1 ms | **1.59x** | GNU ls |

> **lsf adds Nerd Font icons and colored icons to every entry** -- GNU ls only adds filename colors. Despite this extra work, lsf matches or beats GNU ls in most scenarios.

---

## Detailed Results

### 1. Small Directory - Grid View

Listing `src/` (7 Rust source files):

```
Benchmark 1: /sbin/ls --color=always src/
  Time (mean +/- s):     745.8 us +/- 465.4 us
  Range (min ... max):   351.0 us ... 3375.1 us

Benchmark 2: ./target/release/lsf --color=always src/
  Time (mean +/- s):     649.2 us +/- 329.1 us
  Range (min ... max):   399.0 us ... 2437.5 us

Summary: lsf ran 1.15x faster than GNU ls
```

### 2. Large Directory - Grid View

Listing `/tmp/lsf-bench/` (7300 files: 5000 .txt, 500 dirs, 1800 multi-extension):

```
Benchmark 1: /sbin/ls --color=always /tmp/lsf-bench/
  Time (mean +/- s):     10.2 ms +/- 3.6 ms
  Range (min ... max):    5.7 ms ... 22.8 ms

Benchmark 2: ./target/release/lsf --color=always /tmp/lsf-bench/
  Time (mean +/- s):     12.9 ms +/- 3.2 ms
  Range (min ... max):    8.5 ms ... 21.5 ms

Summary: GNU ls ran 1.27x faster than lsf
```

### 3. Long Format - Small Directory

`-lah` on `src/` (7 files):

```
Benchmark 1: /sbin/ls -lah --color=always src/
  Time (mean +/- s):     1.7 ms +/- 1.0 ms

Benchmark 2: ./target/release/lsf -lah --color=always src/
  Time (mean +/- s):     1.8 ms +/- 1.1 ms

Summary: ~tied (1.06x)
```

### 4. Long Format - Large Directory

`-lah` on 7300 files:

```
Benchmark 1: /sbin/ls -lah --color=always /tmp/lsf-bench/
  Time (mean +/- s):     18.4 ms +/- 3.2 ms

Benchmark 2: ./target/release/lsf -lah --color=always /tmp/lsf-bench/
  Time (mean +/- s):     18.3 ms +/- 3.6 ms

Summary: lsf ran 1.01x faster (effectively tied)
```

### 5. Single File

`-l Cargo.toml`:

```
Benchmark 1: /sbin/ls -l --color=always Cargo.toml
  Time (mean +/- s):     1.8 ms +/- 1.0 ms

Benchmark 2: ./target/release/lsf -l --color=always Cargo.toml
  Time (mean +/- s):     1.7 ms +/- 1.0 ms

Summary: lsf ran 1.09x faster
```

### 6. Recursive Listing

`-R /usr/share/doc/`:

```
Benchmark 1: /sbin/ls -R --color=always /usr/share/doc/
  Time (mean +/- s):     17.1 ms +/- 3.1 ms

Benchmark 2: ./target/release/lsf -R --color=always /usr/share/doc/
  Time (mean +/- s):     27.1 ms +/- 2.1 ms

Summary: GNU ls ran 1.59x faster
```

---

## lsf-Only Features

### 7. Tree View

`--tree --depth 3` on `/usr/share/doc/`:

```
Benchmark: ./target/release/lsf --tree --depth 3 --color=always /usr/share/doc/
  Time (mean +/- s):     17.2 ms +/- 3.0 ms
```

> Tree view has no GNU ls equivalent. lsf renders a full tree in ~17ms.

### 8. Icon Overhead

lsf with icons vs without on 7300 files:

```
With icons:    13.2 ms +/- 3.2 ms
Without icons: 13.3 ms +/- 3.4 ms

Summary: icon lookup adds ~0% overhead (effectively zero)
```

> The icon lookup system is so fast that enabling/disabling icons makes no measurable difference.

---

## Binary Size

| | Size |
|---|---|
| lsf binary (stripped, LTO) | 935 KB |
| GNU ls binary | ~140 KB |

lsf is a single static binary with zero runtime dependencies (no Bash, no Python, no GNU coreutils needed).

---

## How to Reproduce

```bash
# Install hyperfine
cargo install hyperfine

# Create test directory (7300 files)
mkdir -p /tmp/lsf-bench
for i in $(seq 1 5000); do touch "/tmp/lsf-bench/file_$i.txt"; done
for i in $(seq 1 500); do mkdir -p "/tmp/lsf-bench/dir_$i"; done
for ext in rs py js ts go c cpp h hpp java rb php css html json yaml toml xml md sh; do
  for i in $(seq 1 100); do touch "/tmp/lsf-bench/sample_${i}.${ext}"; done
done

# Build release binary
cargo build --release

# Run benchmarks
hyperfine --warmup 10 --min-runs 1000 -N \
  '/sbin/ls --color=always src/' \
  './target/release/lsf --color=always src/'

hyperfine --warmup 5 --min-runs 100 -N \
  '/sbin/ls --color=always /tmp/lsf-bench/' \
  './target/release/lsf --color=always /tmp/lsf-bench/'

hyperfine --warmup 10 --min-runs 1000 -N \
  '/sbin/ls -lah --color=always src/' \
  './target/release/lsf -lah --color=always src/'

hyperfine --warmup 5 --min-runs 100 -N \
  '/sbin/ls -lah --color=always /tmp/lsf-bench/' \
  './target/release/lsf -lah --color=always /tmp/lsf-bench/'

hyperfine --warmup 2 --min-runs 10 -N \
  '/sbin/ls -R --color=always /usr/share/doc/' \
  './target/release/lsf -R --color=always /usr/share/doc/'
```
