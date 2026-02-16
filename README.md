# touchme

A CLI tool that sets file modification timestamps based on dates found in filenames.

Files named like `IMG_20240315_sunset.jpg` or `report-2023-07-01.pdf` get their mtime set to the detected date.

## Installation

```bash
cargo install --path .
```

## Usage

```bash
# Update timestamps for specific files
touchme IMG_20240315.jpg notes_2023.12.25.txt

# Recurse into directories
touchme -r ~/Photos/

# Preview changes without modifying anything
touchme -n -r ~/Photos/

# Verbose output
touchme -v -r ~/Photos/
```

## Supported date patterns

| Pattern | Example filename |
|---|---|
| `YYYYMMDD` | `IMG_20240315_sunset.jpg` |
| `YYYY-MM-DD` | `report-2023-07-01.pdf` |
| `YYYY.MM.DD` | `notes_2023.12.25.txt` |
| `YYYY_MM_DD` | `backup_2024_01_10.tar.gz` |

**Note**: Any single-character separator between year, month, and day is accepted.

## Options

| Flag | Description |
|---|---|
| `-n` | Dry run — detect dates but don't modify files |
| `-r` | Recurse into directories |
| `-v` | Verbose output (info level) |
| `-d` | Debug output |
| `-t` | Trace output |

## License

MIT
