#!/bin/bash
# Demo: Blinky - Toggle LED with delay
# Pipeline: Rust → MSP430 ASM → COR24 ASM → assembled binary → emulator
#
# Rust source: examples/msp430-demos/src/lib.rs::demo_blinky()
# Infinite loop: LED on, delay(5000), LED off, delay(5000).
# Runs forever — use --time to limit.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$SCRIPT_DIR/.."
DEMO_DIR="$PROJECT_DIR/examples/msp430-demos"
OUTPUT_DIR="$PROJECT_DIR/output"
mkdir -p "$OUTPUT_DIR"

echo "============================================================"
echo "  Demo: Blinky (Toggle LED with delay)"
echo "  Pipeline: Rust → MSP430 ASM → COR24 ASM → emulator"
echo "============================================================"
echo

# --- Step 1: Show Rust source ---
echo "=== Step 1: Rust Source ==="
sed -n '/Demo 1: Blinky/,/^\/\/ ====/p' "$DEMO_DIR/src/lib.rs" | head -14
echo

# --- Step 2: Show MSP430 assembly ---
MSP430_ASM=$(find "$DEMO_DIR/target/msp430-none-elf/release/deps/" -name "*.s" | head -1)
cp "$MSP430_ASM" "$OUTPUT_DIR/demo_blinky.msp430.s"
echo "=== Step 2: MSP430 ASM ==="
sed -n '/\.text\.demo_blinky/,/\.size.*demo_blinky/p' "$MSP430_ASM"
echo

# --- Step 3: Translate MSP430 → COR24 ---
echo "=== Step 3: COR24 ASM ==="
(cd "$PROJECT_DIR" && cargo run --quiet --bin msp430-to-cor24 -- "$MSP430_ASM" -o "$OUTPUT_DIR/demo_blinky.cor24.s")
sed -n '/function: demo_blinky/,/function:/p' "$OUTPUT_DIR/demo_blinky.cor24.s" | head -30
echo

# --- Step 4: Run ---
echo "=== Step 4: Run in Emulator ==="
echo "(Blinky runs forever — limited to 2 seconds)"
(cd "$PROJECT_DIR" && cargo run --quiet --bin cor24-run -- \
    --run "$OUTPUT_DIR/demo_blinky.cor24.s" \
    --entry demo_blinky \
    --dump \
    --speed 0 \
    --time 2) 2>&1 | tee "$OUTPUT_DIR/demo_blinky.log"

echo
echo "Expected: LED toggles between 0x01 and 0x00 (with delay loop between)"
echo
echo "=== Output Files ==="
echo "  MSP430 ASM:    output/demo_blinky.msp430.s"
echo "  COR24 ASM:     output/demo_blinky.cor24.s"
echo "  Emulator log:  output/demo_blinky.log"
