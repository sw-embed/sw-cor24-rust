# VHS tape: UART Hello demo through the full Rust→COR24 pipeline
# Records: compile, translate, assemble, run
# Usage: vhs docs/vhs/uart-hello.tape
#
# The uart_putc function polls TX busy (bit 7 of status register)
# before each write — verified by the cls/brt pattern in the
# generated COR24 assembly.

Output docs/vhs/uart-hello.gif

Set Shell "bash"
Set FontSize 14
Set Width 1000
Set Height 600
Set Theme "Dracula"

Type "# Rust → MSP430 → COR24 pipeline: UART Hello demo"
Enter
Sleep 1s

Type "# uart_putc polls TX busy before each write"
Enter
Sleep 1s

Type "cd rust-to-cor24/demos && bash demo_uart_hello/run.sh"
Enter
Sleep 30s

Type "# UART output: 'Hello' with TX busy polling"
Enter
Sleep 3s
