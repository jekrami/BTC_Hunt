# BTC Hunt v2.1.0 - Release Summary

## 🎉 What's New

### Version Info at Startup

**Problem:** When running on remote servers, it was hard to know which version was running.

**Solution:** Added a single-line version output at startup:

```
BTC Hunt v2.1.0 starting...
```

**Features:**
- ✅ Always visible (even in `--simple` mode)
- ✅ Shows on the first line of output
- ✅ Automatically updates from `Cargo.toml` version
- ✅ No need to check files or run `--version` flag

**Example output:**

**Normal mode:**
```
BTC Hunt v2.1.0 starting...
╔════════════════════════════════════════════════════════════╗
║              Bitcoin Address Hunter v2.1.0                  ║
║          Unified Mnemonic → Address → Database             ║
╚════════════════════════════════════════════════════════════╝
...
```

**Simple mode (for servers):**
```
BTC Hunt v2.1.0 starting...
[2026-01-06 12:34:56] DB: btc_addresses.db | Batch: 10000 | Threads: 8 | Log: enabled
...
```

**Checking version:**
```bash
# Method 1: Run with --version flag
./btc_hunt --version
# Output: btc_hunt 2.1.0

# Method 2: Just start it and look at first line
./btc_hunt --simple ...
# Output: BTC Hunt v2.1.0 starting...
```

## 📦 Upgrade Instructions

### On Your Development Machine (Windows):

```powershell
cd D:\MyProjects\BTC_Hunt\btc_hunt
cargo build --release
```

### On Your Server (Linux):

```bash
cd /path/to/btc_hunt

# Pull latest code or copy updated files
# Then rebuild
cargo build --release

# Test it
./target/release/btc_hunt --version
```

**That's it!** The version is automatically compiled into the binary.

## 🔍 How to Check Version on Running Server

### If using `screen`:

```bash
# Attach to screen session
screen -r btc_hunt

# Look at the top of the output - version is on the first line!
```

### If using logs:

```bash
# Check the log file for version at startup
head -1 /path/to/your/btc_hunt.log
```

### If using systemd:

```bash
# Check the journal for startup message
journalctl -u btc_hunt -n 100 | head -5
```

## 📝 Technical Details

### Code Changes:

**In `Cargo.toml`:**
```toml
version = "2.1.0"  # Changed from 2.0.0
```

**In `src/main.rs`:**
```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    // Print version info at startup (always, even in simple mode)
    eprintln!("BTC Hunt v{} starting...", env!("CARGO_PKG_VERSION"));
    
    // ... rest of code
}
```

**Benefits:**
- Version is defined in ONE place (`Cargo.toml`)
- Automatically displayed on startup
- Automatically used in `--version` flag
- No manual updating of hardcoded versions

## 🎯 Why This Matters

When you have multiple servers running BTC Hunt:
- ✅ Instantly see which version is running
- ✅ Verify updates were applied
- ✅ Troubleshoot version-specific issues
- ✅ Track which server needs updating

**Before:**
```bash
# Had to check multiple things:
cat /path/to/Cargo.toml | grep version
./btc_hunt --version  # (if you could stop it)
# Or guess from features/behavior
```

**Now:**
```bash
# Just look at the first line of output!
BTC Hunt v2.1.0 starting...
```

## 📊 Complete Version 2.1.0 Changes

See `CHANGELOG.md` for full details, but highlights include:

1. ✅ Version info at startup (this feature)
2. ✅ Daily log rotation system
3. ✅ Simplified daily reports
4. ✅ Fixed cron timing (23:59 → 00:05)
5. ✅ Fixed log directory in scripts
6. ✅ Enhanced documentation

## 🚀 Next Steps

1. **Rebuild on all servers** where BTC Hunt is running
2. **Verify version** by checking first line of output
3. **Update cron jobs** if needed (see `FIXES_APPLIED.md`)
4. **Enjoy** knowing exactly which version is running!

---

**Questions?** Check the documentation:
- `CHANGELOG.md` - All changes
- `LOG_ROTATION_GUIDE.md` - New log system
- `FIXES_APPLIED.md` - Bug fixes

**Version:** 2.1.0  
**Date:** 2026-01-06  
**Status:** ✅ Ready for production

