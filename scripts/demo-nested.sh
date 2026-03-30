#!/bin/bash
# Demo: Nested Calls - show stack frames from A→B→C
# Pipeline: Rust → MSP430 ASM → COR24 ASM → assembled binary → emulator
#
# Rust source: examples/msp430-demos/src/lib.rs::demo_nested()
# Calls demo_nested → level_a → level_b → level_c, halts inside level_c.
# At halt, all four stack frames are live and visible in memory dump.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$SCRIPT_DIR/.."
DEMO_DIR="$PROJECT_DIR/examples/msp430-demos"
OUTPUT_DIR="$PROJECT_DIR/output"
mkdir -p "$OUTPUT_DIR"

echo "============================================================"
echo "  Demo: Nested Calls (4 levels of stack frames)"
echo "  Pipeline: Rust → MSP430 ASM → COR24 ASM → emulator"
echo "============================================================"
echo

# --- Step 1: Show Rust source ---
echo "=== Step 1: Rust Source ==="
sed -n '/Demo 7: Nested/,/^\/\/ ====/p' "$DEMO_DIR/src/lib.rs" | head -30
echo

# --- Step 2: Show MSP430 assembly ---
MSP430_ASM=$(find "$DEMO_DIR/target/msp430-none-elf/release/deps/" -name "*.s" | head -1)
cp "$MSP430_ASM" "$OUTPUT_DIR/demo_nested.msp430.s"
echo "=== Step 2: MSP430 ASM ==="
echo "--- demo_nested (reads button, calls level_a): ---"
sed -n '/\.text\.demo_nested/,/\.size.*demo_nested/p' "$MSP430_ASM"
echo
echo "--- level_a (adds 10, calls level_b): ---"
sed -n '/\.text\.level_a,/,/\.size.*level_a,/p' "$MSP430_ASM"
echo
echo "--- level_b (doubles+3, passes 2 args to level_c): ---"
sed -n '/\.text\.level_b/,/\.size.*level_b/p' "$MSP430_ASM"
echo
echo "--- level_c (push r10, writes LED+UART, halts): ---"
sed -n '/\.text\.level_c/,/\.size.*level_c/p' "$MSP430_ASM"
echo

# --- Step 3: Translate MSP430 → COR24 ---
echo "=== Step 3: COR24 ASM ==="
(cd "$PROJECT_DIR" && cargo run --quiet --bin msp430-to-cor24 -- "$MSP430_ASM" -o "$OUTPUT_DIR/demo_nested.cor24.s")
for fn in demo_nested level_a level_b level_c; do
    echo "--- $fn ---"
    sed -n "/function: $fn\$/,/function:/p" "$OUTPUT_DIR/demo_nested.cor24.s" | head -20
    echo
done

# --- Step 4: Run ---
echo "=== Step 4: Run in Emulator ==="
echo "  demo_nested() → level_a(btn+5) → level_b(btn+15) → level_c(2*(btn+15)+3, btn+15)"
echo "  With btn=1 (default): level_c(35, 16) → LED=35=0x23, UART='\\x10'"
echo
(cd "$PROJECT_DIR" && cargo run --quiet --bin cor24-run -- \
    --run "$OUTPUT_DIR/demo_nested.cor24.s" \
    --entry demo_nested \
    --dump \
    --speed 0 \
    --time 1) 2>&1 | tee "$OUTPUT_DIR/demo_nested.log"

echo
echo "Stack should show: 4 return addresses + saved r10 from level_c"
echo
echo "=== Output Files ==="
echo "  MSP430 ASM:    output/demo_nested.msp430.s"
echo "  COR24 ASM:     output/demo_nested.cor24.s"
echo "  Emulator log:  output/demo_nested.log"
