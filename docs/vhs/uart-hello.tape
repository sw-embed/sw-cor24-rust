# VHS tape: UART Hello demo through the full Rust→COR24 pipeline
# Records: compile, translate, assemble, run
# Usage: vhs docs/vhs/uart-hello.tape

Output docs/vhs/uart-hello.gif

Set Shell "bash"
Set FontSize 14
Set Width 1000
Set Height 600
Set Theme "Dracula"

Type "# Rust → MSP430 → COR24 pipeline: UART Hello demo"
Enter
Sleep 1s

Type "cd rust-to-cor24/demos && bash demo_uart_hello/run.sh"
Enter
Sleep 30s

Type "# Done! UART output shows 'Hello'"
Enter
Sleep 3s
