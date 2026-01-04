# Daily Reports and Email Notifications

This guide explains how to set up daily reporting and email notifications, similar to your old `cleanup_and_report.sh` script.

## Features

✅ **Daily Statistics Reports** - Summarize mnemonics checked, addresses scanned, etc.  
✅ **Email Notifications** - Get notified when matches are found  
✅ **Automatic Logging** - Track all activity with timestamps  
✅ **Performance Metrics** - See addresses/second, searches/hour, etc.  

## Quick Start

### 1. Run with Logging Enabled

```powershell
# Windows - with statistics logging
.\target\release\btc_hunt.exe -d btc_addresses.db -b 50000 --log-stats

# With email notifications too
.\target\release\btc_hunt.exe -d btc_addresses.db -b 50000 --log-stats --email ekrami@yahoo.com
```

This creates log files in `stats/` directory.

### 2. Generate Daily Report

```powershell
# Windows
.\daily_report.bat

# Or manually
cargo run --release --bin daily_report -- stats ekrami@yahoo.com
```

## File Structure

```
btc_hunt/
├── stats/                      # Statistics logs (auto-created)
│   ├── btc_hunt_20260104_*.log
│   └── btc_hunt_20260105_*.log
│
├── reports/                    # Daily reports (auto-created)
│   ├── 2026-01-04/
│   │   └── summary.txt        # Daily summary
│   └── 2026-01-05/
│       └── summary.txt
│
└── results/                    # Match reports
    └── MATCH_FOUND_*.txt
```

## Command-Line Options

### Main Program (btc_hunt)

```powershell
--log-stats              # Enable statistics logging to file
--email <ADDRESS>        # Email address for match notifications
```

Example:
```powershell
.\target\release\btc_hunt.exe `
  -d btc_addresses.db `
  -b 50000 `
  --log-stats `
  --email ekrami@yahoo.com
```

### Daily Report Tool

```bash
daily_report [STATS_DIR] [EMAIL]
```

Arguments:
- `STATS_DIR` - Directory containing log files (default: `stats`)
- `EMAIL` - Email address for notifications (optional)

Example:
```powershell
cargo run --release --bin daily_report -- stats ekrami@yahoo.com
```

## Daily Report Contents

The daily report (`reports/YYYY-MM-DD/summary.txt`) includes:

```
╔════════════════════════════════════════════════════════════╗
║          BTC Hunt Daily Report - 2026-01-04          ║
╚════════════════════════════════════════════════════════════╝

Generated: 2026-01-05 02:00:00

═══════════════════════════════════════════════════════════
STATISTICS
═══════════════════════════════════════════════════════════

Batches Processed:     240
Mnemonics Generated:   12000000
Addresses Derived:     720000000
Addresses Checked:     720000000

Total Runtime:         14h 23m
Average Rate:          231 mnemonics/sec

═══════════════════════════════════════════════════════════
MATCHES FOUND
═══════════════════════════════════════════════════════════

No matches found on 2026-01-04

═══════════════════════════════════════════════════════════
PERFORMANCE
═══════════════════════════════════════════════════════════

Addresses per second:  13889
Searches per hour:     50000000
Searches per day:      1200000000

═══════════════════════════════════════════════════════════
SYSTEM INFO
═══════════════════════════════════════════════════════════

Log files:             3
Report location:       reports\2026-01-04\summary.txt
```

## Email Setup

### Windows 11 (PowerShell)

The program generates a PowerShell script when match is found. Configure SMTP:

1. Edit `send_notification.ps1`:
   ```powershell
   $SmtpServer = "smtp.gmail.com"  # Your SMTP server
   $Port = 587
   $Username = "your-email@gmail.com"
   ```

2. For Gmail, use App Password:
   - Go to Google Account → Security → 2-Step Verification → App passwords
   - Generate password for "Mail"
   - Use that password in script

3. Run the script:
   ```powershell
   .\send_notification.ps1
   ```

**Alternative: Use Windows Mail**
```powershell
# Built-in Windows Mail doesn't support command-line
# Use PowerShell Send-MailMessage instead
```

### Linux/Unix (mail command)

Email works automatically if `mail` command is installed:

```bash
# Install mail command (Debian/Ubuntu)
sudo apt-get install mailutils

# Install mail command (RHEL/CentOS)
sudo yum install mailx

# Configure sendmail or postfix
sudo dpkg-reconfigure exim4-config
```

Then email notifications work automatically!

## Automation

### Windows Task Scheduler

**Run Daily Report at 2 AM:**

1. Open Task Scheduler (`taskschd.msc`)
2. Create Basic Task
3. Name: "BTC Hunt Daily Report"
4. Trigger: Daily at 2:00 AM
5. Action: Start a program
   - Program: `powershell.exe`
   - Arguments: `-File "D:\MyProjects\BTC_Hunt\btc_hunt\daily_report.bat"`
   - Start in: `D:\MyProjects\BTC_Hunt\btc_hunt`
6. Save

**Run BTC Hunt Continuously:**

Create another task:
- Name: "BTC Hunt Scanner"
- Trigger: At startup
- Action: Start a program
  - Program: `powershell.exe`
  - Arguments: `-Command "cd D:\MyProjects\BTC_Hunt\btc_hunt; .\target\release\btc_hunt.exe -d ..\db_search\btc_addresses.db -b 50000 --log-stats --email ekrami@yahoo.com > logs\hunt_$(Get-Date -Format 'yyyyMMdd').log 2>&1"`

