@echo off
REM Build script for BTC Hunt (Windows)

echo ================================================================
echo               Building BTC Hunt v2.0
echo ================================================================
echo.

REM Check if cargo is installed
where cargo >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo [ERROR] Cargo not found. Please install Rust from https://rustup.rs/
    exit /b 1
)

echo [OK] Cargo found
cargo --version
echo.

REM Check if we're in the right directory
if not exist "Cargo.toml" (
    echo [ERROR] Cargo.toml not found. Please run this script from the btc_hunt directory.
    exit /b 1
)

echo Building in release mode (optimized)...
echo This may take 2-5 minutes on first build...
echo.

cargo build --release

if %ERRORLEVEL% EQU 0 (
    echo.
    echo ================================================================
    echo                  BUILD SUCCESSFUL!
    echo ================================================================
    echo.
    echo Binary location:
    echo   target\release\btc_hunt.exe
    echo.
    echo Run with:
    echo   .\target\release\btc_hunt.exe --help
    echo   .\target\release\btc_hunt.exe -d btc_addresses.db -b 1000
    echo.
    echo Documentation:
    echo   - QUICK_START.md     (Get started in 5 minutes^)
    echo   - README.md          (Full documentation^)
    echo   - MIGRATION_GUIDE.md (Transition from old setup^)
    echo.
) else (
    echo.
    echo [ERROR] Build failed. Check error messages above.
    exit /b 1
)



