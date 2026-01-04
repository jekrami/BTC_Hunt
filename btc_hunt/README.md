# BTC Hunt - Unified Bitcoin Address Hunter

A high-performance, unified Rust application that generates BIP39 mnemonics, derives Bitcoin addresses, and searches for matches in a local SQLite database. This consolidates three separate tools into one optimized codebase.

## Features

✨ **Unified Pipeline**: Generate → Derive → Check in a single process  
🚀 **High Performance**: Parallel processing with Rayon, optimized database queries  
💾 **Memory Efficient**: No intermediate file I/O, streaming operations  
📊 **Real-time Statistics**: Live progress updates and performance metrics  
🔍 **Comprehensive Coverage**: Multiple derivation paths (P2PKH, P2WPKH, P2SH-wrapped)  
📝 **Detailed Reports**: Automatic report generation when matches are found  
📧 **Email Notifications**: Instant alerts when matches are found  
📈 **Daily Reports**: Track progress with comprehensive daily summaries  

## Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))
- SQLite database with Bitcoin addresses (table: `btc_addresses`, column: `address`)

## Installation

### Build from Source

```bash
cd btc_hunt
cargo build --release
```

The compiled binary will be at `target/release/btc_hunt` (or `btc_hunt.exe` on Windows).

## Usage

### Basic Usage

```bash
# Run with default settings
./target/release/btc_hunt --database btc_addresses.db

# Process 100 batches of 1000 mnemonics each
./target/release/btc_hunt -d btc_addresses.db -b 1000 -n 100

# Use 8 worker threads
./target/release/btc_hunt -d btc_addresses.db -t 8

# With statistics logging and email notifications
./target/release/btc_hunt -d btc_addresses.db -b 5000 --log-stats --email your@email.com

# Server mode (simple output, good for remote servers and logging)
./target/release/btc_hunt -d btc_addresses.db -b 5000 --simple --log-stats
```

### Command-Line Options

```
Options:
  -d, --database <DATABASE>
          SQLite database path containing Bitcoin addresses
          [default: btc_addresses.db]

  -b, --batch-size <BATCH_SIZE>
          Number of mnemonics to generate per batch
          [default: 1000]

  -n, --max-batches <MAX_BATCHES>
          Number of batches to process (0 = unlimited)
          [default: 0]

  -a, --addresses-per-path <ADDRESSES_PER_PATH>
          Number of addresses to derive per derivation path
          [default: 10]

  -o, --output-dir <OUTPUT_DIR>
          Output directory for results when match is found
          [default: results]

  -t, --threads <THREADS>
          Number of worker threads (0 = auto-detect)
          [default: 0]

  -s, --stats-interval <STATS_INTERVAL>
          Statistics update interval in seconds
          [default: 10]

  --simple
          Simple output mode for servers/logging (no fancy formatting)

  --db-batch-size <DB_BATCH_SIZE>
          Batch size for database queries
          [default: 1000]

  --log-stats
          Log statistics to file for daily reports
          Creates log files in stats/ directory

  --email <EMAIL>
          Email address for match notifications
          Sends email when match is found (requires mail command)

  -h, --help
          Print help

  -V, --version
          Print version
```

## Derivation Paths

The tool checks the following derivation paths for each mnemonic:

| Path Pattern | Type | Script |
|-------------|------|--------|
| `m/0'/0'/{0-9}'` | Legacy | P2PKH |
| `m/44'/0'/0'/0/{0-9}'` | BIP44 | P2PKH |
| `m/49'/0'/0'/0/{0-9}'` | BIP49 | P2WPKH nested in P2SH |
| `m/84'/0'/0'/0/{0-9}'` | BIP84 | P2WPKH (SegWit) |
| `m/0/{0-9}'` | Non-standard | P2WPKH nested in P2SH |
| `m/0/{0-9}'` | Non-standard | P2WPKH |

With default settings (`-a 10`), this generates **60 addresses per mnemonic**.

## Daily Reports and Email Notifications

### Enable Statistics Logging

Track your progress with automatic logging:

```bash
# Windows
.\target\release\btc_hunt.exe -d btc_addresses.db -b 50000 --log-stats

# Linux/Mac
./target/release/btc_hunt -d btc_addresses.db -b 50000 --log-stats
```

This creates log files in the `stats/` directory with statistics logged every 5 seconds.

### Enable Email Notifications

Get instant alerts when matches are found:

```bash
# Windows
.\target\release\btc_hunt.exe -d btc_addresses.db -b 50000 --log-stats --email your@email.com

# Linux/Mac (requires mail command)
./target/release/btc_hunt -d btc_addresses.db -b 50000 --log-stats --email your@email.com
```

**Linux/Unix:** Email is sent automatically using the `mail` command.  
**Windows:** A PowerShell script (`send_notification.ps1`) is created with instructions.

### Generate Daily Reports

Create comprehensive daily summaries:

```bash
# Windows
.\daily_report.bat

# Linux/Mac
./daily_report.sh

# Or manually
cargo run --release --bin daily_report -- stats your@email.com
```

Daily reports include:
- Total mnemonics generated
- Total addresses checked
- Runtime and performance metrics
- Matches found (if any)
- Addresses per second
- Searches per day

Reports are saved to: `reports/YYYY-MM-DD/summary.txt`

### Automate Daily Reports

**Windows Task Scheduler:**
1. Open `taskschd.msc`
2. Create task to run `daily_report.bat` daily at 2 AM

**Linux Cron:**
```bash
crontab -e
# Add: 0 2 * * * cd /path/to/btc_hunt && ./daily_report.sh
```

**For complete documentation on daily reports and email setup, see:**
- `DAILY_REPORTS.md` - Full guide with examples
- `DAILY_REPORTS_QUICK.txt` - Quick reference card

## Performance Optimization

### Recommended Settings

**For Maximum Speed:**
```bash
./target/release/btc_hunt -d btc_addresses.db -b 5000 -t 0 -a 10 --log-stats
```

**For Balanced Performance:**
```bash
./target/release/btc_hunt -d btc_addresses.db -b 1000 -t 0 -a 10 --log-stats
```

**For Lower Memory Usage:**
```bash
./target/release/btc_hunt -d btc_addresses.db -b 500 -t 4 -a 5
```

**Production with Logging & Email:**
```bash
./target/release/btc_hunt -d btc_addresses.db -b 5000 --log-stats --email your@email.com
```

### Performance Tips

1. **Thread Count**: Use `-t 0` to auto-detect optimal thread count
2. **Batch Size**: Larger batches (5000-10000) improve throughput but use more memory
3. **Database**: Ensure your SQLite database has an index on the `address` column:
   ```sql
   CREATE INDEX IF NOT EXISTS idx_address ON addresses(address);
   ```
4. **Storage**: Use an SSD for the database file for faster queries
5. **System**: Close unnecessary applications to free up CPU and RAM

### Expected Performance

On a modern 8-core CPU:
- **Generation**: 5,000-10,000 mnemonics/second
- **Derivation**: 300,000-500,000 addresses/second
- **Checking**: Depends on database size and disk speed

## Output

### Console Statistics

The application displays real-time statistics every 5 seconds (configurable with `-s`):

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

### Match Report

When a match is found, a detailed report is saved to the output directory:

```
results/MATCH_FOUND_1234567890.txt
```

The report includes:
- Timestamp
- BIP39 mnemonic phrase
- BIP39 seed (hex)
- List of found addresses
- Complete CSV of all derived addresses with paths, public keys, and private keys

## Database Setup

Your SQLite database should have the following structure:

```sql
CREATE TABLE btc_addresses (
    address TEXT PRIMARY KEY,
    balance REAL,
    address_type TEXT,
    last_updated INTEGER
);

CREATE INDEX idx_address ON btc_addresses(address);
```

**Note:** btc_hunt only uses the `address` column for searching.

To import addresses from a text file:

```bash
sqlite3 btc_addresses.db
.mode csv
.import addresses.txt btc_addresses
CREATE INDEX idx_address ON btc_addresses(address);
```

