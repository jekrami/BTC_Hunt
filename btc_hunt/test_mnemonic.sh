#!/usr/bin/env bash
# Test script to verify mnemonic derivation

set -e

echo "Building verification tool..."
cargo build --release --bin verify_mnemonic

echo ""
echo "Running test with your provided mnemonic..."
echo ""

./target/release/verify_mnemonic "motor venture dilemma quote subject magnet keep large dry gossip bean paper"

echo ""
echo "================================================================"
echo "Compare the output above with your expected results!"
echo "================================================================"



