; Stack Overflow: Infinite recursion filling the EBR/Stack region
; Each "call" pushes a frame (return addr + depth counter) and recurses
; Halts when SP drops below bottom of EBR (0xFEE000)
; Click Run to watch the stack fill from top (0xFEEC00) downward

        lc   r0, 0         ; recursion depth = 0
        bra  recurse

recurse:
        push r0             ; save depth on stack (SP -= 3)
        push r0             ; simulate saving a local variable too
        lc   r2, 1
        add  r0, r2         ; depth++

        ; check if stack still in EBR region
        mov  r1, sp         ; r1 = current SP
        la   r2, 0xFEE000   ; bottom of EBR
        clu  r2, r1         ; bottom < SP? (still room)
        brt  recurse        ; yes -> recurse deeper

halt:   bra  halt           ; stack overflow - halt!
