# BTC Hunt - Daily Reports Guide

## Overview

The daily report system provides automated summary emails of your Bitcoin address hunting activity. Reports include:

- **Date and Server**: Which day and which server ran the search
- **Statistics**: Batches processed, mnemonics generated, addresses checked
- **Performance**: Runtime and check rates
- **Matches**: Any addresses with balances found

## How It Works

### 1. Main Program Logs (`btc_hunt`)

When you run `btc_hunt` with `--log-stats`, it creates **daily log files** with the date in the filename:

```
stats/btc_hunt_20260104.log  ← January 4th
stats/btc_hunt_20260105.log  ← January 5th
stats/btc_hunt_20260106.log  ← January 6th
```

The log format is:
```
[2026-01-04 17:17:14] Runtime: 10 | Batches: 0 | Mnemonics: 10000 | Addresses: 0 | Checked: 0
[2026-01-04 17:17:24] Runtime: 20 | Batches: 0 | Mnemonics: 10000 | Addresses: 600000 | Checked: 0
[2026-01-04 17:17:34] Runtime: 30 | Batches: 1 | Mnemonics: 20000 | Addresses: 600000 | Checked: 600000
```

**At midnight:**
- ✅ New log file created with new date
- ✅ All counters reset to 0
- ✅ Process continues running seamlessly

### 2. Daily Report Generator (`daily_report`)

The `daily_report` tool:
- Finds yesterday's log file: `btc_hunt_YYYYMMDD.log`
- Reads **ONLY the LAST line** (contains final daily totals)
- Generates a report file in `reports/report_YYYY-MM-DD.txt`
- **ALWAYS sends an email** (not just when matches found)

**Much simpler and faster!**

## Setup on Linux

### Quick Setup

Run the setup script:

```bash
cd btc_hunt
chmod +x setup_daily_report.sh
./setup_daily_report.sh your-email@example.com
```

This will:
1. Add a cron job to run daily at 11:59 PM
2. Test the report generation
3. Show you the cron entry

### Manual Setup

1. **Edit crontab**:
   ```bash
   crontab -e
   ```

2. **Add this line** (adjust path and email):
   ```
   5 0 * * * cd /path/to/btc_hunt && ./target/release/daily_report --log-dir stats --email your@email.com >> daily_report.log 2>&1
   ```
   
   **Note:** Runs at 00:05 AM (5 minutes after midnight) to ensure yesterday's log is complete.

3. **Save and exit**

### Verify Cron Job

```bash
# View your crontab
crontab -l

# Check if cron service is running
systemctl status cron      # Debian/Ubuntu
systemctl status crond     # RHEL/CentOS
```

## Manual Usage

### Generate Report for Yesterday

```bash
cd btc_hunt
./target/release/daily_report --log-dir stats --email your@email.com
```

### Generate Report Without Email

```bash
./target/release/daily_report --log-dir stats
```

This will create a report file in `reports/` but won't send an email.

## Email Format

You'll receive an email like this:

```
Subject: BTC Hunt Daily Report - 2026-01-04

╔════════════════════════════════════════════════════════════╗
║          BTC Hunt Daily Report - 2026-01-04          ║
╚════════════════════════════════════════════════════════════╝

Date:                  2026-01-04
Server:                hp-g9-server

═══════════════════════════════════════════════════════════
DAILY STATISTICS
═══════════════════════════════════════════════════════════

Batches Processed:     1500
Mnemonics Generated:   15000000
Addresses Derived:     900000000
Addresses Checked:     900000000

Total Runtime:         23h 45m 30s
Check Rate:            10526 addresses/sec

═══════════════════════════════════════════════════════════
MATCHES FOUND
═══════════════════════════════════════════════════════════

No matches found today.

---
BTC Hunt - Automated Daily Report
```

If matches are found, the subject will be:
```
Subject: BTC Hunt Daily Report - 2026-01-04 - 2 MATCH(ES) FOUND! 🎉
```

## Linux Server Requirements

### Mail Command

The system uses the `mail` command to send emails. Install it if needed:

**Debian/Ubuntu:**
```bash
sudo apt-get install mailutils
```

**RHEL/CentOS:**
```bash
sudo yum install mailx
```

### Configure Mail (if needed)

If your server doesn't have outgoing mail configured, you may need to set up a mail relay. For simple testing:

```bash
# Install and configure postfix
sudo apt-get install postfix

# During installation, choose "Internet Site"
# Set your server's hostname
```

## Using with Screen

If you're running `btc_hunt` in a `screen` session on your Linux server:

```bash
# Start screen session
screen -S btc_hunt

# Run btc_hunt with logging
cd /path/to/btc_hunt
./target/release/btc_hunt -d btc_addresses.db -b 10000 -t 0 --simple --log-stats --email your@email.com

# Detach: Ctrl+A, then D

# The daily report cron job will work independently
# It reads the log files that btc_hunt creates
```

## Troubleshooting

### No Email Received

1. **Check if mail command works**:
   ```bash
   echo "Test" | mail -s "Test Subject" your@email.com
   ```

2. **Check daily report logs**:
   ```bash
   cat daily_report.log
   ```

3. **Check cron logs**:
   ```bash
   grep CRON /var/log/syslog    # Debian/Ubuntu
   grep CRON /var/log/cron      # RHEL/CentOS
   ```

4. **Check spam folder** - Server emails often go to spam

### No Statistics in Report

1. **Check log file exists**:
   ```bash
   ls -la stats/btc_hunt_stats.log
   ```

2. **Check log file format**:
   ```bash
   tail -20 stats/btc_hunt_stats.log
   ```

3. **Verify `--log-stats` flag** is used when running `btc_hunt`

### Wrong Date

The `daily_report` tool looks for **yesterday's** date in the log file. If you run it on January 5th, it will report on January 4th's activity.

To test with today's data, you can temporarily modify the date check or wait until tomorrow.

## Report Files

Reports are saved in:
```
reports/report_YYYY-MM-DD.txt
```

You can review past reports anytime:
```bash
ls -la reports/
cat reports/report_2026-01-04.txt
```

## Summary

✅ **What happens now:**
1. `btc_hunt` runs continuously, creating log entries every 10 seconds
2. Every night at 11:59 PM, `daily_report` runs via cron
3. It reads yesterday's log entries
4. It generates a report with final statistics
5. **It ALWAYS sends you an email** with the summary
6. If matches were found, the email subject will highlight this

🎯 **You get:** Daily emails telling you exactly how many addresses were checked on each server, every single day, even if no matches are found!

