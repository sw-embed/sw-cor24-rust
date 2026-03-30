#!/bin/bash
# Run a single demo through the full Rust→COR24 pipeline
#
# Usage:
#   ./run-demo.sh <demo_name> [--uart-input <str>] [--skip-compile]
#
# Steps shown:
#   1. Compile Rust → MSP430 assembly (rustc --target msp430-none-elf)
#   2. Translate MSP430 → COR24 assembly (msp430-to-cor24)
#   3. Assemble + run in emulator with register/memory dump (cor24-run)
#
# Examples:
#   ./run-demo.sh demo_add
#   ./run-demo.sh demo_echo_v2 --uart-input 'abc\x21'
#   ./run-demo.sh demo_uart_hello --skip-compile

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
TRANSLATOR_DIR="$SCRIPT_DIR/.."

# --- Parse arguments ---
DEMO=""
UART_INPUT=""
SKIP_COMPILE=false

while [[ $# -gt 0 ]]; do
    case "$1" in
        --uart-input|-u) UART_INPUT="$2"; shift 2 ;;
        --skip-compile)  SKIP_COMPILE=true; shift ;;
        --help|-h)
            echo "Usage: $0 <demo_name> [--uart-input <str>] [--skip-compile]"
            echo
            echo "Available demos:"
            for d in "$SCRIPT_DIR"/demo_*/; do
                basename "$d"
            done
            exit 0
            ;;
        *)
            if [ -z "$DEMO" ]; then
                DEMO="$1"
            fi
            shift
            ;;
    esac
done

if [ -z "$DEMO" ]; then
    echo "Usage: $0 <demo_name> [--uart-input <str>] [--skip-compile]"
    echo
    echo "Available demos:"
    for d in "$SCRIPT_DIR"/demo_*/; do
        basename "$d"
    done
    exit 1
fi

DEMO_DIR="$SCRIPT_DIR/$DEMO"
if [ ! -d "$DEMO_DIR" ]; then
    echo "Error: demo '$DEMO' not found at $DEMO_DIR"
    exit 1
fi

# Build tools
echo "Building msp430-to-cor24 and cor24-run..."
(cd "$TRANSLATOR_DIR" && cargo build --release --quiet)
TRANSLATE="$TRANSLATOR_DIR/target/release/msp430-to-cor24"
RUN="$TRANSLATOR_DIR/target/release/cor24-run"

echo
echo "========================================"
echo "  Pipeline: $DEMO"
echo "========================================"

# --- Step 1: Rust → MSP430 assembly ---
if [ "$SKIP_COMPILE" = true ] && [ -f "$DEMO_DIR/${DEMO}.msp430.s" ]; then
    echo
    echo "  [1/3] SKIP: Using existing ${DEMO}.msp430.s"
else
    echo
    echo "  [1/3] Compiling Rust → MSP430 assembly"
    echo "        rustc --target msp430-none-elf --emit asm"
    echo

    (cd "$DEMO_DIR" && rustup run nightly cargo rustc \
        --target msp430-none-elf \
        -Z build-std=core \
        --release \
        -- --emit asm 2>&1) || {
        echo "  FAILED: Rust compilation"
        exit 1
    }

    MSP430_S=$(find "$DEMO_DIR/target/msp430-none-elf/release/deps/" -name "*.s" | head -1)
    if [ -z "$MSP430_S" ]; then
        echo "  FAILED: No .s file found"
        exit 1
    fi
    cp "$MSP430_S" "$DEMO_DIR/${DEMO}.msp430.s"
fi

echo "  → ${DEMO}.msp430.s"
echo
echo "  --- Rust source (src/lib.rs) ---"
cat "$DEMO_DIR/src/lib.rs"
echo
echo "  --- MSP430 assembly (key functions) ---"
# Show only the .text sections, skip directives
grep -v '^\s*\.' "$DEMO_DIR/${DEMO}.msp430.s" | grep -v '^\s*$' | head -40
echo "  ..."

# --- Step 2: MSP430 → COR24 assembly ---
echo
echo "  [2/3] Translating MSP430 → COR24 assembly"
echo "        msp430-to-cor24 --entry start"
echo
"$TRANSLATE" "$DEMO_DIR/${DEMO}.msp430.s" -o "$DEMO_DIR/${DEMO}.cor24.s"
echo
echo "  --- COR24 assembly ---"
cat "$DEMO_DIR/${DEMO}.cor24.s"

# --- Step 3: Assemble + run ---
echo
echo "  [3/3] Assembling and running in COR24 emulator"

RUN_ARGS=(--run "$DEMO_DIR/${DEMO}.cor24.s" --dump --speed 0 --time 5)

if [ -n "$UART_INPUT" ]; then
    echo "        UART input: '$UART_INPUT'"
    RUN_ARGS+=(--uart-input "$UART_INPUT")
fi

echo
"$RUN" "${RUN_ARGS[@]}"

echo
echo "========================================"
echo "  Done: $DEMO"
echo "========================================"
