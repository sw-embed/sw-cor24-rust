; UART Echo: Interrupt-driven character echo
; For letters: prints uppercase then lowercase (a->Aa, B->Bb)
; For non-letters: prints two-digit hex code (? -> 3F, < -> 3C)
;
; Usage: Assemble & Run, then click the UART RX input and type.
; Each keystroke triggers an interrupt that echoes to UART TX.

; --- Setup interrupt vector ---
        la      r0, isr
        mov     iv, r0          ; r6 = ISR address

; --- Enable UART RX interrupt ---
        la      r1, 0xFF0010
        lc      r0, 1
        sb      r0, 0(r1)       ; enable UART RX interrupt

; --- Print prompt ---
        la      r1, 0xFF0100
        lc      r0, 0x3F        ; '?'
        sb      r0, 0(r1)       ; transmit prompt

; --- Main loop: spin forever (two instructions to avoid halt detection) ---
idle:   lc      r0, 0
        bra     idle

; --- Interrupt Service Routine ---
isr:
        push    r0
        push    r1
        push    r2
        mov     r2, c
        push    r2

        ; Read UART RX byte (acknowledges interrupt)
        la      r1, 0xFF0100
        lb      r0, 0(r1)      ; r0 = received character

        ; Check if letter: 'A'-'Z' (0x41-0x5A) or 'a'-'z' (0x61-0x7A)
        ; First check uppercase range
        lc      r2, 0x41        ; 'A'
        clu     r0, r2          ; char < 'A'?
        brt     not_letter      ; yes -> not a letter
        lc      r2, 0x5B        ; 'Z'+1
        clu     r0, r2          ; char < 'Z'+1?
        brt     is_letter       ; yes -> uppercase letter

        ; Check lowercase range
        lc      r2, 0x61        ; 'a'
        clu     r0, r2          ; char < 'a'?
        brt     not_letter
        lc      r2, 0x7B        ; 'z'+1
        clu     r0, r2          ; char < 'z'+1?
        brt     is_letter       ; yes -> lowercase letter
        bra     not_letter

is_letter:
        ; Make uppercase copy: clear bit 5
        lcu     r2, 0xDF        ; mask to clear bit 5
        and     r2, r0          ; r2 = uppercase version
        la      r1, 0xFF0100
        sb      r2, 0(r1)      ; transmit uppercase

        ; Make lowercase copy: set bit 5
        lcu     r2, 0x20        ; bit 5
        or      r2, r0          ; r2 = lowercase version
        sb      r2, 0(r1)      ; transmit lowercase
        bra     isr_done

not_letter:
        ; Print hex code: two hex digits
        ; High nibble first
        mov     r2, r0          ; r2 = original byte
        lc      r1, 4
        srl     r2, r1          ; r2 = high nibble (0-15)
        la      r1, hex_table
        add     r1, r2          ; r1 = &hex_table[high_nibble]
        lb      r2, 0(r1)      ; r2 = hex char
        la      r1, 0xFF0100
        sb      r2, 0(r1)      ; transmit high hex digit

        ; Low nibble
        lcu     r2, 0x0F
        and     r2, r0          ; r2 = low nibble
        la      r1, hex_table
        add     r1, r2
        lb      r2, 0(r1)      ; r2 = hex char
        la      r1, 0xFF0100
        sb      r2, 0(r1)      ; transmit low hex digit

isr_done:
        ; Restore registers
        pop     r2
        clu     z, r2           ; restore condition flag
        pop     r2
        pop     r1
        pop     r0
        jmp     (ir)            ; return from interrupt

; Hex digit lookup table
hex_table:
        .byte   0x30, 0x31, 0x32, 0x33  ; 0123
        .byte   0x34, 0x35, 0x36, 0x37  ; 4567
        .byte   0x38, 0x39, 0x41, 0x42  ; 89AB
        .byte   0x43, 0x44, 0x45, 0x46  ; CDEF
