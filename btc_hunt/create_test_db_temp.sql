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
