; Variables: Copy "Hello" to three locations
; Hover memory rows to see ASCII tooltip
;
; 1. Constant data at the end of code
; 2. SRAM variable at address 256
; 3. Stack (EBR) via push
;
; After running, check memory viewer:
;   SRAM ~0x0030: source "Hello" in code
;   SRAM 0x0100: copied to variable
;   EBR/Stack: pushed onto stack

; --- Copy from source to SRAM variable ---
        la      r1,src        ; source address
        la      r2,256        ; destination

        lb      r0,0(r1)      ; 'H'
        sb      r0,0(r2)
        lb      r0,1(r1)      ; 'e'
        sb      r0,1(r2)
        lb      r0,2(r1)      ; 'l'
        sb      r0,2(r2)
        lb      r0,3(r1)      ; 'l'
        sb      r0,3(r2)
        lb      r0,4(r1)      ; 'o'
        sb      r0,4(r2)

; --- Push "Hello" onto stack ---
        lcu     r0,72         ; 'H'
        push    r0
        lcu     r0,101        ; 'e'
        push    r0
        lcu     r0,108        ; 'l'
        push    r0
        lcu     r0,108        ; 'l'
        push    r0
        lcu     r0,111        ; 'o'
        push    r0

halt:
        bra     halt

; --- Constant data (in code segment) ---
src:
        .byte   72,101,108,108,111  ; "Hello"
