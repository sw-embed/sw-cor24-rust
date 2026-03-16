; Multiply: 6 x 7 = 42, two ways
; First using native mul, then via loop
; Prints "42 42\n" to UART

        ; --- Method 1: native mul instruction ---
        lc      r0,6
        lc      r1,7
        mul     r0,r1           ; r0 = 42

        ; Print result
        la      r2,print2
        jal     r1,(r2)

        ; Print space
        push    r0
        lc      r0,32           ; ' '
        la      r2,putc
        jal     r1,(r2)
        pop     r0

        ; --- Method 2: repeated addition loop ---
        lc      r0,0            ; sum = 0
        lc      r1,7            ; counter = 7
loop:
        add     r0,6            ; sum += 6
        push    r0
        lc      r0,1
        sub     r1,r0           ; counter--
        pop     r0
        ceq     r1,z
        brf     loop            ; loop while counter != 0

        ; Print result
        la      r2,print2
        jal     r1,(r2)

        ; Print newline
        lc      r0,10           ; '\n'
        la      r2,putc
        jal     r1,(r2)

halt:
        bra     halt

; print2: print r0 as 1-2 digit decimal
; Uses jal convention: r1 = return address
print2:
        push    r1
        lc      r1,0            ; tens = 0
.div10:
        lc      r2,10
        clu     r0,r2           ; r0 < 10?
        brt     .ones           ; yes: r0=ones, r1=tens
        sub     r0,r2           ; r0 -= 10
        add     r1,1            ; tens++
        bra     .div10
.ones:
        push    r0              ; save ones
        ceq     r1,z
        brt     .notens
        push    r1
        lc      r0,48           ; '0'
        add     r0,r1           ; '0' + tens
        la      r2,putc
        jal     r1,(r2)
        pop     r1
.notens:
        pop     r0
        lc      r1,48
        add     r0,r1           ; '0' + ones
        la      r2,putc
        jal     r1,(r2)
        pop     r1
        jmp     (r1)

; putc: send byte in r0, polling TX busy
; Uses jal convention: r1 = return address
putc:
        push    r1
        push    r0
        la      r1,-65280       ; UART base
.wait:
        lb      r2,1(r1)        ; read status byte
        lcu     r0,128
        and     r2,r0           ; bit 7 = TX busy
        ceq     r2,z
        brf     .wait
        pop     r0
        sb      r0,0(r1)        ; transmit
        pop     r1
        jmp     (r1)
