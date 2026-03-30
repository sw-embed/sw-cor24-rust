#!/bin/bash
# Demo: Compute (100 + 200 + 42 = 342)
# Pipeline: Rust → MSP430 ASM → COR24 ASM → assembled binary → emulator
#
# Rust source: examples/msp430-demos/src/lib.rs::demo_add()
# LLVM constant-folds 100+200+42 to 342 at compile time.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$SCRIPT_DIR/.."
DEMO_DIR="$PROJECT_DIR/examples/msp430-demos"
OUTPUT_DIR="$PROJECT_DIR/output"
mkdir -p "$OUTPUT_DIR"

echo "============================================================"
echo "  Demo: Compute (100 + 200 + 42 = 342)"
echo "  Pipeline: Rust → MSP430 ASM → COR24 ASM → emulator"
echo "============================================================"
echo

# --- Step 1: Show Rust source ---
echo "=== Step 1: Rust Source ==="
sed -n '/Demo 3: Compute/,/^\/\/ ====/p' "$DEMO_DIR/src/lib.rs" | head -12
echo

# --- Step 2: Show MSP430 assembly ---
MSP430_ASM=$(find "$DEMO_DIR/target/msp430-none-elf/release/deps/" -name "*.s" | head -1)
cp "$MSP430_ASM" "$OUTPUT_DIR/demo_add.msp430.s"
echo "=== Step 2: MSP430 ASM (LLVM constant-folded!) ==="
sed -n '/\.text\.demo_add/,/\.size.*demo_add/p' "$MSP430_ASM"
echo

# --- Step 3: Translate MSP430 → COR24 ---
echo "=== Step 3: COR24 ASM ==="
(cd "$PROJECT_DIR" && cargo run --quiet --bin msp430-to-cor24 -- "$MSP430_ASM" -o "$OUTPUT_DIR/demo_add.cor24.s")
sed -n '/function: demo_add/,/function:/p' "$OUTPUT_DIR/demo_add.cor24.s" | head -10
echo

# --- Step 4: Run ---
echo "=== Step 4: Run in Emulator ==="
(cd "$PROJECT_DIR" && cargo run --quiet --bin cor24-run -- \
    --run "$OUTPUT_DIR/demo_add.cor24.s" \
    --entry demo_add \
    --dump \
    --speed 0 \
    --time 1) 2>&1 | tee "$OUTPUT_DIR/demo_add.log"

echo
echo "Expected: r0 = 0x000156 (342 = 100 + 200 + 42)"
echo
echo "=== Output Files ==="
echo "  MSP430 ASM:    output/demo_add.msp430.s"
echo "  COR24 ASM:     output/demo_add.cor24.s"
echo "  Emulator log:  output/demo_add.log"
