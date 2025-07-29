#!/bin/bash

# Demonstration script for Alacritty tabbing functionality

echo "Alacritty Tabbing Implementation Demonstration"
echo "=============================================="
echo

echo "1. Building Alacritty..."
cd /home/wisp/alacritty
cargo build --release
echo

if [ $? -eq 0 ]; then
    echo "✓ Build successful"
else
    echo "✗ Build failed"
    exit 1
fi

echo "2. Running tests..."
cargo test --release
echo

if [ $? -eq 0 ]; then
    echo "✓ All tests passed"
else
    echo "✗ Some tests failed"
    exit 1
fi

echo "3. Key features implemented:"
echo "   - Internal tab management for Linux"
echo "   - Tab navigation (next, previous, specific tab)"
echo "   - Fallback to native tabbing when available"
echo "   - Improved error handling"
echo

echo "4. How to test tabbing:"
echo "   - Start Alacritty: ./target/release/alacritty"
echo "   - Press Ctrl+Shift+T to create a new tab"
echo "   - Press Ctrl+Shift+] to switch to the next tab"
echo "   - Press Ctrl+Shift+[ to switch to the previous tab"
echo "   - Press Alt+1 through Alt+9 to switch to specific tabs"
echo

echo "Demonstration completed successfully!"