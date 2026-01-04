# Test script for BTC Hunt with balance display
# Creates a test database with a known address and balance

$ErrorActionPreference = "Stop"

Write-Host "╔════════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║         BTC Hunt - Balance Test Setup                     ║" -ForegroundColor Cyan
Write-Host "╚════════════════════════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""

$testDb = "test_balance.db"
$testMnemonic = "motor venture dilemma quote subject magnet keep large dry gossip bean paper"

# Clean up old test database
if (Test-Path $testDb) {
    Remove-Item $testDb
    Write-Host "✓ Removed old test database" -ForegroundColor Yellow
}

Write-Host "Creating test database with balance..." -ForegroundColor Green

# Create SQL script file
$sqlContent = @"
CREATE TABLE btc_addresses (
    address TEXT PRIMARY KEY,
    balance REAL DEFAULT 0,
    address_type TEXT,
    last_updated INTEGER
);

CREATE INDEX idx_address ON btc_addresses(address);

INSERT INTO btc_addresses (address, balance, address_type, last_updated)
VALUES ('1GyNWR7LPXdLSHeN4nE4b9P3gNEcjZkmzd', 0.5, 'P2PKH', strftime('%s', 'now'));

INSERT INTO btc_addresses (address, balance, address_type, last_updated)
VALUES ('1Jo3qrSUxWYYJdhDawJ58QU7wtyVtqAK5A', 1.25, 'P2PKH', strftime('%s', 'now'));

INSERT INTO btc_addresses (address, balance, address_type, last_updated)
VALUES ('bc1qnc9umhdc04u0u5qfg0qu3aj75wvfps4z4sj7g6', 0.001, 'P2WPKH', strftime('%s', 'now'));
"@

$sqlContent | Out-File -FilePath "temp_create.sql" -Encoding UTF8 -NoNewline

# Create database by reading SQL file
$null = sqlite3 $testDb ".read temp_create.sql" 2>&1

# Clean up temp SQL file
Remove-Item "temp_create.sql" -ErrorAction SilentlyContinue

if (Test-Path $testDb) {
    Write-Host "`n✓ Test database created successfully!" -ForegroundColor Green
    Write-Host "  Database: $((Get-Item $testDb).FullName)" -ForegroundColor Cyan
    Write-Host ""
    
    # Show addresses in database
    Write-Host "Addresses in database:" -ForegroundColor Yellow
    sqlite3 $testDb "SELECT address, balance || ' BTC' as balance FROM btc_addresses;"
    
    Write-Host "`n══════════════════════════════════════════════════════════════" -ForegroundColor Cyan
    Write-Host "Test Mnemonic:" -ForegroundColor Yellow
    Write-Host "  $testMnemonic" -ForegroundColor White
    Write-Host ""
    Write-Host "This mnemonic will derive addresses that match the database!" -ForegroundColor Green
    Write-Host "══════════════════════════════════════════════════════════════" -ForegroundColor Cyan
    Write-Host ""
    
    Write-Host "Now testing with verify_mnemonic..." -ForegroundColor Green
    Write-Host ""
    
    # Run verify to show derived addresses
    cargo run --release --bin verify_mnemonic -- "$testMnemonic" 2>$null | Select-String "1GyNWR7LPXdLSHeN4nE4b9P3gNEcjZkmzd|1Jo3qrSUxWYYJdhDawJ58QU7wtyVtqAK5A|bc1qnc9umhdc04u0u5qfg0qu3aj75wvfps4z4sj7g6" | ForEach-Object { 
        Write-Host "  $($_.Line)" -ForegroundColor Cyan
    }
    
    Write-Host ""
    Write-Host "══════════════════════════════════════════════════════════════" -ForegroundColor Cyan
    Write-Host "Ready to test! Run one of these commands:" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "1. Create seeds file and test:" -ForegroundColor White
    Write-Host "   echo '$testMnemonic' > test_seeds.txt" -ForegroundColor Gray
    Write-Host "   cargo build --release" -ForegroundColor Gray
    Write-Host "   .\target\release\btc_hunt.exe -d $testDb -b 1 -n 1" -ForegroundColor Gray
    Write-Host ""
    Write-Host "2. Quick test (after building):" -ForegroundColor White
    Write-Host "   `$env:TEST_SEED='$testMnemonic'" -ForegroundColor Gray
    Write-Host "   .\target\release\btc_hunt.exe -d $testDb" -ForegroundColor Gray
    Write-Host ""
    Write-Host "Expected output:" -ForegroundColor Yellow
    Write-Host "  ✓ 1GyNWR7LPXdLSHeN4nE4b9P3gNEcjZkmzd (Balance: 0.5 BTC)" -ForegroundColor Green
    Write-Host "  ✓ 1Jo3qrSUxWYYJdhDawJ58QU7wtyVtqAK5A (Balance: 1.25 BTC)" -ForegroundColor Green
    Write-Host "  ✓ bc1qnc9umhdc04u0u5qfg0qu3aj75wvfps4z4sj7g6 (Balance: 0.001 BTC)" -ForegroundColor Green
    Write-Host ""
    Write-Host "══════════════════════════════════════════════════════════════" -ForegroundColor Cyan
    
} else {
    Write-Host "`n✗ Failed to create database" -ForegroundColor Red
    Write-Host "Make sure sqlite3 is installed and in PATH" -ForegroundColor Yellow
}