### Linux Cron

Add to crontab (`crontab -e`):

```bash
# Daily report at 2 AM
0 2 * * * cd /path/to/btc_hunt && ./daily_report.sh

# Or run btc_hunt continuously (restart if crashed)
@reboot cd /path/to/btc_hunt && while true; do ./target/release/btc_hunt -d btc_addresses.db -b 50000 --log-stats --email ekrami@yahoo.com >> logs/hunt.log 2>&1; sleep 10; done
```

## Log Rotation

Logs can grow large. Clean old logs:

### Windows (PowerShell)
```powershell
# Delete logs older than 30 days
Get-ChildItem -Path "stats" -Filter "*.log" | 
  Where-Object {$_.LastWriteTime -lt (Get-Date).AddDays(-30)} | 
  Remove-Item
```

Add this to Task Scheduler weekly.

### Linux (bash)
```bash
# Delete logs older than 30 days
find stats/ -name "*.log" -mtime +30 -delete
```

Add to crontab:
```bash
# Weekly cleanup on Sunday at 3 AM
0 3 * * 0 find /path/to/btc_hunt/stats/ -name "*.log" -mtime +30 -delete
```

## Example Usage

### Full Day Workflow

**Morning (Automatic):**
1. Daily report runs at 2 AM via Task Scheduler
2. Email sent if matches found yesterday
3. Report saved to `reports/2026-01-04/summary.txt`

**During Day:**
1. BTC Hunt runs continuously with `--log-stats`
2. Statistics logged every 5 seconds to `stats/` directory
3. If match found:
   - Program stops
   - Saves detailed report to `results/`
   - Sends email notification (if configured)
   - Logs match in statistics file

**Next Morning:**
1. Daily report analyzes yesterday's logs
2. Summarizes statistics
3. Lists any matches found
4. Emails summary (if configured)

## Manual Commands

### Check Today's Progress
```powershell
# View latest log
Get-Content (Get-ChildItem -Path stats -Filter "*.log" | Sort-Object LastWriteTime -Descending | Select-Object -First 1).FullName -Tail 20
```

### Generate Report for Specific Date
```powershell
# Edit daily_report tool to accept date parameter
# Or manually check logs for that date
Get-Content stats\btc_hunt_20260104*.log
```

### Test Email
```powershell
# Run with match simulation
# (You'd need to add a test match to database)
```

## Comparison with Old Script

### Old `cleanup_and_report.sh`
```bash
- Checked batch directories
- Counted found matches
- Deleted empty batches
- Sent email on match
- Generated daily summary
```

### New System
```bash
✅ Logs statistics continuously to files
✅ Generates comprehensive daily reports
✅ Sends email on match (immediate)
✅ Tracks performance metrics
✅ No cleanup needed (no temp files!)
✅ Better email integration
✅ Cross-platform (Windows + Linux)
```

## Statistics Format

Log file format:
```
[2026-01-04 14:30:00] Runtime: 3600 | Batches: 72 | Mnemonics: 3600000 | Addresses: 216000000 | Checked: 216000000
[2026-01-04 14:30:05] Runtime: 3605 | Batches: 72 | Mnemonics: 3610000 | Addresses: 216600000 | Checked: 216600000
[2026-01-04 14:35:00] 🎉 MATCH FOUND! 2 addresses matched
```

## Email Notification Format

**Subject:** `BTC Hunt - MATCH FOUND!`

**Body:**
```
🎉 Bitcoin Address Match Found!

Timestamp: 2026-01-04 14:35:12

Matched Addresses (2):
  • 1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa
  • 3J98t1WpEZ73CNmYviecrnyiWrnqRhWNLy

Check results/ directory for detailed report.
```

## Troubleshooting

### Email Not Sending (Windows)
- Check SMTP server settings
- Use App Password for Gmail
- Test with simple PowerShell command first:
  ```powershell
  Send-MailMessage -To "test@example.com" -Subject "Test" -Body "Test" -SmtpServer "smtp.gmail.com" -Port 587 -UseSsl -Credential (Get-Credential)
  ```

### Email Not Sending (Linux)
- Install mail command: `sudo apt-get install mailutils`
- Configure mail server
- Test: `echo "Test" | mail -s "Test" your@email.com`

### Reports Not Generated
- Check `stats/` directory exists and has log files
- Ensure date format matches (YYYYMMDD)
- Run daily_report manually to see errors

### Logs Growing Too Large
- Set up log rotation (see above)
- Consider reducing `--stats-interval` (log less often)
- Archive old reports regularly

## Best Practices

1. **Run with logging always:** `--log-stats`
2. **Set up daily reports:** Via Task Scheduler/cron
3. **Configure email:** Get instant notifications
4. **Monitor disk space:** Clean old logs monthly
5. **Backup reports:** Copy to external drive monthly
6. **Test email:** Before relying on it

## Summary

The new system provides **better** reporting than your old script:

✅ **Real-time logging** (not just daily)  
✅ **Immediate email** (not next day)  
✅ **Better statistics** (performance metrics)  
✅ **No cleanup needed** (no temp files)  
✅ **Cross-platform** (Windows + Linux)  

**To enable, just add:** `--log-stats --email your@email.com`

That's it! 🎉

