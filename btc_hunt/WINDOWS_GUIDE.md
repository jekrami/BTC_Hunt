# Windows 11 Quick Start Guide

This guide is specifically for Windows 11 users. All commands are PowerShell-ready.

## 🚀 Quick Start (Windows 11)

### Step 1: Verify Rust Installation

```powershell
# Check if Rust is installed
cargo --version

# If not installed, download from: https://rustup.rs/
# Or use this command:
winget install Rustlang.Rustup
```

### Step 2: Build the Project

```powershell
cd D:\MyProjects\BTC_Hunt\btc_hunt

# Option 1: Use the build script
.\build.bat

# Option 2: Build directly
cargo build --release
```

Build time: 2-5 minutes on first build (downloads dependencies)

### Step 3: Verify Database

```powershell
# Check if your database exists
dir ..\db_search\btc_addresses.db

# Or use absolute path
dir D:\MyProjects\BTC_Hunt\db_search\btc_addresses.db

# Verify database has index (important for performance!)
sqlite3 ..\db_search\btc_addresses.db "PRAGMA index_list('addresses');"
```

If no index exists, create one:
```powershell
sqlite3 ..\db_search\btc_addresses.db "CREATE INDEX IF NOT EXISTS idx_address ON addresses(address);"
```

### Step 4: Run a Test

```powershell
# Small test: 100 mnemonics × 10 batches = 1,000 mnemonics
.\target\release\btc_hunt.exe -d ..\db_search\btc_addresses.db -b 100 -n 10
```

### Step 5: Production Run

```powershell
# Replicate your old bash script (50,000 mnemonics per batch, unlimited)
.\target\release\btc_hunt.exe -d ..\db_search\btc_addresses.db -b 50000

# Or with maximum performance (10,000 per batch, auto threads)
.\target\release\btc_hunt.exe -d ..\db_search\btc_addresses.db -b 10000 -t 0
```

## 📁 File Paths (Windows)

### Using Relative Paths
```powershell
# From btc_hunt directory
.\target\release\btc_hunt.exe -d ..\db_search\btc_addresses.db
```

### Using Absolute Paths
```powershell
# Full Windows path
.\target\release\btc_hunt.exe -d "D:\MyProjects\BTC_Hunt\db_search\btc_addresses.db"
```

### Output Directory
```powershell
# Default (creates 'results' folder in current directory)
.\target\release\btc_hunt.exe -d ..\db_search\btc_addresses.db

# Custom output location
.\target\release\btc_hunt.exe -d ..\db_search\btc_addresses.db -o "D:\BTC_Results"
```

## 🎯 Common Windows Commands

### Check if Process is Running
```powershell
# See if btc_hunt is running
Get-Process | Where-Object {$_.ProcessName -like "*btc_hunt*"}

# Stop it if needed
Stop-Process -Name "btc_hunt" -Force
```

### Run in Background

**Option 1: PowerShell Job**
```powershell
Start-Job -ScriptBlock {
    Set-Location "D:\MyProjects\BTC_Hunt\btc_hunt"
    .\target\release\btc_hunt.exe -d ..\db_search\btc_addresses.db -b 50000
}

# Check job status
Get-Job

# Get output
Receive-Job -Id 1 -Keep
```

**Option 2: Start-Process (Separate Window)**
```powershell
Start-Process powershell -ArgumentList "-NoExit", "-Command", "cd D:\MyProjects\BTC_Hunt\btc_hunt; .\target\release\btc_hunt.exe -d ..\db_search\btc_addresses.db -b 50000"
```

**Option 3: Redirect to Log File**
```powershell
.\target\release\btc_hunt.exe -d ..\db_search\btc_addresses.db -b 50000 > hunt.log 2>&1
```

### Monitor Performance

**PowerShell (Real-time CPU/Memory)**
```powershell
# In a separate PowerShell window
while($true) {
    Get-Process btc_hunt | Select-Object Name, CPU, 
        @{Name="Memory(MB)";Expression={[math]::Round($_.WorkingSet / 1MB, 2)}},
        @{Name="Threads";Expression={$_.Threads.Count}}
    Start-Sleep -Seconds 2
    Clear-Host
}
```

**Task Manager**
- Press `Ctrl+Shift+Esc`
- Find `btc_hunt.exe`
- Watch CPU, Memory, Disk usage

## 🔧 Performance Optimization (Windows 11)

### 1. Windows Defender Exclusion
Add exclusion to improve build and run performance:

```powershell
# Run as Administrator
Add-MpPreference -ExclusionPath "D:\MyProjects\BTC_Hunt"
Add-MpPreference -ExclusionProcess "btc_hunt.exe"
Add-MpPreference -ExclusionProcess "cargo.exe"
```

