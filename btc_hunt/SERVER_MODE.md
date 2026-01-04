# Server Mode Guide

Running BTC Hunt on a remote server with simple, log-friendly output.

## Quick Start

```bash
# Simple mode (perfect for servers and logs)
./target/release/btc_hunt -d btc_addresses.db -b 5000 --simple --log-stats
```

## Output Comparison

### Normal Mode (Default)
```
╔════════════════════════════════════════════════════════════╗
║              Bitcoin Address Hunter v2.0                   ║
║          Unified Mnemonic → Address → Database             ║
╚════════════════════════════════════════════════════════════╝

Configuration:
  • Database:           btc_addresses.db
  • Batch size:         5000 mnemonics
  • Max batches:        unlimited
  • Addresses/path:     10
  • Worker threads:     20
  • DB batch size:      1000

Starting hunt...

╔════════════════════════════════════════════════════════════╗
║                    BTC HUNT STATISTICS                     ║
╠════════════════════════════════════════════════════════════╣
║ Runtime:         120 seconds
║ Batches:         24
║ Mnemonics:       120000 (1000/s)
║ Addresses:       7200000 (60000/s)
║ Checked:         7200000
╚════════════════════════════════════════════════════════════╝
```

### Simple Mode (`--simple`)
```
[2026-01-04 14:30:00] BTC Hunt v2.0 starting | DB: btc_addresses.db | Batch: 5000 | Threads: 20 | Log: enabled
[2026-01-04 14:30:10] Checked: 300000 addresses | Mnemonics: 5000 | Batches: 1 | Runtime: 10s | Rate: 30000/s
[2026-01-04 14:30:20] Checked: 600000 addresses | Mnemonics: 10000 | Batches: 2 | Runtime: 20s | Rate: 30000/s
[2026-01-04 14:30:30] Checked: 900000 addresses | Mnemonics: 15000 | Batches: 3 | Runtime: 30s | Rate: 30000/s
```

## Benefits of Simple Mode

✅ **Log-friendly** - One line per update, easy to parse  
✅ **Timestamps** - Every line has a timestamp  
✅ **Compact** - No fancy boxes or formatting  
✅ **Grep-able** - Easy to search logs  
✅ **Remote-friendly** - Works well over SSH  
✅ **Parseable** - Easy for log analyzers  

## Recommended Server Settings

### For Production Server

```bash
# High performance, simple logging
./target/release/btc_hunt \
  -d btc_addresses.db \
  -b 10000 \
  --simple \
  --log-stats \
  --email your@email.com \
  > btc_hunt.log 2>&1 &
```

### Update Interval

By default, stats update every **10 seconds** (changed from 5 for server use).

Adjust if needed:
```bash
# Every 30 seconds (quieter)
./target/release/btc_hunt -d btc_addresses.db --simple -s 30

# Every 5 seconds (more frequent)
./target/release/btc_hunt -d btc_addresses.db --simple -s 5

# Every minute (very quiet)
./target/release/btc_hunt -d btc_addresses.db --simple -s 60
```

## Running as Background Process

### Linux (nohup)

```bash
nohup ./target/release/btc_hunt \
  -d btc_addresses.db \
  -b 5000 \
  --simple \
  --log-stats \
  > btc_hunt.log 2>&1 &

# Save PID for later
echo $! > btc_hunt.pid
```

Check logs:
```bash
# Follow live
tail -f btc_hunt.log

# Last 20 lines
tail -20 btc_hunt.log

# Check for matches
grep "MATCH FOUND" btc_hunt.log
```

Stop:
```bash
kill $(cat btc_hunt.pid)
```

### Linux (screen)

```bash
# Start screen session
screen -S btc_hunt

# Run in simple mode
./target/release/btc_hunt -d btc_addresses.db -b 5000 --simple --log-stats

# Detach: Ctrl+A, D

# Reattach later
screen -r btc_hunt
```

### Linux (tmux)

```bash
# Start tmux session
tmux new -s btc_hunt

# Run in simple mode
./target/release/btc_hunt -d btc_addresses.db -b 5000 --simple --log-stats

# Detach: Ctrl+B, D

# Reattach later
tmux attach -t btc_hunt
```

### Linux (systemd service)

Create `/etc/systemd/system/btc-hunt.service`:

```ini
[Unit]
Description=BTC Hunt Address Scanner
After=network.target

[Service]
Type=simple
User=your_user
WorkingDirectory=/path/to/btc_hunt
ExecStart=/path/to/btc_hunt/target/release/btc_hunt -d btc_addresses.db -b 5000 --simple --log-stats --email your@email.com
Restart=always
RestartSec=10
StandardOutput=append:/var/log/btc-hunt.log
StandardError=append:/var/log/btc-hunt-error.log

[Install]
WantedBy=multi-user.target
```

Enable and start:
```bash
sudo systemctl enable btc-hunt
sudo systemctl start btc-hunt
sudo systemctl status btc-hunt

# View logs
sudo journalctl -u btc-hunt -f
```

## Log Parsing

### Check Progress

```bash
# Latest status
tail -1 btc_hunt.log

# Addresses checked today
grep "$(date +%Y-%m-%d)" btc_hunt.log | tail -1
```

### Extract Statistics

