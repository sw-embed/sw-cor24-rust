#!/bin/bash
# Demo: Stack-heavy - many locals that spill to stack
# Pipeline: Rust → MSP430 ASM → COR24 ASM → assembled binary → emulator
#
# Rust source: examples/msp430-demos/src/lib.rs::demo_stack_vars()
# accumulate() uses 5 callee-saved registers (r6-r10 on MSP430), which
# are all spilled to fp-relative memory slots in COR24 translation.
# At halt, the spill slots contain the intermediate computation values.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$SCRIPT_DIR/.."
DEMO_DIR="$PROJECT_DIR/examples/msp430-demos"
OUTPUT_DIR="$PROJECT_DIR/output"
mkdir -p "$OUTPUT_DIR"

echo "============================================================"
echo "  Demo: Stack Variables (5 callee-saved regs spilled)"
echo "  Pipeline: Rust → MSP430 ASM → COR24 ASM → emulator"
echo "============================================================"
echo

# --- Step 1: Show Rust source ---
echo "=== Step 1: Rust Source ==="
sed -n '/Demo 8: Stack-heavy/,/^}/p' "$DEMO_DIR/src/lib.rs"
echo

# --- Step 2: Show MSP430 assembly ---
MSP430_ASM=$(find "$DEMO_DIR/target/msp430-none-elf/release/deps/" -name "*.s" | head -1)
cp "$MSP430_ASM" "$OUTPUT_DIR/demo_stack_vars.msp430.s"
echo "=== Step 2: MSP430 ASM ==="
echo "--- demo_stack_vars (reads button+1, calls accumulate): ---"
sed -n '/\.text\.demo_stack_vars/,/\.size.*demo_stack_vars/p' "$MSP430_ASM"
echo
echo "--- accumulate (pushes r6,r7,r8,r9,r10 — 5 callee-saved regs): ---"
sed -n '/\.text\.accumulate/,/\.size.*accumulate/p' "$MSP430_ASM"
echo

# --- Step 3: Translate MSP430 → COR24 ---
echo "=== Step 3: COR24 ASM ==="
(cd "$PROJECT_DIR" && cargo run --quiet --bin msp430-to-cor24 -- "$MSP430_ASM" -o "$OUTPUT_DIR/demo_stack_vars.cor24.s")
echo "--- demo_stack_vars ---"
sed -n '/function: demo_stack_vars/,/function:/p' "$OUTPUT_DIR/demo_stack_vars.cor24.s" | head -12
echo
echo "--- accumulate (spill slots for r6→6(fp), r7→9(fp), r8→12(fp), r9→15(fp), r10→18(fp)): ---"
sed -n '/function: accumulate/,/function:/p' "$OUTPUT_DIR/demo_stack_vars.cor24.s" | head -60
echo

# --- Step 4: Run ---
echo "=== Step 4: Run in Emulator ==="
echo "  With btn=1 (default): seed=btn+1=2"
echo "  a=seed+1=3, b=a+seed=5, c=b+a=8, d=c+b=13, e=d+c=21"
echo "  result = 3^5^8^13^21 = 0x1C = 28"
echo "  UART: a=3, b=5, c=8, d=13, e=21"
echo
(cd "$PROJECT_DIR" && cargo run --quiet --bin cor24-run -- \
    --run "$OUTPUT_DIR/demo_stack_vars.cor24.s" \
    --entry demo_stack_vars \
    --dump \
    --speed 0 \
    --time 1) 2>&1 | tee "$OUTPUT_DIR/demo_stack_vars.log"

echo
echo "EBR/Stack should show: 5 saved callee-saved registers + spill slot values"
echo "  fp-relative slots for r6(a), r7(e), r8(d), r9(c), r10(b)"
echo
echo "=== Output Files ==="
echo "  MSP430 ASM:    output/demo_stack_vars.msp430.s"
echo "  COR24 ASM:     output/demo_stack_vars.cor24.s"
echo "  Emulator log:  output/demo_stack_vars.log"
