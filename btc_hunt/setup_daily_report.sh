#!/bin/bash
# Setup script for BTC Hunt daily reports

# Get the directory where this script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
EMAIL="${1:-}"

if [ -z "$EMAIL" ]; then
    echo "Usage: $0 <your-email@example.com>"
    echo ""
    echo "This script will set up a daily cron job to send you daily reports"
    exit 1
fi

echo "╔════════════════════════════════════════════════════════════╗"
echo "║       BTC Hunt - Daily Report Cron Setup                  ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""
echo "Email:     $EMAIL"
echo "Directory: $SCRIPT_DIR"
echo ""
echo "ℹ️  Daily log files: stats/btc_hunt_YYYYMMDD.log"
echo "ℹ️  Counters reset automatically at midnight"
echo ""

# Create the cron command
CRON_CMD="59 23 * * * cd $SCRIPT_DIR && ./target/release/daily_report --log-dir . --email $EMAIL >> daily_report.log 2>&1"

echo "Cron job to be added:"
echo "  $CRON_CMD"
echo ""
read -p "Add this cron job? (y/n): " -n 1 -r
echo ""

if [[ $REPLY =~ ^[Yy]$ ]]; then
    # Add to crontab
    (crontab -l 2>/dev/null; echo "$CRON_CMD") | crontab -
    echo "✓ Cron job added successfully!"
    echo ""
    echo "Daily reports will be sent to $EMAIL every day at 11:59 PM"
    echo ""
    echo "To view your crontab: crontab -l"
    echo "To edit your crontab: crontab -e"
    echo "To remove this job:   crontab -e (then delete the line)"
else
    echo "Setup cancelled."
    exit 1
fi

echo ""
echo "Testing daily_report tool..."
./target/release/daily_report --log-dir . --email "$EMAIL"

echo ""
echo "✓ Setup complete!"

