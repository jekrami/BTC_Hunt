-- Create Bitcoin address database schema
-- Run with: sqlite3 btc_addresses.db < create_test_db.sql

-- Create btc_addresses table (matching your existing schema)
CREATE TABLE IF NOT EXISTS btc_addresses (
    address TEXT PRIMARY KEY,
    balance REAL DEFAULT 0,
    address_type TEXT,
    last_updated INTEGER
);

-- Create index for fast lookups (CRITICAL for performance!)
CREATE INDEX IF NOT EXISTS idx_address ON btc_addresses(address);

-- Insert some known Bitcoin addresses for testing
-- These are real addresses but for testing purposes only
INSERT OR IGNORE INTO btc_addresses (address, balance, address_type, last_updated) 
VALUES ('1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa', 0, 'P2PKH', strftime('%s', 'now'));  -- Genesis block (Satoshi)

INSERT OR IGNORE INTO btc_addresses (address, balance, address_type, last_updated)
VALUES ('1GyNWR7LPXdLSHeN4nE4b9P3gNEcjZkmzd', 0, 'P2PKH', strftime('%s', 'now'));  -- From verify test

INSERT OR IGNORE INTO btc_addresses (address, balance, address_type, last_updated)
VALUES ('1Jo3qrSUxWYYJdhDawJ58QU7wtyVtqAK5A', 0, 'P2PKH', strftime('%s', 'now'));  -- From verify test

INSERT OR IGNORE INTO btc_addresses (address, balance, address_type, last_updated)
VALUES ('33ML21FE9QSqh9wizdQbZsHfE41vwkRT78', 0, 'P2SH', strftime('%s', 'now'));  -- From verify test

INSERT OR IGNORE INTO btc_addresses (address, balance, address_type, last_updated)
VALUES ('bc1qnc9umhdc04u0u5qfg0qu3aj75wvfps4z4sj7g6', 0, 'P2WPKH', strftime('%s', 'now'));  -- From verify test

-- Display summary
.mode column
.headers on
SELECT COUNT(*) as 'Total Addresses' FROM btc_addresses;
SELECT 'Sample addresses:' as '';
SELECT * FROM btc_addresses LIMIT 5;

-- Instructions
SELECT '' as '';
SELECT 'Database created successfully!' as 'Status';
SELECT 'Location: btc_addresses.db' as 'File';
SELECT 'Table: btc_addresses' as 'Schema';
SELECT 'Index: idx_address' as 'Index';

