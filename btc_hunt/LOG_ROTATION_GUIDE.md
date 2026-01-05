# BTC Hunt - Daily Log Rotation System

## Overview

The logging system now uses **daily log rotation** with automatic counter resets. This makes log management simpler and daily reports more accurate.

## How It Works

### 1. Daily Log Files

Log files are created with the date in the filename:
```
stats/btc_hunt_20260104.log
stats/btc_hunt_20260105.log
stats/btc_hunt_20260106.log
```

Each day gets its own log file.

### 2. Automatic Rotation at Midnight

When the clock hits midnight:
- ✅ Current log file is automatically closed
- ✅ New log file is created with today's date
- ✅ All counters reset to 0 (Runtime, Batches, Mnemonics, Addresses, Checked)
- ✅ Process continues seamlessly without interruption

### 3. Log Format

Each line in the log file contains cumulative stats for that day:
```
[2026-01-04 17:17:14] Runtime: 10 | Batches: 0 | Mnemonics: 10000 | Addresses: 0 | Checked: 0
[2026-01-04 17:17:24] Runtime: 20 | Batches: 0 | Mnemonics: 10000 | Addresses: 600000 | Checked: 0
[2026-01-04 17:17:34] Runtime: 30 | Batches: 1 | Mnemonics: 20000 | Addresses: 600000 | Checked: 600000
```

The **LAST line** contains the final totals for that day.

### 4. Daily Report Reading

The `daily_report` tool:
1. Finds yesterday's log file: `btc_hunt_YYYYMMDD.log`
2. Reads **only the LAST line** (final daily totals)
3. Formats the data
4. Sends email with summary

**This is much simpler and faster!**

## Benefits

✅ **Smaller log files** - One file per day, not one giant growing file
✅ **Easier management** - Archive or delete old logs easily
✅ **Accurate daily stats** - Counters represent just that day's work
✅ **Simpler parsing** - Daily report just reads last line
✅ **No confusion** - Each day's stats are isolated
✅ **Automatic cleanup** - Can easily delete logs older than X days

## Usage

### Running btc_hunt with Logging

```bash
./btc_hunt -d btc_addresses.db -b 10000 -t 0 --simple --log-stats --email your@email.com
```

This will:
- Create `stats/btc_hunt_YYYYMMDD.log` for today
- Write stats every 10 seconds
- Automatically rotate at midnight
- Reset counters when new day starts

### Running Daily Report

```bash
./daily_report --log-dir stats --email your@email.com
```

This will:
- Look for `stats/btc_hunt_YYYYMMDD.log` (yesterday's date)
- Read the last line to get final totals
- Generate report
- Send email

### Cron Setup (Run Daily at 00:05 AM)

```bash
5 0 * * * cd /path/to/btc_hunt && ./target/release/daily_report --log-dir stats --email your@email.com >> daily_report.log 2>&1
```

**Note:** Runs at 00:05 AM (5 minutes after midnight) to ensure yesterday's log file is complete.

## Example Log File Lifecycle

### Day 1 (2026-01-04)
```
File: btc_hunt_20260104.log

[2026-01-04 00:01:14] Runtime: 10 | Batches: 0 | Mnemonics: 10000 | Addresses: 0 | Checked: 0
[2026-01-04 00:02:24] Runtime: 20 | Batches: 1 | Mnemonics: 20000 | Addresses: 600000 | Checked: 600000
...
[2026-01-04 23:59:44] Runtime: 86340 | Batches: 4317 | Mnemonics: 43170000 | Addresses: 2590200000 | Checked: 2590200000
```

### Midnight Rotation

At `2026-01-05 00:00:00`:
- Log file closed
- Counters reset to 0
- New file created

### Day 2 (2026-01-05)
```
File: btc_hunt_20260105.log

[2026-01-05 00:00:14] Runtime: 10 | Batches: 0 | Mnemonics: 10000 | Addresses: 0 | Checked: 0
[2026-01-05 00:01:24] Runtime: 20 | Batches: 1 | Mnemonics: 20000 | Addresses: 600000 | Checked: 600000
...
```

### Daily Report (Run on 2026-01-05)

```bash
./daily_report --log-dir stats --email you@email.com
```

Reads: `btc_hunt_20260104.log`
Gets last line: `Runtime: 86340 | Batches: 4317 | ...`
Sends email with Day 1 summary.

## Log Management

### View Today's Stats

```bash
tail -f stats/btc_hunt_$(date +%Y%m%d).log
```

### View Yesterday's Final Stats

```bash
tail -1 stats/btc_hunt_$(date -d yesterday +%Y%m%d).log
```

### Archive Old Logs

```bash
# Compress logs older than 30 days
find stats/ -name "btc_hunt_*.log" -mtime +30 -exec gzip {} \;
```

### Delete Old Logs

```bash
# Delete logs older than 90 days
find stats/ -name "btc_hunt_*.log*" -mtime +90 -delete
```

### Disk Space Check

```bash
du -h stats/
```

## Troubleshooting

### No Log File Created

**Problem:** `--log-stats` flag not used

**Solution:**
```bash
./btc_hunt ... --log-stats
```

### Log File Empty

**Problem:** Process just started, no stats yet

**Solution:** Wait for the first stats interval (default 10 seconds)

### Daily Report Can't Find Log

**Problem:** Log file name doesn't match expected date format

**Solution:** Check if log file exists:
```bash
ls -la stats/btc_hunt_$(date -d yesterday +%Y%m%d).log
```

### Counters Not Resetting

**Problem:** Process running but counters keep growing across days

**Solution:** The log file will rotate, but you need to check the date. Each day's log starts fresh. If you see continuous numbers, you're looking at the wrong day's log.

## Comparison: Old vs New

### Old System ❌
- One giant log file: `btc_hunt_stats.log`
- Cumulative counters (never reset)
- Had to parse entire file
- Had to search for specific date
- File grows forever
- Hard to manage

### New System ✅
- Daily log files: `btc_hunt_20260104.log`
- Daily counters (reset at midnight)
- Just read last line
- File name IS the date
- One file per day
- Easy to manage

## Summary

🎯 **Key Points:**
1. One log file per day with date in filename
2. Counters reset automatically at midnight
3. Daily report just reads the last line
4. Much simpler, faster, and more accurate
5. Easy to archive/delete old logs

📧 **Daily emails now show exactly what happened THAT day on each server!**

