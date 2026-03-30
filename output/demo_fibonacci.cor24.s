; COR24 Assembly - Generated from MSP430 via msp430-to-cor24
; Pipeline: Rust -> rustc (msp430-none-elf) -> MSP430 ASM -> COR24 ASM

; Reset vector -> demo_fibonacci
    la      r0, demo_fibonacci
    jmp     (r0)

; --- function: _RNvCsgMG9zBUy57e_7___rustc17rust_begin_unwind ---
_RNvCsgMG9zBUy57e_7___rustc17rust_begin_unwind:
.LBB0_1:
    bra     .LBB0_1
.Lfunc_end0:

; --- function: delay ---
delay:
    sub     sp, 3
    ceq     r0, z
    brt     .LBB1_3
    add     r0, -1
.LBB1_2:
    mov     r2, sp
    sw      r0, 0(r2)
    add     r0, -1
    lc      r1, -1
    ceq     r0, r1
    brf     .LBB1_2
.LBB1_3:
    add     sp, 3
    pop     r2
    jmp     (r2)
.Lfunc_end1:

; --- function: demo_add ---
demo_add:
    la      r0, 0x000156
    pop     r2
    jmp     (r2)
.Lfunc_end2:

; --- function: demo_blinky ---
demo_blinky:
.LBB3_1:
    la      r0, 0xFF0000
    lc      r1, 1
    ; call mmio_write
    la      r2, .Lret_0
    push    r2
    la      r2, mmio_write
    jmp     (r2)
    .Lret_0:
    la      r0, 0x001388
    ; call delay
    la      r2, .Lret_1
    push    r2
    la      r2, delay
    jmp     (r2)
    .Lret_1:
    la      r0, 0xFF0000
    lc      r1, 0
    ; call mmio_write
    la      r2, .Lret_2
    push    r2
    la      r2, mmio_write
    jmp     (r2)
    .Lret_2:
    la      r0, 0x001388
    ; call delay
    la      r2, .Lret_3
    push    r2
    la      r2, delay
    jmp     (r2)
    .Lret_3:
    bra     .LBB3_1
.Lfunc_end3:

; --- function: demo_button_echo ---
demo_button_echo:
.LBB4_1:
    la      r0, 0xFF0000
    ; call mmio_read
    la      r2, .Lret_4
    push    r2
    la      r2, mmio_read
    jmp     (r2)
    .Lret_4:
    mov     r1, r0
    lc      r0, 1
    and     r1, r0
    la      r0, 0xFF0000
    ; call mmio_write
    la      r2, .Lret_5
    push    r2
    la      r2, mmio_write
    jmp     (r2)
    .Lret_5:
    bra     .LBB4_1
.Lfunc_end4:

; --- function: demo_countdown ---
demo_countdown:
    lw      r0, 18(fp)
    push    r0
    lc      r0, 10
    sw      r0, 18(fp)
.LBB5_1:
    la      r0, 0xFF0000
    lw      r1, 18(fp)
    ; call mmio_write
    la      r2, .Lret_6
    push    r2
    la      r2, mmio_write
    jmp     (r2)
    .Lret_6:
    la      r0, 0x0003E8
    ; call delay
    la      r2, .Lret_7
    push    r2
    la      r2, delay
    jmp     (r2)
    .Lret_7:
    lw      r0, 18(fp)
    add     r0, -1
    sw      r0, 18(fp)
    lw      r0, 18(fp)
    ceq     r0, z
    brf     .LBB5_1
    la      r0, 0xFF0000
    lc      r1, 0
    ; call mmio_write
    la      r2, .Lret_8
    push    r2
    la      r2, mmio_write
    jmp     (r2)
    .Lret_8:
.LBB5_3:
    bra     .LBB5_3
.Lfunc_end5:

; --- function: demo_fibonacci ---
demo_fibonacci:
    la      r0, 0xFF0000
    lc      r1, 55
    ; call mmio_write
    la      r2, .Lret_9
    push    r2
    la      r2, mmio_write
    jmp     (r2)
    .Lret_9:
.LBB6_1:
    bra     .LBB6_1
.Lfunc_end6:

; --- function: demo_uart_hello ---
demo_uart_hello:
    lc      r0, 72
    ; call uart_putc
    la      r2, .Lret_10
    push    r2
    la      r2, uart_putc
    jmp     (r2)
    .Lret_10:
    lc      r0, 101
    ; call uart_putc
    la      r2, .Lret_11
    push    r2
    la      r2, uart_putc
    jmp     (r2)
    .Lret_11:
    lc      r0, 108
    ; call uart_putc
    la      r2, .Lret_12
    push    r2
    la      r2, uart_putc
    jmp     (r2)
    .Lret_12:
    lc      r0, 108
    ; call uart_putc
    la      r2, .Lret_13
    push    r2
    la      r2, uart_putc
    jmp     (r2)
    .Lret_13:
    lc      r0, 111
    ; call uart_putc
    la      r2, .Lret_14
    push    r2
    la      r2, uart_putc
    jmp     (r2)
    .Lret_14:
    lc      r0, 10
    ; call uart_putc
    la      r2, .Lret_15
    push    r2
    la      r2, uart_putc
    jmp     (r2)
    .Lret_15:
.LBB7_1:
    bra     .LBB7_1
.Lfunc_end7:

; --- function: fibonacci ---
fibonacci:
    lc      r1, 2
    clu     r0, r1
    brf     .LBB8_2
    mov     r1, r0
    bra     .LBB8_4
.LBB8_2:
    lc      r2, 2
    lc      r0, 0
    sw      r0, 24(fp)
    lc      r1, 1
.LBB8_3:
    sw      r1, 21(fp)
    lw      r1, 24(fp)
    lw      r0, 21(fp)
    add     r1, r0
    add     r2, 1
    clu     r0, r2
    lw      r0, 21(fp)
    sw      r0, 24(fp)
    brf     .LBB8_3
.LBB8_4:
    mov     r0, r1
    pop     r2
    jmp     (r2)
.Lfunc_end8:

; --- function: mmio_read ---
mmio_read:
    lw      r0, 0(r0)
    pop     r2
    jmp     (r2)
.Lfunc_end9:

; --- function: mmio_write ---
mmio_write:
    sw      r1, 0(r0)
    pop     r2
    jmp     (r2)
.Lfunc_end10:

; --- function: uart_putc ---
uart_putc:
    mov     r1, r0
    la      r0, 0xFF0100
    ; call mmio_write
    la      r2, .Lret_16
    push    r2
    la      r2, mmio_write
    jmp     (r2)
    .Lret_16:
    pop     r2
    jmp     (r2)
.Lfunc_end11:

