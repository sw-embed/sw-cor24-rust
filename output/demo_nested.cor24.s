; COR24 Assembly - Generated from MSP430 via msp430-to-cor24
; Pipeline: Rust -> rustc (msp430-none-elf) -> MSP430 ASM -> COR24 ASM

; Reset vector -> demo_nested
    la      r0, demo_nested
    jmp     (r0)

; --- function: _RNvCsgMG9zBUy57e_7___rustc17rust_begin_unwind ---
_RNvCsgMG9zBUy57e_7___rustc17rust_begin_unwind:
.LBB0_1:
    bra     .LBB0_1
.Lfunc_end0:

; --- function: accumulate ---
accumulate:
    lw      r0, 6(fp)
    push    r0
    lw      r0, 9(fp)
    push    r0
    lw      r0, 12(fp)
    push    r0
    lw      r0, 15(fp)
    push    r0
    lw      r0, 18(fp)
    push    r0
    sw      r0, 18(fp)
    lw      r0, 18(fp)
    sw      r0, 6(fp)
    lw      r0, 6(fp)
    add     r0, 1
    sw      r0, 6(fp)
    lw      r0, 18(fp)
    lw      r1, 6(fp)
    add     r0, r1
    sw      r0, 18(fp)
    lw      r0, 18(fp)
    sw      r0, 15(fp)
    lw      r0, 15(fp)
    lw      r1, 6(fp)
    add     r0, r1
    sw      r0, 15(fp)
    lw      r1, 18(fp)
    lw      r0, 6(fp)
    xor     r1, r0
    lw      r0, 15(fp)
    xor     r1, r0
    lw      r0, 15(fp)
    sw      r0, 12(fp)
    lw      r0, 12(fp)
    lw      r1, 18(fp)
    add     r0, r1
    sw      r0, 12(fp)
    lw      r0, 12(fp)
    xor     r1, r0
    lw      r0, 12(fp)
    sw      r0, 9(fp)
    lw      r0, 9(fp)
    lw      r1, 15(fp)
    add     r0, r1
    sw      r0, 9(fp)
    lw      r0, 9(fp)
    xor     r1, r0
    la      r0, 0xFF0000
    ; call mmio_write
    la      r2, .Lret_0
    push    r2
    la      r2, mmio_write
    jmp     (r2)
    .Lret_0:
    lw      r0, 6(fp)
    ; call uart_putc
    la      r2, .Lret_1
    push    r2
    la      r2, uart_putc
    jmp     (r2)
    .Lret_1:
    lw      r0, 18(fp)
    ; call uart_putc
    la      r2, .Lret_2
    push    r2
    la      r2, uart_putc
    jmp     (r2)
    .Lret_2:
    lw      r0, 15(fp)
    ; call uart_putc
    la      r2, .Lret_3
    push    r2
    la      r2, uart_putc
    jmp     (r2)
    .Lret_3:
    lw      r0, 12(fp)
    ; call uart_putc
    la      r2, .Lret_4
    push    r2
    la      r2, uart_putc
    jmp     (r2)
    .Lret_4:
    lw      r0, 9(fp)
    ; call uart_putc
    la      r2, .Lret_5
    push    r2
    la      r2, uart_putc
    jmp     (r2)
    .Lret_5:
.LBB1_1:
    bra     .LBB1_1
.Lfunc_end1:

; --- function: delay ---
delay:
    sub     sp, 3
    ceq     r0, z
    brt     .LBB2_3
    add     r0, -1
.LBB2_2:
    mov     r2, sp
    sw      r0, 0(r2)
    add     r0, -1
    lc      r1, -1
    ceq     r0, r1
    brf     .LBB2_2
.LBB2_3:
    add     sp, 3
    pop     r2
    jmp     (r2)
.Lfunc_end2:

; --- function: demo_add ---
demo_add:
    la      r0, 0x000156
    pop     r2
    jmp     (r2)
.Lfunc_end3:

; --- function: demo_blinky ---
demo_blinky:
.LBB4_1:
    la      r0, 0xFF0000
    lc      r1, 1
    ; call mmio_write
    la      r2, .Lret_6
    push    r2
    la      r2, mmio_write
    jmp     (r2)
    .Lret_6:
    la      r0, 0x001388
    ; call delay
    la      r2, .Lret_7
    push    r2
    la      r2, delay
    jmp     (r2)
    .Lret_7:
    la      r0, 0xFF0000
    lc      r1, 0
    ; call mmio_write
    la      r2, .Lret_8
    push    r2
    la      r2, mmio_write
    jmp     (r2)
    .Lret_8:
    la      r0, 0x001388
    ; call delay
    la      r2, .Lret_9
    push    r2
    la      r2, delay
    jmp     (r2)
    .Lret_9:
    bra     .LBB4_1
.Lfunc_end4:

; --- function: demo_button_echo ---
demo_button_echo:
.LBB5_1:
    la      r0, 0xFF0000
    ; call mmio_read
    la      r2, .Lret_10
    push    r2
    la      r2, mmio_read
    jmp     (r2)
    .Lret_10:
    mov     r1, r0
    lc      r0, 1
    and     r1, r0
    la      r0, 0xFF0000
    ; call mmio_write
    la      r2, .Lret_11
    push    r2
    la      r2, mmio_write
    jmp     (r2)
    .Lret_11:
    bra     .LBB5_1
.Lfunc_end5:

; --- function: demo_countdown ---
demo_countdown:
    lw      r0, 18(fp)
    push    r0
    lc      r0, 10
    sw      r0, 18(fp)
