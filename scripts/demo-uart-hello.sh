#!/bin/bash
# Demo: UART Hello World
# Pipeline: Rust → MSP430 ASM → COR24 ASM → assembled binary → emulator
#
# Rust source: examples/msp430-demos/src/lib.rs::demo_uart_hello()
# Sends "Hello\n" via UART, then halts.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$SCRIPT_DIR/.."
DEMO_DIR="$PROJECT_DIR/examples/msp430-demos"
OUTPUT_DIR="$PROJECT_DIR/output"
mkdir -p "$OUTPUT_DIR"

echo "============================================================"
echo "  Demo: UART Hello World"
echo "  Pipeline: Rust → MSP430 ASM → COR24 ASM → emulator"
echo "============================================================"
echo

# --- Step 1: Show Rust source ---
echo "=== Step 1: Rust Source ==="
echo "File: examples/msp430-demos/src/lib.rs"
echo
sed -n '/Demo 2: UART/,/^\/\/ ====/p' "$DEMO_DIR/src/lib.rs" | head -20
echo "..."
echo

# --- Step 2: Compile Rust → MSP430 assembly ---
echo "=== Step 2: Compile Rust → MSP430 ASM ==="
echo "Command: rustup run nightly cargo rustc --target msp430-none-elf -Z build-std=core --release -- --emit asm"
echo

# Find existing .s file (skip recompile if present)
MSP430_ASM=$(find "$DEMO_DIR/target/msp430-none-elf/release/deps/" -name "*.s" 2>/dev/null | head -1)
if [ -z "$MSP430_ASM" ]; then
    echo "Compiling (first time, ~6s for core library)..."
    (cd "$DEMO_DIR" && rustup run nightly cargo rustc --target msp430-none-elf -Z build-std=core --release -- --emit asm)
    MSP430_ASM=$(find "$DEMO_DIR/target/msp430-none-elf/release/deps/" -name "*.s" | head -1)
fi

cp "$MSP430_ASM" "$OUTPUT_DIR/demo_uart_hello.msp430.s"
echo "MSP430 ASM: $OUTPUT_DIR/demo_uart_hello.msp430.s"
echo
echo "--- MSP430 assembly for demo_uart_hello + uart_putc ---"
sed -n '/\.text\.demo_uart_hello/,/\.size.*demo_uart_hello/p' "$MSP430_ASM"
echo
sed -n '/\.text\.uart_putc/,/\.size.*uart_putc/p' "$MSP430_ASM"
echo

# --- Step 3: Translate MSP430 → COR24 assembly ---
echo "=== Step 3: Translate MSP430 → COR24 ASM ==="
echo "Command: msp430-to-cor24 $MSP430_ASM -o output/demo_uart_hello.cor24.s"
(cd "$PROJECT_DIR" && cargo run --quiet --bin msp430-to-cor24 -- "$MSP430_ASM" -o "$OUTPUT_DIR/demo_uart_hello.cor24.s")
echo "COR24 ASM: $OUTPUT_DIR/demo_uart_hello.cor24.s"
echo
echo "--- COR24 assembly for demo_uart_hello ---"
sed -n '/function: demo_uart_hello/,/function:/p' "$OUTPUT_DIR/demo_uart_hello.cor24.s" | head -30
echo

# --- Step 4: Assemble + Run in emulator ---
echo "=== Step 4: Assemble & Run in COR24 Emulator ==="
echo "Command: cor24-run --run output/demo_uart_hello.cor24.s --entry demo_uart_hello --dump --speed 0 --time 1"
echo
(cd "$PROJECT_DIR" && cargo run --quiet --bin cor24-run -- \
    --run "$OUTPUT_DIR/demo_uart_hello.cor24.s" \
    --entry demo_uart_hello \
    --dump \
    --speed 0 \
    --time 1) 2>&1 | tee "$OUTPUT_DIR/demo_uart_hello.log"

echo
echo "=== Output Files ==="
echo "  Rust source:   examples/msp430-demos/src/lib.rs"
echo "  MSP430 ASM:    output/demo_uart_hello.msp430.s"
echo "  COR24 ASM:     output/demo_uart_hello.cor24.s"
echo "  Emulator log:  output/demo_uart_hello.log"
