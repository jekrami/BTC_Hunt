# Complete Command-Line Options Reference

This document lists ALL available command-line options for btc_hunt.

## Main Program: btc_hunt

### Synopsis

```bash
btc_hunt [OPTIONS]
```

### All Options

| Option | Short | Type | Default | Description |
|--------|-------|------|---------|-------------|
| `--database` | `-d` | Path | `btc_addresses.db` | SQLite database path |
| `--batch-size` | `-b` | Number | `1000` | Mnemonics per batch |
| `--max-batches` | `-n` | Number | `0` | Max batches (0=unlimited) |
| `--addresses-per-path` | `-a` | Number | `10` | Addresses per derivation path |
| `--output-dir` | `-o` | Path | `results` | Output directory for reports |
| `--threads` | `-t` | Number | `0` | Worker threads (0=auto) |
| `--stats-interval` | `-s` | Seconds | `5` | Statistics update interval |
| `--db-batch-size` | | Number | `1000` | Database query batch size |
| `--log-stats` | | Flag | `false` | Enable statistics logging |
| `--email` | | String | None | Email for notifications |
| `--help` | `-h` | Flag | | Show help message |
| `--version` | `-V` | Flag | | Show version |

### Detailed Descriptions

#### `-d, --database <PATH>`
Path to SQLite database containing Bitcoin addresses.
- **Type:** File path
- **Default:** `btc_addresses.db`
- **Example:** `-d /path/to/btc_addresses.db`
- **Required:** Yes (or use default)

Database must have:
- Table named `addresses`
- Column named `address` (TEXT)
- Index recommended: `CREATE INDEX idx_address ON addresses(address);`

#### `-b, --batch-size <NUMBER>`
Number of mnemonics to generate per batch.
- **Type:** Positive integer
- **Default:** `1000`
- **Range:** 1 to unlimited (limited by memory)
- **Example:** `-b 50000`

Larger batches = higher throughput but more memory.
Recommended values:
- Test: `100-1000`
- Production: `5000-10000`
- Low memory: `500`

#### `-n, --max-batches <NUMBER>`
Maximum number of batches to process before stopping.
- **Type:** Non-negative integer
- **Default:** `0` (unlimited)
- **Example:** `-n 100`

Use for:
- Testing: `-n 10` (process 10 batches then stop)
- Limited runs: `-n 1000` (stop after 1000 batches)
- Unlimited: `-n 0` (run forever until match or manual stop)

#### `-a, --addresses-per-path <NUMBER>`
Number of addresses to derive per derivation path.
- **Type:** Positive integer
- **Default:** `10`
- **Range:** 1 to unlimited
- **Example:** `-a 20`

With 6 derivation paths:
- `-a 10` = 60 addresses per mnemonic
- `-a 5` = 30 addresses per mnemonic
- `-a 20` = 120 addresses per mnemonic

Higher values = more coverage but slower.

#### `-o, --output-dir <PATH>`
Directory where match reports are saved.
- **Type:** Directory path
- **Default:** `results`
- **Example:** `-o /path/to/my/results`

Created automatically if doesn't exist.
Match reports named: `MATCH_FOUND_<timestamp>.txt`

#### `-t, --threads <NUMBER>`
Number of worker threads for parallel processing.
- **Type:** Non-negative integer
- **Default:** `0` (auto-detect)
- **Example:** `-t 8`

Values:
- `0` = Auto-detect (recommended)
- `1` = Single-threaded (debugging)
- `4-16` = Manual thread count

Auto-detect uses all available CPU cores.

#### `-s, --stats-interval <SECONDS>`
How often to print statistics to console.
- **Type:** Positive integer (seconds)
- **Default:** `5`
- **Example:** `-s 10`

Values:
- `1` = Every second (very verbose)
- `5` = Every 5 seconds (recommended)
- `60` = Every minute (quiet)

#### `--db-batch-size <NUMBER>`
Number of addresses to check per database query.
- **Type:** Positive integer
- **Default:** `1000`
- **Example:** `--db-batch-size 500`

Larger batches = fewer queries but larger SQL statements.
Usually default is optimal.

#### `--log-stats`
Enable logging of statistics to files.
- **Type:** Boolean flag
- **Default:** `false` (disabled)
- **Example:** `--log-stats`

When enabled:
- Creates `stats/` directory
- Logs statistics every interval to timestamped file
- Format: `stats/btc_hunt_YYYYMMDD_timestamp.log`

Used for daily reports. See `DAILY_REPORTS.md`.

#### `--email <ADDRESS>`
Email address for match notifications.
- **Type:** Email address string
- **Default:** None (disabled)
- **Example:** `--email your@email.com`

When match found:
- **Linux/Unix:** Sends email automatically (requires `mail` command)
- **Windows:** Creates PowerShell script with instructions

See `DAILY_REPORTS.md` for email setup.

#### `-h, --help`
Display help message and exit.
- **Type:** Boolean flag
- **Example:** `--help`

Shows all options with descriptions.

#### `-V, --version`
Display version and exit.
- **Type:** Boolean flag
- **Example:** `--version`

Shows program version number.

## Verification Tool: verify_mnemonic

### Synopsis

```bash
cargo run --bin verify_mnemonic -- "<mnemonic phrase>"
```

### Arguments

| Argument | Type | Required | Description |
|----------|------|----------|-------------|
| Mnemonic | String | Yes | 12-word BIP39 mnemonic phrase |

### Example

```bash
cargo run --release --bin verify_mnemonic -- "word1 word2 word3 word4 word5 word6 word7 word8 word9 word10 word11 word12"
```

See `VERIFICATION.md` for details.

## Daily Report Tool: daily_report

