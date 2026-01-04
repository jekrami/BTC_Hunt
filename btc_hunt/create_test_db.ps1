# Create a test Bitcoin address database
# This creates a sample database with a few known addresses for testing

$dbPath = "..\btc_addresses.db"

Write-Host "Creating test Bitcoin address database..." -ForegroundColor Green

# Remove old database if exists
if (Test-Path $dbPath) {
    Remove-Item $dbPath
    Write-Host "Removed existing database" -ForegroundColor Yellow
}

# Create database and table
sqlite3 $dbPath @"
-- Create btc_addresses table (matching your existing schema)
CREATE TABLE btc_addresses (
    address TEXT PRIMARY KEY,
    balance REAL DEFAULT 0,
    address_type TEXT,
    last_updated INTEGER
);

-- Create index for fast lookups
CREATE INDEX idx_address ON btc_addresses(address);

-- Insert some known Bitcoin addresses for testing
INSERT INTO btc_addresses (address, balance, address_type, last_updated) 
VALUES ('1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa', 0, 'P2PKH', strftime('%s', 'now'));

INSERT INTO btc_addresses (address, balance, address_type, last_updated)
VALUES ('1GyNWR7LPXdLSHeN4nE4b9P3gNEcjZkmzd', 0, 'P2PKH', strftime('%s', 'now'));

INSERT INTO btc_addresses (address, balance, address_type, last_updated)
VALUES ('1Jo3qrSUxWYYJdhDawJ58QU7wtyVtqAK5A', 0, 'P2PKH', strftime('%s', 'now'));

INSERT INTO btc_addresses (address, balance, address_type, last_updated)
VALUES ('33ML21FE9QSqh9wizdQbZsHfE41vwkRT78', 0, 'P2SH', strftime('%s', 'now'));

INSERT INTO btc_addresses (address, balance, address_type, last_updated)
VALUES ('bc1qnc9umhdc04u0u5qfg0qu3aj75wvfps4z4sj7g6', 0, 'P2WPKH', strftime('%s', 'now'));

-- Display summary
.mode column
.headers on
SELECT COUNT(*) as 'Total Addresses' FROM btc_addresses;
SELECT * FROM btc_addresses;
"@

if ($LASTEXITCODE -eq 0) {
    Write-Host "`n✓ Database created successfully!" -ForegroundColor Green
    Write-Host "Location: $((Get-Item $dbPath).FullName)" -ForegroundColor Cyan
    Write-Host "`nYou can now run:" -ForegroundColor Yellow
    Write-Host "  .\target\release\btc_hunt.exe -d $dbPath -b 100 -n 1" -ForegroundColor White
} else {
    Write-Host "`n✗ Failed to create database" -ForegroundColor Red
    Write-Host "Make sure sqlite3 is installed and in PATH" -ForegroundColor Yellow
}

