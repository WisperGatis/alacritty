#!/bin/bash

# Test script for Alacritty tabbing functionality
echo "Testing Alacritty tabbing implementation..."

# Build the project
echo "Building Alacritty..."
cd /home/wisp/alacritty
cargo build

if [ $? -eq 0 ]; then
    echo "Build successful!"
else
    echo "Build failed!"
    exit 1
fi

# Run tests
echo "Running tests..."
cargo test

if [ $? -eq 0 ]; then
    echo "All tests passed!"
else
    echo "Some tests failed!"
    exit 1
fi

echo "Tabbing implementation test completed successfully!"