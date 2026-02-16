# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

`touchme` is a CLI tool that sets file modification timestamps based on dates detected in filenames. It matches patterns like `YYYYMMDD` or `YYYY-MM-DD` (with any separator) in filenames and updates the file's mtime accordingly. Supports recursive directory traversal and dry-run mode.

## Build & Run

```bash
cargo build                # debug build
cargo build --release      # release build (fat LTO, opt-level 3)
cargo run -- [args]        # run with arguments
cargo fmt                  # format code
cargo clippy               # lint
```

No tests exist in this project.

## Code Structure

- `build.rs` — uses the `build-data` crate to embed git branch, commit, source timestamp, and rustc version as compile-time env vars
- `src/lib.rs` — re-exports std and dependency types as a prelude (std fs/path/time, anyhow, chrono, clap, regex, tracing)
- `src/config.rs` — `OptsCommon` struct (clap derive) with CLI flags (`-v`, `-d`, `-t`, `-n`, `-r`) and tracing initialization via `start_pgm()`
- `src/bin/touchme.rs` — main binary: regex-based date detection from filenames, recursive directory walking, and timestamp updating

## Conventions

- Rust edition 2024, stable toolchain
- rustfmt: 120 max width, crate-level import granularity, std/external/crate import grouping
- All source files end with `// EOF` comment
- Prelude pattern: `lib.rs` re-exports everything, modules use `use crate::*`