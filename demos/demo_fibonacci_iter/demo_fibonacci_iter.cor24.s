; COR24 Assembly - Generated from MSP430 via msp430-to-cor24
; Pipeline: Rust -> rustc (msp430-none-elf) -> MSP430 ASM -> COR24 ASM

; Reset vector -> start
    mov     fp, sp
    la      r0, start
    jmp     (r0)

; --- function: _RNvCsgMG9zBUy57e_7___rustc17rust_begin_unwind ---
_RNvCsgMG9zBUy57e_7___rustc17rust_begin_unwind:
.LBB0_1:
    bra     .LBB0_1
.Lfunc_end0:

; --- function: demo_fibonacci_iter ---
demo_fibonacci_iter:
    lc      r0, 10
    ; call fibonacci_iter
    push    r1
    la      r2, fibonacci_iter
    jal     r1, (r2)
    pop     r1
    sw      r0, 24(fp)
    la      r0, 256
    ; call mem_write
    push    r1
    la      r2, mem_write
    jal     r1, (r2)
    pop     r1
.LBB1_1:
    bra     .LBB1_1
.Lfunc_end1:

; --- function: fibonacci_iter ---
fibonacci_iter:
    push    r1
    lc      r1, 1
    lc      r2, 1
    ceq     r0, z
    brt     .fib_done
.fib_loop:
    push    r1
    mov     r1, r2
    pop     r2
    add     r2, r1
    add     r0, -1
    ceq     r0, z
    brf     .fib_loop
.fib_done:
    mov     r0, r1
    pop     r1
    jmp     (r1)
.Lfunc_end2:

; --- function: mem_write ---
mem_write:
    lw      r2, 24(fp)
    sb      r2, 0(r0)
    jmp     (r1)
.Lfunc_end3:

; --- function: start ---
start:
    ; call demo_fibonacci_iter
    push    r1
    la      r2, demo_fibonacci_iter
    jal     r1, (r2)
    pop     r1
.Lfunc_end4:

