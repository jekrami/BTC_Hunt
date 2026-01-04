@echo off
REM Test script to verify mnemonic derivation

echo Building verification tool...
cargo build --release --bin verify_mnemonic

echo.
echo Running test with your provided mnemonic...
echo.

.\target\release\verify_mnemonic.exe "motor venture dilemma quote subject magnet keep large dry gossip bean paper"

echo.
echo ================================================================
echo Compare the output above with your expected results!
echo ================================================================
pause



