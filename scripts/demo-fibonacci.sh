#!/bin/bash
# Demo: Fibonacci(10) = 55
# Pipeline: Rust → MSP430 ASM → COR24 ASM → assembled binary → emulator
#
# Rust source: examples/msp430-demos/src/lib.rs::demo_fibonacci()
# LLVM constant-folds fib(10) to 55 at compile time.
# The standalone fibonacci(n) function is also translated (with register spilling).
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$SCRIPT_DIR/.."
DEMO_DIR="$PROJECT_DIR/examples/msp430-demos"
OUTPUT_DIR="$PROJECT_DIR/output"
mkdir -p "$OUTPUT_DIR"

echo "============================================================"
echo "  Demo: Fibonacci(10) = 55"
echo "  Pipeline: Rust → MSP430 ASM → COR24 ASM → emulator"
echo "============================================================"
echo

# --- Step 1: Show Rust source ---
echo "=== Step 1: Rust Source ==="
sed -n '/Demo 6: Fibonacci/,/^}/p' "$DEMO_DIR/src/lib.rs"
echo

# --- Step 2: Show MSP430 assembly ---
MSP430_ASM=$(find "$DEMO_DIR/target/msp430-none-elf/release/deps/" -name "*.s" | head -1)
cp "$MSP430_ASM" "$OUTPUT_DIR/demo_fibonacci.msp430.s"
echo "=== Step 2: MSP430 ASM ==="
echo "--- demo_fibonacci (constant-folded by LLVM): ---"
sed -n '/\.text\.demo_fibonacci/,/\.size.*demo_fibonacci/p' "$MSP430_ASM"
echo
echo "--- fibonacci(n) standalone (register spills r11,r15): ---"
sed -n '/\.text\.fibonacci,/,/\.size.*fibonacci,/p' "$MSP430_ASM"
echo

# --- Step 3: Translate MSP430 → COR24 ---
echo "=== Step 3: COR24 ASM ==="
(cd "$PROJECT_DIR" && cargo run --quiet --bin msp430-to-cor24 -- "$MSP430_ASM" -o "$OUTPUT_DIR/demo_fibonacci.cor24.s")
echo "--- demo_fibonacci ---"
sed -n '/function: demo_fibonacci/,/function:/p' "$OUTPUT_DIR/demo_fibonacci.cor24.s" | head -15
echo
echo "--- fibonacci (with spill slots for r11→21(fp), r15→24(fp)) ---"
sed -n '/function: fibonacci/,/function:/p' "$OUTPUT_DIR/demo_fibonacci.cor24.s" | head -25
echo

# --- Step 4: Run ---
echo "=== Step 4: Run in Emulator ==="
(cd "$PROJECT_DIR" && cargo run --quiet --bin cor24-run -- \
    --run "$OUTPUT_DIR/demo_fibonacci.cor24.s" \
    --entry demo_fibonacci \
    --dump \
    --speed 0 \
    --time 1) 2>&1 | tee "$OUTPUT_DIR/demo_fibonacci.log"

echo
echo "Expected: LED = 0x37 (55 = fib(10))"
echo
echo "=== Output Files ==="
echo "  MSP430 ASM:    output/demo_fibonacci.msp430.s"
echo "  COR24 ASM:     output/demo_fibonacci.cor24.s"
echo "  Emulator log:  output/demo_fibonacci.log"
