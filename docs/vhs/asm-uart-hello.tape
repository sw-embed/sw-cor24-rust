# VHS tape: Assembler UART Hello — direct COR24 assembly
# Shows source, assembly listing, and emulator output
# Usage: vhs docs/vhs/asm-uart-hello.tape

Output docs/vhs/asm-uart-hello.gif

Set Shell "bash"
Set FontSize 32
Set Width 1920
Set Height 1080
Set Theme "Dracula"

# --- Title ---
Type "# COR24 Assembler: UART Hello"
Enter
Type "# Polls TX busy (cls/brt) before each write"
Enter
Sleep 3s

# --- Show source: main code ---
Type "clear && echo '=== Source: Main Code ===' && head -13 src/examples/assembler/uart_hello.s"
Enter
Sleep 6s

# --- Show source: putc function ---
Type "clear && echo '=== Source: putc (TX Busy Poll) ===' && tail -13 src/examples/assembler/uart_hello.s"
Enter
Sleep 6s

# --- Assemble and run, capture output ---
Type "clear && echo '=== Assembling and Running...' && rust-to-cor24/target/release/cor24-run --run src/examples/assembler/uart_hello.s --dump --trace 10 > /tmp/asm-demo.txt 2>&1 && echo 'Done.'"
Enter
Sleep 5s

# --- Show UART output ---
Type "clear && echo '=== UART Output ===' && grep -B1 -A3 'UART:' /tmp/asm-demo.txt"
Enter
Sleep 5s

# --- Show instruction trace ---
Type "clear && echo '=== Instruction Trace ===' && head -13 /tmp/asm-demo.txt"
Enter
Sleep 6s

# --- Show register state ---
Type "clear && echo '=== Final CPU State ===' && grep -A8 '=== Registers ===' /tmp/asm-demo.txt"
Enter
Sleep 5s

# --- Done ---
Type "clear && echo '=== Done! ===' && echo 'uart_hello.s → cor24-run → Hello on UART'"
Enter
Sleep 4s
