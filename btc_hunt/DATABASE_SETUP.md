# Database Setup Guide

Your Bitcoin address database must follow this schema.

## Required Schema

```sql
CREATE TABLE btc_addresses (
    address TEXT PRIMARY KEY,
    balance REAL,
    address_type TEXT,
    last_updated INTEGER
);

CREATE INDEX idx_address ON btc_addresses(address);
```

**Note:** btc_hunt only uses the `address` column. Other columns are ignored.

## Quick Setup

### Option 1: Create Test Database (PowerShell)

```powershell
cd btc_hunt
.\create_test_db.ps1
```

This creates `btc_addresses.db` with sample addresses for testing.

### Option 2: Create Test Database (SQL)

```powershell
cd btc_hunt
sqlite3 ..\btc_addresses.db < create_test_db.sql
```

### Option 3: Create Empty Database

```powershell
sqlite3 ..\btc_addresses.db
```

Then paste:
```sql
CREATE TABLE btc_addresses (
    address TEXT PRIMARY KEY,
    balance REAL,
    address_type TEXT,
    last_updated INTEGER
);

CREATE INDEX idx_address ON btc_addresses(address);

.quit
```

### Option 4: Import Your Own Addresses

If you have a text file with addresses (one per line):

```powershell
# Create database and table
sqlite3 btc_addresses.db "CREATE TABLE addresses (address TEXT PRIMARY KEY);"

# Import addresses
sqlite3 btc_addresses.db ".mode csv" ".import addresses.txt addresses"

# Create index (IMPORTANT!)
sqlite3 btc_addresses.db "CREATE INDEX idx_address ON addresses(address);"
```

## Verify Database

Check your database structure:

```powershell
# Check tables
sqlite3 btc_addresses.db ".tables"

# Check schema
sqlite3 btc_addresses.db ".schema"

# Check count
sqlite3 btc_addresses.db "SELECT COUNT(*) FROM addresses;"

# Check index
sqlite3 btc_addresses.db "PRAGMA index_list('addresses');"

# Sample addresses
sqlite3 btc_addresses.db "SELECT * FROM addresses LIMIT 5;"
```

## Test with btc_hunt

```powershell
# Small test
.\target\release\btc_hunt.exe -d ..\btc_addresses.db -b 100 -n 1

# If database is in btc_hunt folder:
.\target\release\btc_hunt.exe -d btc_addresses.db -b 100 -n 1
```

## Common Issues

### "no such table: btc_addresses"

**Problem:** Database exists but doesn't have the `btc_addresses` table.

**Solution:**
```powershell
sqlite3 btc_addresses.db "CREATE TABLE btc_addresses (address TEXT PRIMARY KEY, balance REAL, address_type TEXT, last_updated INTEGER);"
sqlite3 btc_addresses.db "CREATE INDEX idx_address ON btc_addresses(address);"
```

### "unable to open database file"

**Problem:** Database file doesn't exist.

**Solution:**
```powershell
# Run the create script
.\create_test_db.ps1

# Or create manually
sqlite3 btc_addresses.db < create_test_db.sql
```

### Database is empty

**Problem:** Table exists but no addresses in it.

**Solution:**
```powershell
# Add test addresses
sqlite3 btc_addresses.db "INSERT INTO addresses VALUES ('1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa');"

# Or import from file
sqlite3 btc_addresses.db ".mode csv" ".import your_addresses.txt addresses"
```

### Slow performance

**Problem:** No index on btc_addresses table.

**Solution:**
```powershell
# Check if index exists
sqlite3 btc_addresses.db "PRAGMA index_list('btc_addresses');"

# Create index (this is critical!)
sqlite3 btc_addresses.db "CREATE INDEX IF NOT EXISTS idx_address ON btc_addresses(address);"
```

## Production Database

For production use with millions of addresses:

1. **Create table:**
   ```sql
   CREATE TABLE addresses (address TEXT PRIMARY KEY);
   ```

2. **Import addresses:**
   ```bash
   sqlite3 btc_addresses.db ".mode csv" ".import large_addresses.txt addresses"
   ```

3. **Create index (CRITICAL!):**
   ```sql
   CREATE INDEX idx_address ON addresses(address);
   ```

4. **Optimize:**
   ```sql
   VACUUM;
   ANALYZE;
   ```

5. **Verify:**
   ```sql
   SELECT COUNT(*) FROM addresses;
   PRAGMA index_list('addresses');
   ```

## Database Location

btc_hunt looks for database at path specified by `-d`:

```powershell
# Relative path (from btc_hunt folder)
.\target\release\btc_hunt.exe -d ..\btc_addresses.db

# Absolute path
.\target\release\btc_hunt.exe -d "D:\MyProjects\BTC_Hunt\btc_addresses.db"

# Same directory
.\target\release\btc_hunt.exe -d btc_addresses.db
```

## Example: Complete Setup

```powershell
# 1. Navigate to project
cd D:\MyProjects\BTC_Hunt\btc_hunt

# 2. Create test database
.\create_test_db.ps1

# 3. Verify it works
sqlite3 ..\btc_addresses.db "SELECT COUNT(*) FROM addresses;"

# 4. Test with btc_hunt
.\target\release\btc_hunt.exe -d ..\btc_addresses.db -b 100 -n 1

# 5. If successful, run production
.\target\release\btc_hunt.exe -d ..\btc_addresses.db -b 5000 --log-stats
```

## Summary

✅ **Required:**
- Table named `btc_addresses`
- Column named `address` (TEXT)
- Index named `idx_address` on `address` column
- Optional columns: `balance`, `address_type`, `last_updated` (ignored by btc_hunt)

✅ **Quick test:**
```powershell
.\create_test_db.ps1
.\target\release\btc_hunt.exe -d ..\btc_addresses.db -b 100 -n 1
```

✅ **Verify:**
```powershell
sqlite3 btc_addresses.db ".schema"
```

Should show:
```
CREATE TABLE btc_addresses (
    address TEXT PRIMARY KEY,
    balance REAL,
    address_type TEXT,
    last_updated INTEGER
);
CREATE INDEX idx_address ON btc_addresses(address);
```

