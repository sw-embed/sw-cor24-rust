#!/bin/bash
# Run this demo through the full Rust→COR24 pipeline
# Default UART input: 'abc3\x21' (letters→uppercase, digit as-is, !→halt)
# Override with: ./run.sh --uart-input 'Hello\x21'
DIR="$(cd "$(dirname "$0")" && pwd)"
DEMO="$(basename "$DIR")"

# Check if user provided --uart-input; if not, add default
HAS_UART=false
for arg in "$@"; do
    if [ "$arg" = "--uart-input" ] || [ "$arg" = "-u" ]; then
        HAS_UART=true
        break
    fi
done

if [ "$HAS_UART" = false ]; then
    exec "$(dirname "$DIR")/run-demo.sh" "$DEMO" --uart-input 'abc3\x21' "$@"
else
    exec "$(dirname "$DIR")/run-demo.sh" "$DEMO" "$@"
fi
