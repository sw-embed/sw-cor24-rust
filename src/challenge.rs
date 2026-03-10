//! Challenge system for COR24 emulator

use crate::cpu::CpuState;

/// A challenge for the user to complete
#[derive(Clone)]
pub struct Challenge {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub initial_code: String,
    pub hint: String,
    pub validator: fn(&CpuState) -> bool,
}

/// Get all available challenges
pub fn get_challenges() -> Vec<Challenge> {
    vec![
        Challenge {
            id: 1,
            name: "Load and Add".to_string(),
            description: "Load the value 10 into r0, then add 5 to it. Result should be 15 in r0."
                .to_string(),
            initial_code: "; Load 10 into r0, add 5\n; Result: r0 = 15\n\n".to_string(),
            hint: "Use 'lc r0,10' to load 10, then 'add r0,5' to add 5".to_string(),
            validator: |cpu| cpu.get_reg(0) == 15,
        },
        Challenge {
            id: 2,
            name: "Compare and Branch".to_string(),
            description: "Set r0 to 1 if 5 < 10 (signed), otherwise 0. Use cls and brt/brf."
                .to_string(),
            initial_code: "; Compare 5 < 10 and set r0 accordingly\n; Result: r0 = 1\n\n"
                .to_string(),
            hint: "Load values, use cls to compare, then mov r0,c to get the result".to_string(),
            validator: |cpu| cpu.get_reg(0) == 1,
        },
        Challenge {
            id: 3,
            name: "Stack Operations".to_string(),
            description: "Push values 1, 2, 3 onto the stack, then pop them into r0, r1, r2."
                .to_string(),
            initial_code: "; Push 1, 2, 3 then pop into r0, r1, r2\n; Result: r0=3, r1=2, r2=1\n\n"
                .to_string(),
            hint: "Remember LIFO order - last pushed is first popped".to_string(),
            validator: |cpu| cpu.get_reg(0) == 3 && cpu.get_reg(1) == 2 && cpu.get_reg(2) == 1,
        },
        Challenge {
            id: 4,
            name: "Max of Two".to_string(),
            description: "Set r0 to the maximum of r0=7 and r1=12 (without branching). Use mov ra,c!"
                .to_string(),
            initial_code: "; Find max of r0=7 and r1=12, store result in r0\n; Hint: Use COR24's mov ra,c feature\n; Result: r0 = 12\n\n        lc      r0,7\n        lc      r1,12\n\n        ; Your code here\n\nhalt:   bra     halt\n"
                .to_string(),
            hint: "cls sets C if r0 < r1. If true, you want r1. Use sub/add with C flag.".to_string(),
            validator: |cpu| cpu.get_reg(0) == 12,
        },
        Challenge {
            id: 5,
            name: "Byte Sign Extension".to_string(),
            description: "Load -50 (0xCE) as unsigned into r0, then sign-extend it. Result should be 0xFFFFCE."
                .to_string(),
            initial_code: "; Load 0xCE unsigned, then sign extend\n; Result: r0 = 0xFFFFCE (-50)\n\n"
                .to_string(),
            hint: "Use lcu to load unsigned, then sxt to sign extend".to_string(),
            validator: |cpu| cpu.get_reg(0) == 0xFFFFCE,
        },
    ]
}

