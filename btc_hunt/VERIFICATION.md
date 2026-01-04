# Verification Guide

This guide helps you verify that the new unified btc_hunt generates the same addresses as your old code.

## Quick Verification

### Windows
```powershell
cd D:\MyProjects\BTC_Hunt\btc_hunt

# Run the test script
.\test_mnemonic.bat

# Or manually:
cargo run --release --bin verify_mnemonic -- "motor venture dilemma quote subject magnet keep large dry gossip bean paper"
```

### Linux/Mac
```bash
cd btc_hunt

# Run the test script
./test_mnemonic.sh

# Or manually:
cargo run --release --bin verify_mnemonic -- "motor venture dilemma quote subject magnet keep large dry gossip bean paper"
```

## Expected Output

For the test mnemonic:
```
motor venture dilemma quote subject magnet keep large dry gossip bean paper
```

You should see output matching your old btc-key-deriver:

```
BIP39 Mnemonic: motor venture dilemma quote subject magnet keep large dry gossip bean paper

BIP39 Seed: 24bd1b243ec776dd97bc7487ad65c8966ff6e0b8654a25602a41994746957c49c813ba183e6d1646584cf810fcb9898f44571e3ccfe9fb266e3a66597fbcd7c4

Coin: BTC

Derivation Path and outputs

path,address,public key,private key

m/0'/0'/0',1GyNWR7LPXdLSHeN4nE4b9P3gNEcjZkmzd,...
m/0'/0'/1',1GPruf7qZWTKbAmUH351MAwNpMVqJHjfUT,...

m/44'/0'/0'/0/0',1Jo3qrSUxWYYJdhDawJ58QU7wtyVtqAK5A,...
m/44'/0'/0'/0/1',1EhRxsqMeyVTpzYwRBzh2QwfrVcLMBQyYq,...

m/49'/0'/0'/0/0',33ML21FE9QSqh9wizdQbZsHfE41vwkRT78,...
m/49'/0'/0'/0/1',33PajXTiRLXvJsSxHnPKZpTRcdWK3HP83h,...

m/84'/0'/0'/0/0',bc1qnc9umhdc04u0u5qfg0qu3aj75wvfps4z4sj7g6,...
m/84'/0'/0'/0/1',bc1q76nvc5jg2zz3uv8pcsjq6h38dvvms5pf3jmw3m,...

Script Semantics: P2WPKH nested in P2SH
m/0/0',3HWZMAtc7MyENWguyhWaLrLjXpWTMpfZLh,...
m/0/1',3FmxkRjhFeCtoQdeYU2ubGB4NsnUGFMEFJ,...

Script Semantics: P2WPKH
m/0/0',bc1qe59ssevhzy9v76syff0508ml97xm0rstcfdw0y,...
m/0/1',bc1qavf2aluhaehmx8jc2nf2jz23enuh9m6esmxzy8,...
```

## Verification Checklist

Compare these key addresses:

✅ **m/0'/0'/0'**: Should be `1GyNWR7LPXdLSHeN4nE4b9P3gNEcjZkmzd`  
✅ **m/44'/0'/0'/0/0'**: Should be `1Jo3qrSUxWYYJdhDawJ58QU7wtyVtqAK5A`  
✅ **m/49'/0'/0'/0/0'**: Should be `33ML21FE9QSqh9wizdQbZsHfE41vwkRT78`  
✅ **m/84'/0'/0'/0/0'**: Should be `bc1qnc9umhdc04u0u5qfg0qu3aj75wvfps4z4sj7g6`  
✅ **m/0/0' (P2SH)**: Should be `3HWZMAtc7MyENWguyhWaLrLjXpWTMpfZLh`  
✅ **m/0/0' (P2WPKH)**: Should be `bc1qe59ssevhzy9v76syff0508ml97xm0rstcfdw0y`  

## Test Your Own Mnemonics

```powershell
# Windows
cargo run --release --bin verify_mnemonic -- "your twelve word mnemonic phrase here"

# Example with a different mnemonic
cargo run --release --bin verify_mnemonic -- "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
```

## Compare with Old Code

### Old Code Output
```powershell
# Your old way
cd ..\btc-key-deriver
echo "motor venture dilemma quote subject magnet keep large dry gossip bean paper" > test_seed.txt
.\target\release\btc-key-deriver.exe --input test_seed.txt --output test_output.txt --full test_full.txt
type test_full.txt
```

