# VHS tape: Assembler UART Hello — direct COR24 assembly
# Shows source, assembly listing, and emulator output
# Usage: vhs docs/vhs/asm-uart-hello.tape

Output docs/vhs/asm-uart-hello.gif

Set Shell "bash"
Set FontSize 14
Set Width 1000
Set Height 600
Set Theme "Dracula"

# --- Title ---
Type "# COR24 Assembler: UART Hello"
Enter
Type "# Polls TX busy (cls/brt) before each character"
Enter
Sleep 3s

# --- Show source: main code (first 24 lines) ---
Type "clear && echo '=== Source: Main Code ===' && head -24 src/examples/assembler/uart_hello.s"
Enter
Sleep 6s

# --- Show source: putc function (remaining lines) ---
Type "clear && echo '=== Source: putc with TX Busy Poll ===' && tail -16 src/examples/assembler/uart_hello.s"
Enter
Sleep 6s

# --- Assemble and run, capture output ---
Type "clear && echo '=== Assembling and Running...' && rust-to-cor24/target/release/cor24-run --run src/examples/assembler/uart_hello.s --dump --trace 20 > /tmp/asm-demo.txt 2>&1 && echo 'Done.'"
Enter
Sleep 5s

# --- Show UART output ---
Type "clear && echo '=== UART Output ===' && grep -B1 -A3 'UART:' /tmp/asm-demo.txt"
Enter
Sleep 5s

# --- Show instruction trace (last 20) ---
Type "clear && echo '=== Instruction Trace (last 20) ===' && head -25 /tmp/asm-demo.txt"
Enter
Sleep 6s

# --- Show register state ---
Type "clear && echo '=== Final CPU State ===' && grep -A8 '=== Registers ===' /tmp/asm-demo.txt"
Enter
Sleep 5s

# --- Done ---
Type "clear && echo '=== Done! ===' && echo 'uart_hello.s → cor24-run → Hello on UART' && echo 'TX busy poll: lb status, cls r2,z, brt .wait'"
Enter
Sleep 4s