/// Get example programs
pub fn get_examples() -> Vec<(String, String, String)> {
    vec![
        (
            "Add".to_string(),
            "Compute 100 + 200 + 42 = 342, return in r0".to_string(),
            r#"; Add: Compute 100 + 200 + 42 = 342
; Result in r0

        lc      r0,100      ; r0 = 100
        lcu     r1,200      ; r1 = 200 (unsigned, >127)
        add     r0,r1       ; r0 = 300
        lc      r1,42       ; r1 = 42
        add     r0,r1       ; r0 = 342 (0x156)

halt:   bra     halt
"#
            .to_string(),
        ),
        (
            "Blink LED".to_string(),
            "Toggle LED with delay loop".to_string(),
            r#"; Blink LED: Toggle LED D2 on and off
; LED D2 at 0xFF0000 (write bit 0)
; Click Run to watch the LED blink!

        la      r1,0xFF0000

loop:
        lc      r0,1
        sb      r0,0(r1)

        push    r1
        lc      r1,10
delay1: lc      r2,0
wait1:  lc      r0,1
        add     r2,r0
        lc      r0,127
        clu     r2,r0
        brt     wait1
        lc      r0,1
        sub     r1,r0
        ceq     r1,z
        brf     delay1
        pop     r1

        lc      r0,0
        sb      r0,0(r1)

        push    r1
        lc      r1,10
delay2: lc      r2,0
wait2:  lc      r0,1
        add     r2,r0
        lc      r0,127
        clu     r2,r0
        brt     wait2
        lc      r0,1
        sub     r1,r0
        ceq     r1,z
        brf     delay2
        pop     r1

        bra     loop

halt:   bra     halt
"#
            .to_string(),
        ),
        (
            "Button Echo".to_string(),
            "LED D2 follows button S2".to_string(),
            r#"; Button Echo: LED follows button state
; LED D2 lights when button S2 is pressed
; Click S2 button in I/O panel while running
;
; S2 is active-low (normally 1, pressed = 0)
; We invert with XOR so LED on = button pressed

        la      r1,0xFF0000 ; I/O address (LEDSWDAT)
        lc      r2,1        ; Bit mask for XOR

loop:
        lb      r0,0(r1)    ; Read button S2 (bit 0: 1=released, 0=pressed)
        xor     r0,r2       ; Invert: pressed(0)->1(LED on), released(1)->0(LED off)
        sb      r0,0(r1)    ; Write to LED D2 (bit 0)

        bra     loop        ; Keep polling

halt:   bra     halt        ; Never reached
"#
            .to_string(),
        ),
        (
            "Countdown".to_string(),
            "Count 10→0 on LED, then halt".to_string(),
            r#"; Countdown: Display 10 down to 0 on LED
; Writes count to LED register, delays, decrements

        la      r1,0xFF0000 ; LED address
        lc      r0,10       ; Start at 10

loop:   sb      r0,0(r1)    ; Write count to LED

        ; Delay loop
        push    r0
        lc      r2,0
wait:   add     r2,1
        lc      r0,127
        clu     r2,r0
        brt     wait
        pop     r0

        lc      r2,1
        sub     r0,r2       ; count--
        ceq     r0,z        ; count == 0?
        brf     loop        ; Continue if not zero

        ; Clear LED and halt
        lc      r0,0
        sb      r0,0(r1)
halt:   bra     halt
"#
            .to_string(),
        ),
        (
            "Echo".to_string(),
            "Interrupt-driven UART echo (letters→Aa, non-letters→hex)".to_string(),
            r#"; UART Echo: Interrupt-driven character echo
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
"#
            .to_string(),
        ),
        (
            "Fibonacci".to_string(),
            "Print fib(1)..fib(10) to UART".to_string(),
            r#"; Fibonacci: Print series 1 1 2 3 5 8 13 21 34 55
; UART TX: "1 1 2 3 5 8 13 21 34 55\n"

        lc      r0,0            ; a = 0
        lc      r1,1            ; b = 1
        lc      r2,10           ; 10 iterations

loop:   push    r0              ; save a
        push    r2              ; save counter
        ; print b (current fib number)
        mov     r0,r1           ; r0 = b (value to print)
        push    r1              ; save b
        la      r2,pret
        push    r2
        bra     print_num
pret:   pop     r1              ; restore b
        pop     r2              ; restore counter

        ; print space (unless last iteration)
        push    r0
        lc      r0,1
        ceq     r0,r2           ; 1 == counter? (last)
        pop     r0
        brt     skip_sp
        push    r1
        push    r2
        lc      r0,32           ; ' '
        la      r2,spret
        push    r2
        bra     putc
spret:  pop     r2
        pop     r1
skip_sp:
        ; advance: a=old_b, b=old_a+old_b
        pop     r0              ; restore old a
        push    r1              ; save old b
        add     r1,r0           ; b = a + b (new fib)
        pop     r0              ; a = old b

        push    r0
        lc      r0,1
        sub     r2,r0           ; counter--
        pop     r0
        ceq     r2,z
        brf     loop

        ; print newline
        lc      r0,10
        la      r2,halt
        push    r2
        bra     putc

halt:   bra     halt

; print_num: print r0 as 1-2 digit decimal
; clobbers r0,r1,r2
print_num:
        lc      r1,0            ; tens = 0
.div:   lc      r2,10
        clu     r0,r2           ; r0 < 10?
        brt     .ones           ; yes, r0 = ones digit
        sub     r0,r2           ; r0 -= 10
        add     r1,1            ; tens++
        bra     .div
.ones:  push    r0              ; save ones
        ; print tens if nonzero
        ceq     r1,z
        brt     .notens
        push    r1
        lc      r0,48
        add     r0,r1           ; '0' + tens
        la      r2,.tret
        push    r2
        bra     putc
.tret:  pop     r1
.notens:
        pop     r0              ; ones
        lc      r1,48
        add     r0,r1           ; '0' + ones
        la      r2,.oret
        push    r2
        bra     putc
.oret:  pop     r2
        jmp     (r2)            ; return

; putc: send byte in r0, polling TX busy first
putc:   push    r0              ; save char
        la      r1,0xFF0100     ; UART base
.wait:  lb      r2,1(r1)        ; read status byte
        lcu     r0,128
        and     r2,r0           ; isolate bit 7
        ceq     r2,z
        brf     .wait           ; spin while TX busy
        pop     r0              ; restore char
        sb      r0,0(r1)        ; transmit byte
        pop     r2              ; return address
        jmp     (r2)
"#
            .to_string(),
        ),
        (
            "Memory Access".to_string(),
            "Store to non-adjacent memory blocks".to_string(),
            r#"; Memory Access: Store and load from non-adjacent regions
; Writes to 0x0100 and 0x0200 (256 bytes apart)
; Demonstrates memory viewer zero-row collapsing

        lc      r0,42       ; First value
        la      r1,0x0100   ; First address block

        ; Store to first block at 0x0100
        sb      r0,0(r1)    ; mem[0x0100] = 42
        sb      r0,1(r1)    ; mem[0x0101] = 42

        lcu     r0,200      ; Second value
        la      r1,0x0200   ; Second address block (256 bytes away)

        ; Store to second block at 0x0200
        sb      r0,0(r1)    ; mem[0x0200] = 200
        sw      r0,4(r1)    ; mem[0x0204..06] = 200

        ; Load them back to verify
        la      r1,0x0100
        lb      r2,0(r1)    ; r2 = 42
        la      r1,0x0200
        lw      r2,4(r1)    ; r2 = 200

halt:   bra     halt
"#
            .to_string(),
        ),
        (
            "Multiply".to_string(),
            "6 × 7 = 42 via loop, print to UART".to_string(),
            r#"; Multiply: 6 × 7 = 42 via repeated addition
; Prints "42\n" to UART

        lc      r0,0            ; sum = 0
        lc      r1,7            ; counter = 7
loop:   add     r0,6            ; sum += 6
        push    r0              ; save sum
        lc      r0,1
        sub     r1,r0           ; counter--
        pop     r0              ; restore sum
        ceq     r1,z
        brf     loop            ; loop while counter != 0

        ; r0 = 42, divide by 10 (repeated subtraction)
        lc      r1,0            ; tens = 0
div10:  lc      r2,10
        clu     r0,r2           ; r0 < 10?
        brt     done            ; yes: r0=ones, r1=tens
        sub     r0,r2           ; r0 -= 10
        add     r1,1            ; tens++
        bra     div10
done:
        ; Print tens digit
        push    r0              ; save ones
        lc      r0,48           ; '0'
        add     r0,r1           ; r0 = '0' + tens
        la      r2,ret1
        push    r2
        bra     putc
ret1:
        ; Print ones digit
        pop     r0              ; restore ones
        lc      r1,48           ; '0'
        add     r0,r1           ; r0 = '0' + ones
        la      r2,ret2
        push    r2
        bra     putc
ret2:
        ; Print newline
        lc      r0,10           ; '\n'
        la      r2,halt
        push    r2
        bra     putc

halt:   bra     halt

; putc: send byte in r0, polling TX busy first
putc:   push    r0              ; save char
        la      r1,0xFF0100     ; UART base
.wait:  lb      r2,1(r1)        ; read status byte
        lcu     r0,128
        and     r2,r0           ; isolate bit 7
        ceq     r2,z
        brf     .wait           ; spin while TX busy
        pop     r0              ; restore char
        sb      r0,0(r1)        ; transmit byte
        pop     r2              ; return address
        jmp     (r2)
"#
            .to_string(),
        ),
        (
            "Nested Calls".to_string(),
            "Function call chain showing stack frames".to_string(),
            r#"; Nested Calls: 3-level function call chain
; main -> level_a -> level_b, showing stack frames
; Result: r0 = ((5 + 10) * 2) + 3 = 33

        ; --- main ---
        lc      r0,5            ; arg = 5
        la      r1,ret_a        ; return address
        la      r2,level_a
        jal     r1,(r2)         ; call level_a(5)
ret_a:
halt:   bra     halt            ; r0 = 33

        ; --- level_a(x): returns level_b(x + 10) ---
level_a:
        push    fp
        push    r1              ; save return addr
        mov     fp,sp
        add     r0,10           ; x + 10 = 15
        la      r1,ret_b
        la      r2,level_b
        jal     r1,(r2)         ; call level_b(15)
ret_b:  mov     sp,fp
        pop     r1              ; restore return addr
        pop     fp
        jmp     (r1)            ; return

        ; --- level_b(x): returns x * 2 + 3 ---
level_b:
        push    fp
        push    r1              ; save return addr
        mov     fp,sp
        add     r0,r0           ; x * 2 = 30
        add     r0,3            ; + 3 = 33
        mov     sp,fp
        pop     r1
        pop     fp
        jmp     (r1)            ; return
"#
            .to_string(),
        ),
        (
            "Stack Variables".to_string(),
            "Local variables and register spilling".to_string(),
            r#"; Stack Variables: Local vars on the stack
; Demonstrates register spilling via push/pop
;
; Computes: a=seed+1, b=a+seed, c=b+a, result=a^b^c
; with seed=7: a=8, b=15, c=23, result=8^15^23=16

        lc      r0,7            ; seed = 7
        la      r1,ret_main
        la      r2,compute
        jal     r1,(r2)         ; call compute(7)
ret_main:
        ; r0 = result (16 = 0x10)
        la      r1,0xFF0000
        sb      r0,0(r1)        ; Display on LED
halt:   bra     halt

        ; --- compute(seed in r0) ---
        ; Uses r0-r2 for values, spills to stack when
        ; we run out of registers
compute:
        push    r1              ; spill return addr

        ; a = seed + 1
        mov     r1,r0           ; r1 = seed (keep copy)
        add     r0,1            ; r0 = a = 8

        ; b = a + seed
        mov     r2,r0           ; r2 = a (save)
        add     r0,r1           ; r0 = b = a + seed = 15

        ; c = b + a  (need a, but r2 has it)
        push    r0              ; spill b — out of regs
        add     r0,r2           ; r0 = c = b + a = 23

        ; result = a ^ b ^ c
        xor     r2,r0           ; r2 = a ^ c
        pop     r0              ; restore b
        xor     r2,r0           ; r2 = a ^ c ^ b = 16
        mov     r0,r2           ; r0 = result

        pop     r1              ; restore return addr
        jmp     (r1)
"#
            .to_string(),
        ),
        (
            "UART Hello".to_string(),
            "Write \"Hello\\n\" to UART output".to_string(),
            r#"; UART Hello: Send "Hello\n" via UART
; UART data at 0xFF0100, status at 0xFF0101
; Poll TX busy (bit 7) before each byte

        lc      r0,72           ; 'H'
        la      r2,next1
        push    r2
        bra     putc
next1:  lc      r0,101          ; 'e'
        la      r2,next2
        push    r2
        bra     putc
next2:  lc      r0,108          ; 'l'
        la      r2,next3
        push    r2
        bra     putc
next3:  lc      r0,108          ; 'l'
        la      r2,next4
        push    r2
        bra     putc
next4:  lc      r0,111          ; 'o'
        la      r2,next5
        push    r2
        bra     putc
next5:  lc      r0,10           ; '\n'
        la      r2,halt
        push    r2
        bra     putc

halt:   bra     halt

; putc: send byte in r0, polling TX busy first
putc:   push    r0              ; save char
        la      r1,0xFF0100     ; UART base
.wait:  lb      r2,1(r1)        ; read status byte
        lcu     r0,128
        and     r2,r0           ; isolate bit 7
        ceq     r2,z
        brf     .wait           ; spin while TX busy
        pop     r0              ; restore char
        sb      r0,0(r1)        ; transmit byte
        pop     r2              ; return address
        jmp     (r2)
"#
            .to_string(),
        ),
    ]
}