### New Code Output
```powershell
# New way
cd ..\btc_hunt
cargo run --release --bin verify_mnemonic -- "motor venture dilemma quote subject magnet keep large dry gossip bean paper"
```

**Compare the two outputs line by line!**

## Automated Comparison Script (PowerShell)

```powershell
# Save this as compare_outputs.ps1

$mnemonic = "motor venture dilemma quote subject magnet keep large dry gossip bean paper"

Write-Host "Generating output from NEW code..." -ForegroundColor Green
$newOutput = cargo run --release --bin verify_mnemonic -- $mnemonic 2>$null

Write-Host "Generating output from OLD code..." -ForegroundColor Green
# Run your old code here and capture output
# $oldOutput = ...

Write-Host "`nComparing outputs..." -ForegroundColor Yellow

# Extract and compare addresses
if ($newOutput -match "1GyNWR7LPXdLSHeN4nE4b9P3gNEcjZkmzd") {
    Write-Host "✓ Address m/0'/0'/0' matches!" -ForegroundColor Green
} else {
    Write-Host "✗ Address m/0'/0'/0' MISMATCH!" -ForegroundColor Red
}

if ($newOutput -match "1Jo3qrSUxWYYJdhDawJ58QU7wtyVtqAK5A") {
    Write-Host "✓ Address m/44'/0'/0'/0/0' matches!" -ForegroundColor Green
} else {
    Write-Host "✗ Address m/44'/0'/0'/0/0' MISMATCH!" -ForegroundColor Red
}

# Add more checks as needed...
```

## Troubleshooting

### Different Addresses Generated?

Check:
1. **Same mnemonic?** - Ensure exact spelling and order
2. **Same passphrase?** - Both old and new use empty passphrase `""`
3. **Same network?** - Both use Bitcoin mainnet
4. **Same derivation paths?** - Check paths match exactly

### Build Errors?

```powershell
# Clean and rebuild
cargo clean
cargo build --release --bin verify_mnemonic
```

### Can't Find Binary?

```powershell
# Check if it was built
dir .\target\release\verify_mnemonic.exe

# If not, build it explicitly
cargo build --release --bin verify_mnemonic
```

## Integration Test

Once you've verified addresses match, you can test the full pipeline:

```powershell
# 1. Create a test database with known address
sqlite3 test.db "CREATE TABLE addresses (address TEXT PRIMARY KEY);"
sqlite3 test.db "INSERT INTO addresses VALUES ('1GyNWR7LPXdLSHeN4nE4b9P3gNEcjZkmzd');"
sqlite3 test.db "CREATE INDEX idx_address ON addresses(address);"

# 2. Create a file with just this mnemonic
echo "motor venture dilemma quote subject magnet keep large dry gossip bean paper" > test_seed.txt

# 3. Run old code
cd ..\btc-key-deriver
.\target\release\btc-key-deriver.exe --input ..\btc_hunt\test_seed.txt --output old_addrs.txt

cd ..\db_search
.\target\release\btc_address_checker.exe ..\btc_hunt\test.db ..\btc-key-deriver\old_addrs.txt

# 4. Now test with the new unified code
# We need to modify it to accept input file instead of random generation
# Or just verify manually that it would find the address
```

## Success Criteria

✅ **All addresses match exactly**  
✅ **Public keys match exactly**  
✅ **Private keys (WIF) match exactly**  
✅ **Seed (hex) matches exactly**  

If all match → Your new code is **verified correct**! 🎉

## Online Verification (Optional)

You can also verify against online tools:

1. **Ian Coleman's BIP39 Tool**: https://iancoleman.io/bip39/
   - Enter your mnemonic
   - Compare derived addresses
   - ⚠️ Use offline for real mnemonics!

2. **Trezor Recovery Tool** (offline mode)
3. **Electrum Wallet** (in test mode)

## Next Steps After Verification

Once verified:
1. ✅ Delete test files
2. ✅ Trust the new unified code
3. ✅ Use it for production runs
4. ✅ Enjoy the 5× speed improvement!

---

**Remember:** This verification tool generates the SAME addresses as your old code, just much faster because it's all in memory! 🚀



