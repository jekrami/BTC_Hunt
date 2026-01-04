#!/usr/bin/env bash
# Test script for BTC Hunt with balance display
# Creates a test database with a known address and balance

set -e

echo "╔════════════════════════════════════════════════════════════╗"
echo "║         BTC Hunt - Balance Test Setup                     ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

TEST_DB="test_balance.db"
TEST_MNEMONIC="motor venture dilemma quote subject magnet keep large dry gossip bean paper"

# Clean up old test database
if [ -f "$TEST_DB" ]; then
    rm "$TEST_DB"
    echo "✓ Removed old test database"
fi

echo "Creating test database with balance..."

# Create database with schema
sqlite3 "$TEST_DB" <<'EOF'
-- Create table
CREATE TABLE btc_addresses (
    address TEXT PRIMARY KEY,
    balance REAL DEFAULT 0,
    address_type TEXT,
    last_updated INTEGER
);

-- Create index
CREATE INDEX idx_address ON btc_addresses(address);

-- Insert test addresses with balances
-- These are from the test mnemonic: motor venture dilemma...
INSERT INTO btc_addresses (address, balance, address_type, last_updated)
VALUES ('1GyNWR7LPXdLSHeN4nE4b9P3gNEcjZkmzd', 0.5, 'P2PKH', strftime('%s', 'now'));

INSERT INTO btc_addresses (address, balance, address_type, last_updated)
VALUES ('1Jo3qrSUxWYYJdhDawJ58QU7wtyVtqAK5A', 1.25, 'P2PKH', strftime('%s', 'now'));

INSERT INTO btc_addresses (address, balance, address_type, last_updated)
VALUES ('bc1qnc9umhdc04u0u5qfg0qu3aj75wvfps4z4sj7g6', 0.001, 'P2WPKH', strftime('%s', 'now'));

-- Display what we created
.mode column
.headers on
SELECT * FROM btc_addresses;
EOF

echo ""
echo "✓ Test database created successfully!"
echo "  Database: $(pwd)/$TEST_DB"
echo ""

# Show addresses in database
echo "Addresses in database:"
sqlite3 "$TEST_DB" "SELECT address, balance || ' BTC' as balance FROM btc_addresses;"

echo ""
echo "══════════════════════════════════════════════════════════════"
echo "Test Mnemonic:"
echo "  $TEST_MNEMONIC"
echo ""
echo "This mnemonic will derive addresses that match the database!"
echo "══════════════════════════════════════════════════════════════"
echo ""

echo "Now testing with verify_mnemonic..."
echo ""

# Run verify to show derived addresses
cargo run --release --bin verify_mnemonic -- "$TEST_MNEMONIC" 2>/dev/null | grep -E "1GyNWR7LPXdLSHeN4nE4b9P3gNEcjZkmzd|1Jo3qrSUxWYYJdhDawJ58QU7wtyVtqAK5A|bc1qnc9umhdc04u0u5qfg0qu3aj75wvfps4z4sj7g6" || true

echo ""
echo "══════════════════════════════════════════════════════════════"
echo "Ready to test! Run one of these commands:"
echo ""
echo "1. Create seeds file and test:"
echo "   echo '$TEST_MNEMONIC' > test_seeds.txt"
echo "   cargo build --release"
echo "   ./target/release/btc_hunt -d $TEST_DB -b 1 -n 1"
echo ""
echo "Expected output:"
echo "  ✓ 1GyNWR7LPXdLSHeN4nE4b9P3gNEcjZkmzd (Balance: 0.5 BTC)"
echo "  ✓ 1Jo3qrSUxWYYJdhDawJ58QU7wtyVtqAK5A (Balance: 1.25 BTC)"
echo "  ✓ bc1qnc9umhdc04u0u5qfg0qu3aj75wvfps4z4sj7g6 (Balance: 0.001 BTC)"
echo ""
echo "══════════════════════════════════════════════════════════════"

