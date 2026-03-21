# VHS tape: UART Hello — Rust → MSP430 → COR24 pipeline
# Shows each stage with pauses for narration
# Usage: vhs docs/vhs/uart-hello.tape

Output docs/vhs/uart-hello.gif

Set Shell "bash"
Set FontSize 14
Set Width 1000
Set Height 600
Set Theme "Dracula"

# --- Title ---
Type "# Rust → COR24 Pipeline: UART Hello"
Enter
Type "# uart_putc polls TX busy before each write"
Enter
Sleep 3s

# --- Show Rust source: MMIO + uart_putc (first 27 lines) ---
Type "clear && echo '=== Rust Source: MMIO and uart_putc ===' && head -27 rust-to-cor24/demos/demo_uart_hello/src/lib.rs"
Enter
Sleep 6s

# --- Show Rust source: demo function + entry point ---
Type "clear && echo '=== Rust Source: demo + entry point ===' && tail -33 rust-to-cor24/demos/demo_uart_hello/src/lib.rs"
Enter
Sleep 6s

# --- Run the pipeline, capture output ---
Type "clear && echo '=== Running Pipeline: Rust → MSP430 → COR24 → Emulator ===' && cd rust-to-cor24/demos && bash demo_uart_hello/run.sh > /tmp/uart-demo.txt 2>&1 && cd ../.. && echo 'Pipeline complete.'"
Enter
Sleep 25s

# --- Show COR24 assembly: uart_putc function ---
Type "clear && echo '=== COR24 Assembly: uart_putc (translated) ===' && grep -A15 'function: uart_putc' /tmp/uart-demo.txt"
Enter
Sleep 6s

# --- Show UART output result ---
Type "clear && echo '=== Emulator Result ===' && grep -B1 -A5 'UART:' /tmp/uart-demo.txt"
Enter
Sleep 5s

# --- Show register state ---
Type "clear && echo '=== Final CPU State ===' && grep -A8 '=== Registers ===' /tmp/uart-demo.txt"
Enter
Sleep 5s

# --- Done ---
Type "clear && echo '=== Pipeline Complete ===' && echo 'Rust source → rustc (MSP430) → translator → COR24 asm → emulator' && echo 'UART output: Hello (with TX busy polling)'"
Enter
Sleep 4s
