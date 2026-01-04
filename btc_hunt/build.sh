#!/usr/bin/env bash
# Build script for BTC Hunt

set -e

echo "╔════════════════════════════════════════════════════════════╗"
echo "║              Building BTC Hunt v2.0                        ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo not found. Please install Rust from https://rustup.rs/"
    exit 1
fi

echo "✓ Cargo found: $(cargo --version)"
echo ""

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Cargo.toml not found. Please run this script from the btc_hunt directory."
    exit 1
fi

echo "📦 Building in release mode (optimized)..."
echo "   This may take 2-5 minutes on first build..."
echo ""

cargo build --release

if [ $? -eq 0 ]; then
    echo ""
    echo "╔════════════════════════════════════════════════════════════╗"
    echo "║                 ✅ BUILD SUCCESSFUL!                       ║"
    echo "╚════════════════════════════════════════════════════════════╝"
    echo ""
    echo "Binary location:"
    
    if [ -f "target/release/btc_hunt" ]; then
        echo "  📍 target/release/btc_hunt"
        echo ""
        echo "Run with:"
        echo "  ./target/release/btc_hunt --help"
        echo "  ./target/release/btc_hunt -d btc_addresses.db -b 1000"
    elif [ -f "target/release/btc_hunt.exe" ]; then
        echo "  📍 target/release/btc_hunt.exe"
        echo ""
        echo "Run with:"
        echo "  .\\target\\release\\btc_hunt.exe --help"
        echo "  .\\target\\release\\btc_hunt.exe -d btc_addresses.db -b 1000"
    fi
    
    echo ""
    echo "📖 Documentation:"
    echo "  - QUICK_START.md     (Get started in 5 minutes)"
    echo "  - README.md          (Full documentation)"
    echo "  - MIGRATION_GUIDE.md (Transition from old setup)"
    echo ""
else
    echo ""
    echo "❌ Build failed. Check error messages above."
    exit 1
fi