.LBB6_1:
    la      r0, 0xFF0000
    lw      r1, 18(fp)
    ; call mmio_write
    la      r2, .Lret_12
    push    r2
    la      r2, mmio_write
    jmp     (r2)
    .Lret_12:
    la      r0, 0x0003E8
    ; call delay
    la      r2, .Lret_13
    push    r2
    la      r2, delay
    jmp     (r2)
    .Lret_13:
    lw      r0, 18(fp)
    add     r0, -1
    sw      r0, 18(fp)
    lw      r0, 18(fp)
    ceq     r0, z
    brf     .LBB6_1
    la      r0, 0xFF0000
    lc      r1, 0
    ; call mmio_write
    la      r2, .Lret_14
    push    r2
    la      r2, mmio_write
    jmp     (r2)
    .Lret_14:
.LBB6_3:
    bra     .LBB6_3
.Lfunc_end6:

; --- function: demo_fibonacci ---
demo_fibonacci:
    la      r0, 0xFF0000
    lc      r1, 55
    ; call mmio_write
    la      r2, .Lret_15
    push    r2
    la      r2, mmio_write
    jmp     (r2)
    .Lret_15:
.LBB7_1:
    bra     .LBB7_1
.Lfunc_end7:

; --- function: demo_nested ---
demo_nested:
    la      r0, 0xFF0000
    ; call mmio_read
    la      r2, .Lret_16
    push    r2
    la      r2, mmio_read
    jmp     (r2)
    .Lret_16:
    add     r0, 5
    ; call level_a
    la      r2, .Lret_17
    push    r2
    la      r2, level_a
    jmp     (r2)
    .Lret_17:
.Lfunc_end8:

; --- function: demo_stack_vars ---
demo_stack_vars:
    la      r0, 0xFF0000
    ; call mmio_read
    la      r2, .Lret_18
    push    r2
    la      r2, mmio_read
    jmp     (r2)
    .Lret_18:
    add     r0, 1
    ; call accumulate
    la      r2, .Lret_19
    push    r2
    la      r2, accumulate
    jmp     (r2)
    .Lret_19:
.Lfunc_end9:

; --- function: demo_uart_hello ---
demo_uart_hello:
    lc      r0, 72
    ; call uart_putc
    la      r2, .Lret_20
    push    r2
    la      r2, uart_putc
    jmp     (r2)
    .Lret_20:
    lc      r0, 101
    ; call uart_putc
    la      r2, .Lret_21
    push    r2
    la      r2, uart_putc
    jmp     (r2)
    .Lret_21:
    lc      r0, 108
    ; call uart_putc
    la      r2, .Lret_22
    push    r2
    la      r2, uart_putc
    jmp     (r2)
    .Lret_22:
    lc      r0, 108
    ; call uart_putc
    la      r2, .Lret_23
    push    r2
    la      r2, uart_putc
    jmp     (r2)
    .Lret_23:
    lc      r0, 111
    ; call uart_putc
    la      r2, .Lret_24
    push    r2
    la      r2, uart_putc
    jmp     (r2)
    .Lret_24:
    lc      r0, 10
    ; call uart_putc
    la      r2, .Lret_25
    push    r2
    la      r2, uart_putc
    jmp     (r2)
    .Lret_25:
.LBB10_1:
    bra     .LBB10_1
.Lfunc_end10:

; --- function: fibonacci ---
fibonacci:
    lc      r1, 2
    clu     r0, r1
    brf     .LBB11_2
    mov     r1, r0
    bra     .LBB11_4
.LBB11_2:
    lc      r2, 2
    lc      r0, 0
    sw      r0, 24(fp)
    lc      r1, 1
.LBB11_3:
    sw      r1, 21(fp)
    lw      r1, 24(fp)
    lw      r0, 21(fp)
    add     r1, r0
    add     r2, 1
    clu     r0, r2
    lw      r0, 21(fp)
    sw      r0, 24(fp)
    brf     .LBB11_3
.LBB11_4:
    mov     r0, r1
    pop     r2
    jmp     (r2)
.Lfunc_end11:

; --- function: level_a ---
level_a:
    add     r0, 10
    ; call level_b
    la      r2, .Lret_26
    push    r2
    la      r2, level_b
    jmp     (r2)
    .Lret_26:
.Lfunc_end12:

; --- function: level_b ---
level_b:
    mov     r1, r0
    add     r0, r0
    add     r0, 3
    ; call level_c
    la      r2, .Lret_27
    push    r2
    la      r2, level_c
    jmp     (r2)
    .Lret_27:
.Lfunc_end13:

; --- function: level_c ---
level_c:
    lw      r0, 18(fp)
    push    r0
    sw      r1, 18(fp)
    mov     r1, r0
    la      r0, 0xFF0000
    ; call mmio_write
    la      r2, .Lret_28
    push    r2
    la      r2, mmio_write
    jmp     (r2)
    .Lret_28:
    lw      r0, 18(fp)
    ; call uart_putc
    la      r2, .Lret_29
    push    r2
    la      r2, uart_putc
    jmp     (r2)
    .Lret_29:
.LBB14_1:
    bra     .LBB14_1
.Lfunc_end14:

; --- function: mmio_read ---
mmio_read:
    lw      r0, 0(r0)
    pop     r2
    jmp     (r2)
.Lfunc_end15:

; --- function: mmio_write ---
mmio_write:
    sw      r1, 0(r0)
    pop     r2
    jmp     (r2)
.Lfunc_end16:

; --- function: uart_putc ---
uart_putc:
    mov     r1, r0
    la      r0, 0xFF0100
    ; call mmio_write
    la      r2, .Lret_30
    push    r2
    la      r2, mmio_write
    jmp     (r2)
    .Lret_30:
    pop     r2
    jmp     (r2)
.Lfunc_end17:

