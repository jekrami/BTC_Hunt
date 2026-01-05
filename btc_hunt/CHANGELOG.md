# Changelog

All notable changes to BTC Hunt will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.1.0] - 2026-01-06

### Added
- **Version info at startup**: Program now prints version number on the first line when starting
  - Always visible, even in `--simple` mode
  - Makes it easy to verify which version is running on remote servers
  - Format: `BTC Hunt v2.1.0 starting...`
- **Daily log rotation system**: Logs now rotate automatically at midnight
  - One log file per day with date in filename: `btc_hunt_YYYYMMDD.log`
  - Counters reset automatically at midnight (Runtime, Batches, Mnemonics, Addresses, Checked)
  - Much easier log management and daily reporting
- **Simplified daily reports**: Now just reads the last line of yesterday's log
  - 10x faster processing
  - More accurate daily statistics
- **LogManager struct**: Handles automatic log file rotation seamlessly

### Fixed
- **Daily report cron timing**: Changed from 23:59 (11:59 PM) to 00:05 (12:05 AM)
  - Ensures "yesterday's" date logic is correct
  - Gives 5 minutes after midnight for log rotation to complete
- **Log directory in scripts**: Fixed `--log-dir .` to `--log-dir stats` in all documentation and scripts
- **Daily email reports**: Now sends email ALWAYS (not just when matches found)
  - Includes server hostname
  - Shows exactly what happened that day

### Changed
- Log file naming: From `btc_hunt_{date}_{timestamp}.log` to `btc_hunt_YYYYMMDD.log`
- Daily counters now represent just that day's work (not cumulative)
- Hardcoded version strings now use `env!("CARGO_PKG_VERSION")` for consistency

### Documentation
- Added `LOG_ROTATION_GUIDE.md`: Complete guide to the new log rotation system
- Added `WHATS_NEW_LOG_ROTATION.md`: Summary of log rotation changes
- Added `FIXES_APPLIED.md`: Documentation of bug fixes
- Updated `DAILY_REPORTS_NEW.md`: Reflects new behavior
- Updated `DAILY_REPORT_QUICK_REFERENCE.txt`: Corrected commands and timing
- Added `CHANGELOG.md`: This file!

## [2.0.0] - 2026-01-04

### Added
- **Unified codebase**: Consolidated three separate Rust projects into one
  - `bip39_generator`: BIP39 mnemonic generation
  - `btc-key-deriver`: Bitcoin address derivation
  - `btc_address_checker`: Database search
- **In-memory pipeline**: Eliminated intermediate file I/O
- **Parallel processing**: Using `rayon` for multi-core CPU utilization
- **Batched database queries**: Grouping multiple address checks
- **Real-time statistics**: Live performance metrics
- **Command-line interface**: Using `clap` for argument parsing
- **Email notifications**: Immediate alerts when matches found
- **Simple server mode**: `--simple` flag for clean logging
- **Statistics logging**: `--log-stats` flag for daily reports
- **Daily report generator**: `daily_report` binary for email summaries
- **Mnemonic verification tool**: `verify_mnemonic` binary for debugging
- **Multiple derivation paths**: 
  - m/0'/0' (legacy)
  - m/44'/0'/0' (BIP44)
  - m/49'/0'/0' (BIP49)
  - m/84'/0'/0' (BIP84)
  - m/0' (non-hardened)
- **Address types**: P2PKH, P2WPKH-P2SH (nested SegWit), P2WPKH (native SegWit)

### Performance Improvements
- 10-100x faster than old bash script approach
- Efficient memory usage
- Optimized database queries with read-only mode
- Configurable thread count and batch sizes

### Documentation
- `README.md`: Main documentation
- `WINDOWS_GUIDE.md`: Windows 11 specific guidance
- `VERIFICATION.md`: How to verify address derivation
- `DAILY_REPORTS.md`: Daily reporting setup
- `ALL_OPTIONS.md`: Complete CLI reference
- `SERVER_MODE.md`: Remote server deployment guide
- `DOCUMENTATION_INDEX.md`: Navigation guide

### Scripts
- `setup_daily_report.sh`: Automated cron setup for Linux
- `test_mnemonic.sh/bat`: Test mnemonic verification
- `daily_report.sh/bat`: Manual daily report generation
- `test_with_balance.sh/ps1`: Database testing with sample data

## [1.0.0] - Previous Version (Bash Script)

### Features
- Three separate Rust programs + bash orchestration
- Sequential processing with file I/O between steps
- Basic functionality for BIP39 → addresses → database search

---

## Version Numbering

- **Major** (X.0.0): Breaking changes, major rewrites
- **Minor** (2.X.0): New features, improvements, non-breaking changes
- **Patch** (2.1.X): Bug fixes, documentation updates

[2.1.0]: https://github.com/yourusername/btc_hunt/compare/v2.0.0...v2.1.0
[2.0.0]: https://github.com/yourusername/btc_hunt/releases/tag/v2.0.0

