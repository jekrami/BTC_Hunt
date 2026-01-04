# Quick Start Guide

## Build the Project

```bash
cd btc_hunt
cargo build --release
```

Build time: ~2-5 minutes (downloads dependencies first time)

## Run Your First Test

```bash
# Create a test database (if you don't have one)
sqlite3 test.db << EOF
CREATE TABLE addresses (address TEXT PRIMARY KEY);
INSERT INTO addresses VALUES ('1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa');
CREATE INDEX idx_address ON addresses(address);
EOF

# Run the hunter
./target/release/btc_hunt -d test.db -b 100 -n 10
```

This will:
- Generate 100 mnemonics per batch
- Process 10 batches (1,000 mnemonics total)
- Check against your test database

## Typical Production Run

```bash
# Unlimited run with optimal settings
./target/release/btc_hunt -d btc_addresses.db -b 5000
```

Press `Ctrl+C` to stop anytime. Statistics are shown every 5 seconds.

## When a Match is Found

The program will:
1. Stop immediately
2. Print "🎉 MATCH FOUND! 🎉"
3. Save a detailed report to `results/MATCH_FOUND_<timestamp>.txt`
4. Exit with code 0

## Performance Tuning

### Fast Machine (16+ cores, 32GB+ RAM)
```bash
./target/release/btc_hunt -d btc_addresses.db -b 10000 -t 0 -a 10
```

### Medium Machine (8 cores, 16GB RAM)
```bash
./target/release/btc_hunt -d btc_addresses.db -b 5000 -t 0 -a 10
```

### Slow Machine (4 cores, 8GB RAM)
```bash
./target/release/btc_hunt -d btc_addresses.db -b 1000 -t 4 -a 5
```

## Statistics Explained

```
║ Runtime:         120 seconds           ← Total time running
║ Batches:         120                   ← Completed batches
║ Mnemonics:       120000 (1000/s)       ← Total generated (rate)
║ Addresses:       7200000 (60000/s)     ← Total derived (rate)
║ Checked:         7200000                ← Addresses checked in DB
```

## Common Issues

### "Database not found"
```bash
# Check path
ls -l btc_addresses.db

# Use absolute path
./target/release/btc_hunt -d /full/path/to/btc_addresses.db
```

### "Too slow"
```bash
# Check if database has index
sqlite3 btc_addresses.db "PRAGMA index_list('addresses');"

# Create index if missing
sqlite3 btc_addresses.db "CREATE INDEX idx_address ON addresses(address);"
```

### "Out of memory"
```bash
# Reduce batch size
./target/release/btc_hunt -d btc_addresses.db -b 100 -a 5
```

## What's Different from Your Old Setup?

### Old Way (3 programs + bash script)
```bash
# Step 1: Generate seeds to file
./bip39_generator --count 50000 --output seeds.txt

# Step 2: Derive addresses to file  
./btc-key-deriver --input seeds.txt --output addressonly.txt

# Step 3: Check addresses against DB
./btc_address_checker btc_addresses.db addressonly.txt
```

**Problems:**
- Slow file I/O between each step
- No parallelism in generation
- One DB query per address
- Manual bash script management

### New Way (1 unified program)
```bash
./target/release/btc_hunt -d btc_addresses.db -b 50000
```

**Benefits:**
- ✅ Everything in memory (no files)
- ✅ Parallel generation and derivation
- ✅ Batched DB queries (1000x faster)
- ✅ Real-time statistics
- ✅ Stops immediately on match
- ✅ Single binary to manage

## Windows Users

```powershell
# Build
cd btc_hunt
cargo build --release

# Run
.\target\release\btc_hunt.exe -d btc_addresses.db -b 5000
```

## Linux/Mac Users

```bash
# Build
cd btc_hunt
cargo build --release

# Run
./target/release/btc_hunt -d btc_addresses.db -b 5000
```

## Next Steps

1. **Build the project**: `cargo build --release`
2. **Prepare your database**: Ensure it has an index on `address` column
3. **Start hunting**: Run with your preferred settings
4. **Monitor**: Watch the statistics to tune performance
5. **Enjoy**: It's much faster than the old setup! 🚀

Happy hunting! 🎯