### 2. Power Settings
```powershell
# Set to High Performance mode
powercfg /setactive 8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c
```

Or manually:
- Settings → System → Power & battery
- Power mode: **Best performance**

### 3. Close Unnecessary Apps
```powershell
# Stop Windows Search indexing temporarily
Stop-Service -Name "WSearch" -Force

# Restart it later
Start-Service -Name "WSearch"
```

### 4. Use SSD for Database
Ensure your database is on an SSD (C: or NVMe drive), not HDD.

```powershell
# Check drive type
Get-PhysicalDisk | Select-Object DeviceID, MediaType, FriendlyName
```

### 5. Increase Priority (Optional)
```powershell
# Run btc_hunt, then in another PowerShell:
Get-Process btc_hunt | ForEach-Object { $_.PriorityClass = "High" }
```

## 🎨 Windows Terminal Settings

For better experience, use Windows Terminal:

```powershell
# Install Windows Terminal (if not already installed)
winget install Microsoft.WindowsTerminal
```

**Recommended settings:**
- Profile → Appearance → Font size: 12
- Profile → Appearance → Font face: Cascadia Code
- Profile → Advanced → Text antialiasing: ClearType

## 📊 Example Output (Windows)

When running, you'll see:
```
╔════════════════════════════════════════════════════════════╗
║              Bitcoin Address Hunter v2.0                   ║
║          Unified Mnemonic → Address → Database             ║
╚════════════════════════════════════════════════════════════╝

Configuration:
  • Database:           ..\db_search\btc_addresses.db
  • Batch size:         50000 mnemonics
  • Max batches:        unlimited
  • Addresses/path:     10
  • Worker threads:     16
  • DB batch size:      1000

Starting hunt...

╔════════════════════════════════════════════════════════════╗
║                    BTC HUNT STATISTICS                     ║
╠════════════════════════════════════════════════════════════╣
║ Runtime:         120 seconds
║ Batches:         24
║ Mnemonics:       1200000 (10000/s)
║ Addresses:       72000000 (600000/s)
║ Checked:         72000000
╚════════════════════════════════════════════════════════════╝
```

## 🛠️ Troubleshooting (Windows-Specific)

### "Cannot find btc_hunt.exe"
```powershell
# Make sure you're in the right directory
cd D:\MyProjects\BTC_Hunt\btc_hunt

# Check if binary exists
dir .\target\release\btc_hunt.exe

# If not, rebuild
cargo build --release
```

### "Database is locked"
```powershell
# Check what's using the database
handle.exe btc_addresses.db

# Or close all database connections
taskkill /F /IM sqlite3.exe
```

### "Access Denied"
```powershell
# Run PowerShell as Administrator, or
# Check file permissions
icacls .\target\release\btc_hunt.exe
```

### "Slow Performance"
1. **Check CPU usage** (Task Manager)
2. **Check if database has index:**
   ```powershell
   sqlite3 ..\db_search\btc_addresses.db "PRAGMA index_list('addresses');"
   ```
3. **Try different thread counts:**
   ```powershell
   # Auto-detect
   .\target\release\btc_hunt.exe -d ..\db_search\btc_addresses.db -t 0
   
   # Manual (e.g., 8 threads)
   .\target\release\btc_hunt.exe -d ..\db_search\btc_addresses.db -t 8
   ```

### "Out of Memory"
```powershell
# Reduce batch size
.\target\release\btc_hunt.exe -d ..\db_search\btc_addresses.db -b 500 -a 5

# Check available RAM
Get-CimInstance Win32_OperatingSystem | 
    Select-Object @{Name="Free RAM (GB)";Expression={[math]::Round($_.FreePhysicalMemory / 1MB, 2)}}
```

### Build Errors
```powershell
# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build --release

# If still issues, check Visual Studio Build Tools
# Download from: https://visualstudio.microsoft.com/downloads/
# Install: "Desktop development with C++"
```

## 📝 Scheduled Task (Run Automatically)

Create a scheduled task to run btc_hunt automatically:

```powershell
# Create a scheduled task that runs daily at 2 AM
$action = New-ScheduledTaskAction -Execute "PowerShell.exe" `
    -Argument "-WindowStyle Hidden -Command `"cd D:\MyProjects\BTC_Hunt\btc_hunt; .\target\release\btc_hunt.exe -d ..\db_search\btc_addresses.db -b 50000 > daily_hunt.log 2>&1`""

$trigger = New-ScheduledTaskTrigger -Daily -At 2am

$settings = New-ScheduledTaskSettingsSet -AllowStartIfOnBatteries -DontStopIfGoingOnBatteries

Register-ScheduledTask -TaskName "BTC Hunt" -Action $action -Trigger $trigger -Settings $settings
```

