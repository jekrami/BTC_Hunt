#!/usr/bin/env bash
# Daily report generator for BTC Hunt
# Run this daily (e.g., via cron)

set -e

echo "================================================================"
echo "         BTC Hunt - Daily Report Generator"
echo "================================================================"
echo ""

# Set your email here (optional)
EMAIL="ekrami@yahoo.com"

echo "Generating daily report..."
echo ""

cargo run --release --bin daily_report -- stats "$EMAIL"

echo ""
echo "================================================================"
echo "Report complete! Check reports/ directory"
echo "================================================================"

