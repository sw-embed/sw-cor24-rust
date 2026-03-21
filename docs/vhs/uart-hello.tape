# VHS tape: UART Hello — Rust → MSP430 → COR24 pipeline
# Shows each stage with pauses for narration
# Usage: vhs docs/vhs/uart-hello.tape

Output docs/vhs/uart-hello.gif

Set Shell "bash"
Set FontSize 32
Set Width 1920
Set Height 1080
Set Theme "Dracula"

# --- Title ---
Type "# Rust → COR24 Pipeline: UART Hello"
Enter
Type "# uart_putc polls TX busy before each write"
Enter
Sleep 3s

# --- Show Rust source: MMIO + uart_putc ---
Type "clear && echo '=== Rust Source: uart_putc ===' && sed -n '1,13p' rust-to-cor24/demos/demo_uart_hello/src/lib.rs"
Enter
Sleep 6s

# --- Show Rust source: demo function ---
Type "clear && echo '=== Rust Source: demo + entry ===' && sed -n '14,27p' rust-to-cor24/demos/demo_uart_hello/src/lib.rs"
Enter
Sleep 6s

# --- Run the pipeline, capture output ---
Type "clear && echo '=== Running Pipeline... ===' && cd rust-to-cor24/demos && bash demo_uart_hello/run.sh > /tmp/uart-demo.txt 2>&1 && cd ../.. && echo 'Done.'"
Enter
Sleep 25s

# --- Show COR24 assembly: uart_putc ---
Type "clear && echo '=== COR24: uart_putc ===' && grep -A12 'function: uart_putc' /tmp/uart-demo.txt"
Enter
Sleep 6s

# --- Show UART output ---
Type "clear && echo '=== Emulator Result ===' && grep -B1 -A5 'UART:' /tmp/uart-demo.txt"
Enter
Sleep 5s

# --- Show register state ---
Type "clear && echo '=== Final CPU State ===' && grep -A8 '=== Registers ===' /tmp/uart-demo.txt"
Enter
Sleep 5s

# --- Done ---
Type "clear && echo '=== Pipeline Complete ===' && echo 'Rust → rustc (MSP430) → translator → COR24 → emulator' && echo 'UART: Hello (with TX busy polling)'"
Enter
Sleep 4s
