; COR24 Assembly - Generated from MSP430 via msp430-to-cor24
; Pipeline: Rust -> rustc (msp430-none-elf) -> MSP430 ASM -> COR24 ASM

; Reset vector -> start
    mov     fp, sp
    la      r0, start
    jmp     (r0)

; --- function: demo_add ---
demo_add:
    la      r0, 0x000156
    pop     r2
    jmp     (r2)
.Lfunc_end0:

; --- function: start ---
start:
    ; call demo_add
    la      r2, .Lret_0
    push    r2
    la      r2, demo_add
    jmp     (r2)
    .Lret_0:
    ; store result to memory at 0x0100
    la      r1, 0x000100
    sw      r0, 0(r1)
.LBB1_1:
    bra     .LBB1_1
.Lfunc_end1:

; --- function: panic handler ---
_RNvCsgMG9zBUy57e_7___rustc17rust_begin_unwind:
.LBB2_1:
    bra     .LBB2_1
.Lfunc_end2:
