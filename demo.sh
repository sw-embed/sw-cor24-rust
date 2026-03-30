#!/bin/bash
# COR24 LED Demo Script
# Usage: ./demo.sh           # Run built-in demo
#        ./demo.sh file.s    # Assemble and run custom file

cd "$(dirname "$0")"

# Build if needed
if [ ! -f target/release/cor24-run ]; then
    echo "Building cor24-run..."
    cargo build --release --bin cor24-run 2>/dev/null
fi

# Run
if [ "$1" ]; then
    ./target/release/cor24-run "$1"
else
    ./target/release/cor24-run --demo
fi