```bash
# Total addresses checked (last line)
tail -1 btc_hunt.log | grep -oP 'Checked: \K[0-9]+'

# Current rate
tail -1 btc_hunt.log | grep -oP 'Rate: \K[0-9]+'

# Runtime
tail -1 btc_hunt.log | grep -oP 'Runtime: \K[0-9]+'
```

### Find Matches

```bash
# Check for any matches
grep "MATCH FOUND" btc_hunt.log

# Count matches
grep -c "MATCH FOUND" btc_hunt.log

# Show match details
grep -A 5 "MATCH FOUND" btc_hunt.log
```

## Monitoring

### Watch Live

```bash
# Update every 2 seconds
watch -n 2 'tail -1 btc_hunt.log'
```

### Check if Running

```bash
# By process name
ps aux | grep btc_hunt | grep -v grep

# By PID file
if [ -f btc_hunt.pid ] && kill -0 $(cat btc_hunt.pid) 2>/dev/null; then
    echo "Running"
else
    echo "Not running"
fi
```

### Disk Usage

```bash
# Log file size
du -h btc_hunt.log

# Stats directory size
du -sh stats/

# Results directory
du -sh results/
```

## Log Rotation

### Manual Rotation

```bash
#!/bin/bash
# rotate_logs.sh

LOG_FILE="btc_hunt.log"
DATE=$(date +%Y%m%d_%H%M%S)

if [ -f "$LOG_FILE" ]; then
    mv "$LOG_FILE" "$LOG_FILE.$DATE"
    gzip "$LOG_FILE.$DATE"
    
    # Keep only last 30 days
    find . -name "btc_hunt.log.*.gz" -mtime +30 -delete
fi

# Restart btc_hunt to create new log
killall -HUP btc_hunt
```

### Using logrotate (Linux)

Create `/etc/logrotate.d/btc-hunt`:

```
/path/to/btc_hunt.log {
    daily
    rotate 30
    compress
    delaycompress
    notifempty
    missingok
    postrotate
        killall -HUP btc_hunt 2>/dev/null || true
    endscript
}
```

## Remote Monitoring

### SSH + watch

```bash
ssh user@server 'watch -n 2 tail -1 /path/to/btc_hunt.log'
```

### Parse and Display

```bash
#!/bin/bash
# monitor.sh - Nice display of current status

LOG="/path/to/btc_hunt.log"

while true; do
    clear
    echo "=== BTC Hunt Status ==="
    echo ""
    tail -5 "$LOG"
    echo ""
    echo "Last update: $(date)"
    sleep 10
done
```

## Email Alerts

With `--simple` mode, you can easily send email alerts:

```bash
#!/bin/bash
# alert_on_match.sh

LOG="btc_hunt.log"
LAST_CHECK="last_check.txt"
EMAIL="your@email.com"

# Get new lines since last check
NEW_LINES=$(comm -13 <(sort "$LAST_CHECK" 2>/dev/null) <(sort "$LOG"))

# Check for matches
if echo "$NEW_LINES" | grep -q "MATCH FOUND"; then
    echo "$NEW_LINES" | grep "MATCH FOUND" | mail -s "BTC Hunt - Match Found!" "$EMAIL"
fi

# Update last check
cp "$LOG" "$LAST_CHECK"
```

Run via cron:
```bash
# Check every 5 minutes
*/5 * * * * /path/to/alert_on_match.sh
```

## Performance Tuning for Servers

### CPU Priority

```bash
# Normal priority
nice -n 0 ./target/release/btc_hunt -d btc_addresses.db --simple

# Lower priority (nicer to other processes)
nice -n 10 ./target/release/btc_hunt -d btc_addresses.db --simple

# High priority (use more CPU)
nice -n -10 ./target/release/btc_hunt -d btc_addresses.db --simple
```

### Memory Limits

```bash
# Limit to 2GB RAM
ulimit -v 2097152
./target/release/btc_hunt -d btc_addresses.db --simple -b 1000
```

### CPU Affinity

```bash
# Run on specific CPUs (0-7)
taskset -c 0-7 ./target/release/btc_hunt -d btc_addresses.db --simple -b 5000
```

## Example: Complete Server Setup

```bash
#!/bin/bash
# setup_btc_hunt_server.sh

# 1. Build
cd /opt/btc_hunt
cargo build --release

# 2. Create directories
mkdir -p logs stats results

# 3. Start in background with simple mode
nohup ./target/release/btc_hunt \
    -d /data/btc_addresses.db \
    -b 10000 \
    --simple \
    --log-stats \
    --email admin@example.com \
    -s 10 \
    > logs/btc_hunt_$(date +%Y%m%d).log 2>&1 &

# 4. Save PID
echo $! > btc_hunt.pid

echo "BTC Hunt started with PID: $(cat btc_hunt.pid)"
echo "Log: logs/btc_hunt_$(date +%Y%m%d).log"
echo ""
echo "Monitor with: tail -f logs/btc_hunt_$(date +%Y%m%d).log"
```

## Summary

**For remote servers, always use:**

```bash
./target/release/btc_hunt \
  -d btc_addresses.db \
  -b 5000 \
  --simple \
  --log-stats \
  > btc_hunt.log 2>&1 &
```

**Key features:**
- ✅ `--simple` - Clean, timestamped output
- ✅ `--log-stats` - Save stats to files for daily reports
- ✅ `-s 10` - Update every 10 seconds (default)
- ✅ Background process with logs
- ✅ Easy to parse and monitor

Perfect for production servers! 🚀

