#!/bin/bash
# Demo: Countdown 10→0 on LED
# Pipeline: Rust → MSP430 ASM → COR24 ASM → assembled binary → emulator
#
# Rust source: examples/msp430-demos/src/lib.rs::demo_countdown()
# Counts from 10 to 0, writing each value to the LED register,
# with a delay(1000) between each step.  Uses register spilling (r10).
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$SCRIPT_DIR/.."
DEMO_DIR="$PROJECT_DIR/examples/msp430-demos"
OUTPUT_DIR="$PROJECT_DIR/output"
mkdir -p "$OUTPUT_DIR"

echo "============================================================"
echo "  Demo: Countdown 10 → 0 on LED"
echo "  Pipeline: Rust → MSP430 ASM → COR24 ASM → emulator"
echo "============================================================"
echo

# --- Step 1: Show Rust source ---
echo "=== Step 1: Rust Source ==="
sed -n '/Demo 4: Countdown/,/^\/\/ ====/p' "$DEMO_DIR/src/lib.rs" | head -18
echo

# --- Step 2: Show MSP430 assembly ---
MSP430_ASM=$(find "$DEMO_DIR/target/msp430-none-elf/release/deps/" -name "*.s" | head -1)
cp "$MSP430_ASM" "$OUTPUT_DIR/demo_countdown.msp430.s"
echo "=== Step 2: MSP430 ASM (uses callee-saved r10) ==="
sed -n '/\.text\.demo_countdown/,/\.size.*demo_countdown/p' "$MSP430_ASM"
echo

# --- Step 3: Translate MSP430 → COR24 ---
echo "=== Step 3: COR24 ASM ==="
(cd "$PROJECT_DIR" && cargo run --quiet --bin msp430-to-cor24 -- "$MSP430_ASM" -o "$OUTPUT_DIR/demo_countdown.cor24.s")
sed -n '/function: demo_countdown/,/function:/p' "$OUTPUT_DIR/demo_countdown.cor24.s" | head -35
echo

# --- Step 4: Run ---
echo "=== Step 4: Run in Emulator ==="
(cd "$PROJECT_DIR" && cargo run --quiet --bin cor24-run -- \
    --run "$OUTPUT_DIR/demo_countdown.cor24.s" \
    --entry demo_countdown \
    --dump \
    --speed 0 \
    --time 10) 2>&1 | tee "$OUTPUT_DIR/demo_countdown.log"

echo
echo "Expected: LED = 0x00 (counted down to 0, then halted)"
echo
echo "=== Output Files ==="
echo "  MSP430 ASM:    output/demo_countdown.msp430.s"
echo "  COR24 ASM:     output/demo_countdown.cor24.s"
echo "  Emulator log:  output/demo_countdown.log"