View/manage tasks:
```powershell
# List tasks
Get-ScheduledTask | Where-Object {$_.TaskName -like "*BTC*"}

# Remove task
Unregister-ScheduledTask -TaskName "BTC Hunt" -Confirm:$false
```

## 🎯 Quick Commands Cheat Sheet

```powershell
# Navigate to project
cd D:\MyProjects\BTC_Hunt\btc_hunt

# Build
cargo build --release

# Test run (1,000 mnemonics)
.\target\release\btc_hunt.exe -d ..\db_search\btc_addresses.db -b 100 -n 10

# Production run (unlimited)
.\target\release\btc_hunt.exe -d ..\db_search\btc_addresses.db -b 50000

# Maximum performance
.\target\release\btc_hunt.exe -d ..\db_search\btc_addresses.db -b 10000 -t 0

# Low memory
.\target\release\btc_hunt.exe -d ..\db_search\btc_addresses.db -b 500 -a 5

# Background with log
Start-Process powershell -ArgumentList "-Command", "cd D:\MyProjects\BTC_Hunt\btc_hunt; .\target\release\btc_hunt.exe -d ..\db_search\btc_addresses.db -b 50000" -RedirectStandardOutput "hunt.log" -NoNewWindow

# Check if running
Get-Process btc_hunt -ErrorAction SilentlyContinue

# Stop if running
Stop-Process -Name btc_hunt -Force -ErrorAction SilentlyContinue

# View log
Get-Content .\hunt.log -Tail 20 -Wait
```

## 💻 System Requirements (Windows 11)

**Minimum:**
- Windows 11 (any edition)
- 4 CPU cores
- 8 GB RAM
- 5 GB free disk space
- Rust 1.70+

**Recommended:**
- Windows 11 Pro/Enterprise
- 8+ CPU cores
- 16 GB+ RAM
- NVMe SSD with 10+ GB free
- Rust 1.75+

**Optimal:**
- Windows 11 Pro
- 16+ CPU cores
- 32 GB+ RAM
- Fast NVMe SSD
- Dedicated machine (no other heavy apps running)

## 🔒 Security (Windows 11)

1. **Use BitLocker** for full-disk encryption
   ```powershell
   # Check if BitLocker is enabled
   Get-BitLockerVolume
   ```

2. **Encrypt report files immediately**
   ```powershell
   # After a match is found
   Compress-Archive -Path .\results\MATCH_FOUND_*.txt -DestinationPath .\results\secure_backup.zip
   # Then delete originals
   Remove-Item .\results\MATCH_FOUND_*.txt
   ```

3. **Secure delete** (use SDelete from Sysinternals)
   ```powershell
   # Download SDelete first
   sdelete64.exe -p 3 .\results\MATCH_FOUND_*.txt
   ```

## 🎓 Tips for Windows 11 Users

1. **Pin PowerShell to Taskbar** for quick access
2. **Use Windows Terminal** instead of old PowerShell console
3. **Create a shortcut:**
   - Right-click Desktop → New → Shortcut
   - Target: `powershell.exe -Command "cd D:\MyProjects\BTC_Hunt\btc_hunt; .\target\release\btc_hunt.exe -d ..\db_search\btc_addresses.db -b 50000"`
   - Name: "BTC Hunt"

4. **Monitor with Desktop Widgets:**
   - Use Windows 11 Widgets to monitor CPU/RAM
   - Install "Performance Monitor" widget

5. **Backup Strategy:**
   ```powershell
   # Backup entire project
   Compress-Archive -Path "D:\MyProjects\BTC_Hunt\btc_hunt" -DestinationPath "D:\Backups\btc_hunt_$(Get-Date -Format 'yyyyMMdd').zip"
   ```

## 📞 Getting Help

If you encounter issues:

1. **Check build output:** Look for errors during `cargo build`
2. **Check Windows Event Viewer:** Applications and Services Logs
3. **Enable verbose output:**
   ```powershell
   $env:RUST_BACKTRACE=1
   .\target\release\btc_hunt.exe -d ..\db_search\btc_addresses.db -b 100 -n 1
   ```

## 🎉 You're All Set!

Your Windows 11 machine is perfect for this project. The unified btc_hunt runs great on Windows and you have all the tools you need.

**Quick start:**
```powershell
cd D:\MyProjects\BTC_Hunt\btc_hunt
.\target\release\btc_hunt.exe -d ..\db_search\btc_addresses.db -b 50000
```

Happy hunting on Windows 11! 🚀