### Synopsis

```bash
cargo run --bin daily_report -- [STATS_DIR] [EMAIL]
```

### Arguments

| Argument | Type | Default | Required | Description |
|----------|------|---------|----------|-------------|
| Stats Dir | Path | `stats` | No | Directory with log files |
| Email | String | None | No | Email for notifications |

### Example

```bash
# Basic (no email)
cargo run --release --bin daily_report

# With stats directory
cargo run --release --bin daily_report -- stats

# With email
cargo run --release --bin daily_report -- stats your@email.com
```

See `DAILY_REPORTS.md` for details.

## Common Command Examples

### Quick Test (100 mnemonics)
```bash
btc_hunt -d btc_addresses.db -b 100 -n 1
```

### Standard Run (1000 per batch, unlimited)
```bash
btc_hunt -d btc_addresses.db -b 1000
```

### Production Run (high performance with logging)
```bash
btc_hunt -d btc_addresses.db -b 5000 -t 0 --log-stats --email your@email.com
```

### Low Memory Run
```bash
btc_hunt -d btc_addresses.db -b 500 -a 5
```

### Maximum Coverage (more addresses per mnemonic)
```bash
btc_hunt -d btc_addresses.db -b 5000 -a 20
```

### Test for 1 Hour (approximate)
```bash
# Depends on your hardware, but roughly:
btc_hunt -d btc_addresses.db -b 10000 -n 360
# 360 batches × 10 seconds per batch ≈ 1 hour
```

### Background Run (Windows)
```powershell
Start-Process powershell -ArgumentList "-Command", "cd D:\MyProjects\BTC_Hunt\btc_hunt; .\target\release\btc_hunt.exe -d btc_addresses.db -b 50000 --log-stats" -NoNewWindow
```

### Background Run (Linux)
```bash
nohup ./target/release/btc_hunt -d btc_addresses.db -b 50000 --log-stats > hunt.log 2>&1 &
```

## Option Combinations

### Testing
```bash
btc_hunt -d btc_addresses.db -b 100 -n 10 -t 1
```
- Small batches
- Limited run (10 batches)
- Single thread (easier debugging)

### Development
```bash
btc_hunt -d btc_addresses.db -b 1000 -n 100 --log-stats -s 1
```
- Medium batches
- Limited run
- Logging enabled
- Frequent stats (every second)

### Production (Maximum Speed)
```bash
btc_hunt -d btc_addresses.db -b 10000 -t 0 --log-stats --email your@email.com
```
- Large batches
- Auto threads
- Logging enabled
- Email alerts

### Production (Balanced)
```bash
btc_hunt -d btc_addresses.db -b 5000 --log-stats --email your@email.com
```
- Medium-large batches
- Auto threads (default)
- Logging enabled
- Email alerts

### Production (Conservative Memory)
```bash
btc_hunt -d btc_addresses.db -b 2000 -a 8 --log-stats --email your@email.com
```
- Smaller batches
- Fewer addresses per path
- Still comprehensive

## Environment Variables

Currently, btc_hunt does not use environment variables.
All configuration is via command-line options.

## Configuration Files

Currently, btc_hunt does not use configuration files.
All configuration is via command-line options.

Future enhancement: Could add `btc_hunt.toml` support.

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success (match found or max batches reached) |
| 1 | Error (invalid arguments, file not found, etc.) |
| 130 | Interrupted by user (Ctrl+C) |

## Output

### Console Output

Statistics printed every interval (default 5 seconds):
```
╔════════════════════════════════════════════════════════════╗
║                    BTC HUNT STATISTICS                     ║
╠════════════════════════════════════════════════════════════╣
║ Runtime:         120 seconds
║ Batches:         120
║ Mnemonics:       120000 (1000/s)
║ Addresses:       7200000 (60000/s)
║ Checked:         7200000
╚════════════════════════════════════════════════════════════╝
```

### File Output

#### Match Reports (results/)
- Created when match found
- Format: `MATCH_FOUND_<timestamp>.txt`
- Contains: mnemonic, seed, addresses, keys, paths

#### Statistics Logs (stats/) - with `--log-stats`
- Created continuously during run
- Format: `btc_hunt_YYYYMMDD_<timestamp>.log`
- Contains: timestamped statistics

#### Daily Reports (reports/) - from daily_report tool
- Created by daily_report tool
- Format: `YYYY-MM-DD/summary.txt`
- Contains: daily summary, performance metrics

## Performance Tuning

### For Maximum Speed
```bash
btc_hunt -d btc_addresses.db -b 10000 -t 0 -a 10
```

### For Maximum Coverage
```bash
btc_hunt -d btc_addresses.db -b 5000 -a 20
```

### For Low Memory
```bash
btc_hunt -d btc_addresses.db -b 500 -a 5 -t 4
```

### For Testing
```bash
btc_hunt -d btc_addresses.db -b 10 -n 1 -t 1
```

## Documentation Reference

- **README.md** - Main documentation
- **QUICK_START.md** - Quick start guide
- **WINDOWS_GUIDE.md** - Windows 11 specific
- **DAILY_REPORTS.md** - Logging and email
- **MIGRATION_GUIDE.md** - From old setup
- **VERIFICATION.md** - Verify addresses
- **FEATURES.md** - Technical details
- **ALL_OPTIONS.md** - This file

## See Also

- `btc_hunt --help` - Built-in help
- `DAILY_REPORTS.md` - For `--log-stats` and `--email` details
- `VERIFICATION.md` - For verify_mnemonic tool
- `WINDOWS_GUIDE.md` - For Windows-specific commands

---

**Last Updated:** 2026-01-04  
**Version:** 2.0.0