## Differences from Original Implementation

### Key Improvements

1. **No File I/O**: All data stays in memory between pipeline stages
2. **Parallel Everything**: Mnemonic generation and address derivation run in parallel
3. **Batch Database Queries**: Checks 1000 addresses per query instead of one at a time
4. **Memory Efficient**: Processes in batches, doesn't hold all results in memory
5. **Early Exit**: Stops immediately when a match is found
6. **Single Binary**: One executable instead of three separate programs
7. **Better Statistics**: Real-time performance monitoring

### Performance Comparison

| Operation | Old Method | New Method | Improvement |
|-----------|-----------|------------|-------------|
| Pipeline | 3 processes + bash | Single process | 3-5x faster |
| I/O | Write/read files | In-memory | 10-100x faster |
| DB Queries | Individual | Batched | 100-1000x faster |
| Parallelism | Sequential | Parallel | 8x faster (8 cores) |

## Project Structure

```
btc_hunt/
├── Cargo.toml              # Dependencies and build configuration
├── README.md               # This file (main documentation)
├── bip39-english.txt       # BIP39 wordlist
├── src/
│   ├── main.rs            # Unified application
│   └── bin/
│       ├── verify_mnemonic.rs  # Verification tool
│       └── daily_report.rs     # Daily report generator
├── stats/                  # Statistics logs (auto-created)
├── reports/                # Daily reports (auto-created)
├── results/                # Match reports (auto-created)
├── QUICK_START.md         # Quick start guide
├── WINDOWS_GUIDE.md       # Windows 11 specific guide
├── DAILY_REPORTS.md       # Daily reports & email guide
├── MIGRATION_GUIDE.md     # Transition from old setup
├── VERIFICATION.md        # How to verify addresses
├── FEATURES.md            # Technical deep-dive
└── target/
    └── release/
        ├── btc_hunt       # Main binary
        ├── verify_mnemonic  # Verification tool
        └── daily_report     # Report generator
```

## Troubleshooting

### "Database locked" Error

If you see database lock errors, ensure no other process is accessing the database. The tool opens the database in read-only mode with `PRAGMA query_only = ON`.

### Slow Performance

1. Check CPU usage with `top` or Task Manager
2. Verify database has an index: `sqlite3 btc_addresses.db "PRAGMA index_list('addresses');"`
3. Reduce batch size if running out of memory: `-b 500`
4. Try different thread counts: `-t 4` or `-t 8`

### Out of Memory

Reduce batch size and addresses per path:
```bash
./target/release/btc_hunt -d btc_addresses.db -b 100 -a 5
```

## Security Notes

- This is for educational and personal use only
- Never share private keys or mnemonic phrases
- The tool is designed for searching your own wallets or research purposes
- Generated mnemonics are cryptographically random using `OsRng`

## Development

### Building in Debug Mode

```bash
cargo build
./target/debug/btc_hunt --help
```

### Running Tests

```bash
cargo test
```

### Code Structure

The application follows a pipeline architecture:

1. **Generate**: Create random BIP39 mnemonics (parallel)
2. **Derive**: Generate addresses from mnemonics (parallel)  
3. **Check**: Query database for matches (batched)
4. **Report**: Generate detailed report on match (sequential)

## License

MIT License - See original project licenses for details.

## Credits

Original separate components by Jafar Ekrami
- `bip39_generator`
- `btc-key-deriver`
- `db_search`

Unified implementation: BTC Hunt v2.0

## Contributing

This is a personal learning project. Feel free to fork and modify for your own use!

## Changelog

### v2.0.0 (2026-01-04)
- ✨ Unified three separate tools into one application
- 🚀 Added parallel processing for all stages
- 💾 Eliminated file I/O between stages
- 📊 Added real-time statistics and monitoring
- 🎯 Optimized database queries with batching
- 📝 Enhanced report generation with timestamps

### v1.x (Previous)
- Separate tools: bip39_generator, btc-key-deriver, db_search
- Bash script orchestration
- File-based communication

