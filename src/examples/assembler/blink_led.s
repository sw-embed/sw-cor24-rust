; Blink LED: Toggle LED D2
; Hover D2 to see duty cycle (~50%)
; Use Step to watch each instruction
; Use Run speed slider to control blink rate

        la      r1,-65536   ; LED I/O address

loop:
        lc      r0,1
        sb      r0,0(r1)    ; LED on
        lc      r0,0
        sb      r0,0(r1)    ; LED off
        bra     loop
