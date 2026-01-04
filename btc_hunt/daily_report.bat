@echo off
REM Daily report generator for BTC Hunt
REM Run this daily (e.g., via Task Scheduler)

echo ================================================================
echo          BTC Hunt - Daily Report Generator
echo ================================================================
echo.

REM Set your email here (optional)
set EMAIL=ekrami@yahoo.com

echo Generating daily report...
echo.

cargo run --release --bin daily_report -- stats %EMAIL%

echo.
echo ================================================================
echo Report complete! Check reports\ directory
echo ================================================================
pause

