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
    push    r2
    lc      r2, -1
    ceq     r0, r2
    pop     r2
    brf     .LBB1_2
.LBB1_3:
    add     sp, 3
    jmp     (r1)
.Lfunc_end1:

; --- function: demo_countdown ---
demo_countdown:
    sw      r0, 30(fp)
    lw      r0, 18(fp)
    push    r0
    lw      r0, 30(fp)
    push    r0
    lc      r0, 10
    sw      r0, 18(fp)
    pop     r0
.LBB2_1:
    la      r0, 256
    push    r0
    lw      r0, 18(fp)
    sw      r0, 24(fp)
    pop     r0
    ; call mem_write
    push    r1
    la      r2, mem_write
    jal     r1, (r2)
    pop     r1
    lc      r0, 10
    ; call delay
    push    r1
    la      r2, delay
    jal     r1, (r2)
    pop     r1
    push    r0
    lw      r0, 18(fp)
    add     r0, -1
    sw      r0, 18(fp)
    pop     r0
    push    r0
    lw      r0, 18(fp)
    ceq     r0, z
    pop     r0
    brf     .LBB2_1
    la      r0, 256
    push    r0
    lc      r0, 0
    sw      r0, 24(fp)
    pop     r0
    ; call mem_write
    push    r1
    la      r2, mem_write
    jal     r1, (r2)
    pop     r1
.LBB2_3:
    bra     .LBB2_3
.Lfunc_end2:

; --- function: mem_write ---
mem_write:
    lw      r2, 24(fp)
    sb      r2, 0(r0)
    jmp     (r1)
.Lfunc_end3:

; --- function: start ---
start:
    ; call demo_countdown
    push    r1
    la      r2, demo_countdown
    jal     r1, (r2)
    pop     r1
.Lfunc_end4:

