; OOM: Fill SRAM with incrementing counter (256-byte stride)
; Writes one byte every 256 addresses from 0x0100 to top of SRAM (0x100000)
; Demonstrates sparse memory display with zero-row collapsing
; Click Run to watch memory fill up, then halt when SRAM is exhausted

        la   r1, 0x0100    ; start address (past program code)
        lc   r0, 1         ; counter starts at 1

loop:   sb   r0, 0(r1)     ; store counter byte at current address
        lc   r2, 1
        add  r0, r2         ; counter++ (wraps naturally as byte)
        lcu  r2, 128
        add  r1, r2
        add  r1, r2         ; address += 256
        la   r2, 0x100000   ; top of SRAM (1 MB)
        clu  r1, r2         ; address < top?
        brt  loop           ; yes -> keep writing

halt:   bra  halt           ; out of memory - halt
