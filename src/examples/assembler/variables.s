; Variables: "Hello World!" in memory
; Hover SRAM rows to see ASCII tooltip
;
; Three copies visible in memory viewer:
;   1. Source string in code (.byte data)
;   2. Copy at address 256 (byte-by-byte)
;   3. Copy at address 512 (byte-by-byte)

        la      r1,src        ; source address
        la      r2,256        ; destination 1

        ; Copy 12 bytes: "Hello World!"
        lc      r0,12         ; length
copy1:
        push    r0            ; save counter
        push    r2            ; save dest
        lb      r0,0(r1)     ; load byte from source
        sb      r0,0(r2)     ; store to dest
        add     r1,1          ; src++
        pop     r2
        add     r2,1          ; dest++
        pop     r0
        add     r0,-1         ; count--
        ceq     r0,z
        brf     copy1

        ; Second copy to address 512
        la      r1,src
        la      r2,512
        lc      r0,12
copy2:
        push    r0
        push    r2
        lb      r0,0(r1)
        sb      r0,0(r2)
        add     r1,1
        pop     r2
        add     r2,1
        pop     r0
        add     r0,-1
        ceq     r0,z
        brf     copy2

halt:
        bra     halt

; --- Source string (contiguous bytes in code) ---
src:
        .byte 72 101 108 108 111 32 87 111 114 108 100 33
        ; H  e   l   l   o  ' ' W   o   r   l   d   !
