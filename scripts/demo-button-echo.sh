#!/bin/bash
# Demo: Button Echo - Read button, echo to LED
# Pipeline: Rust → MSP430 ASM → COR24 ASM → assembled binary → emulator
#
# Rust source: examples/msp430-demos/src/lib.rs::demo_button_echo()
# Infinite loop: reads button state from I/O, masks bit 0, writes to LED.
# Runs forever — use --time to limit.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$SCRIPT_DIR/.."
DEMO_DIR="$PROJECT_DIR/examples/msp430-demos"
OUTPUT_DIR="$PROJECT_DIR/output"
mkdir -p "$OUTPUT_DIR"

echo "============================================================"
echo "  Demo: Button Echo (read button → echo to LED)"
echo "  Pipeline: Rust → MSP430 ASM → COR24 ASM → emulator"
echo "============================================================"
echo

# --- Step 1: Show Rust source ---
echo "=== Step 1: Rust Source ==="
sed -n '/Demo 5: Button Echo/,/^\/\/ ====/p' "$DEMO_DIR/src/lib.rs" | head -14
echo

# --- Step 2: Show MSP430 assembly ---
MSP430_ASM=$(find "$DEMO_DIR/target/msp430-none-elf/release/deps/" -name "*.s" | head -1)
cp "$MSP430_ASM" "$OUTPUT_DIR/demo_button_echo.msp430.s"
echo "=== Step 2: MSP430 ASM ==="
sed -n '/\.text\.demo_button_echo/,/\.size.*demo_button_echo/p' "$MSP430_ASM"
echo

# --- Step 3: Translate MSP430 → COR24 ---
echo "=== Step 3: COR24 ASM ==="
(cd "$PROJECT_DIR" && cargo run --quiet --bin msp430-to-cor24 -- "$MSP430_ASM" -o "$OUTPUT_DIR/demo_button_echo.cor24.s")
sed -n '/function: demo_button_echo/,/function:/p' "$OUTPUT_DIR/demo_button_echo.cor24.s" | head -25
echo

# --- Step 4: Run ---
echo "=== Step 4: Run in Emulator ==="
echo "(Button echo runs forever — limited to 1 second)"
(cd "$PROJECT_DIR" && cargo run --quiet --bin cor24-run -- \
    --run "$OUTPUT_DIR/demo_button_echo.cor24.s" \
    --entry demo_button_echo \
    --dump \
    --speed 0 \
    --time 1) 2>&1 | tee "$OUTPUT_DIR/demo_button_echo.log"

echo
echo "Expected: LED mirrors button state (bit 0 of I/O register)"
echo
echo "=== Output Files ==="
echo "  MSP430 ASM:    output/demo_button_echo.msp430.s"
echo "  COR24 ASM:     output/demo_button_echo.cor24.s"
echo "  Emulator log:  output/demo_button_echo.log"
