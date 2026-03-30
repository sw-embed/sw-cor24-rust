//! Demo: Stack Variables (Register Spilling)
//! Accumulates values across 5+ variables, forcing register spills to stack.
//! MSP430 uses 16 registers; COR24 has only 3 GP registers (r0-r2).
//! The translator maps r12→r0 and spills r13-r15, r4-r11 to fp-relative
//! stack slots, generating sw/lw pairs for each access. This is correct
//! but slower than hand-written code — the cost of automatic translation.
//! Compare with the fibonacci_iter demo which uses @cor24 passthrough
//! to avoid spills entirely.
//! Pipeline: this file → rustc (msp430) → .msp430.s → msp430-to-cor24 → .cor24.s → assembler → emulator

#![no_std]

const RESULT_ADDR: u16 = 0x0100;
const SWITCH_ADDR: u16 = 0xFF00;
const UART_DATA: u16 = 0xFF01;

#[inline(never)]
#[no_mangle]
pub unsafe fn mem_write(addr: u16, val: u8) {
    core::ptr::write_volatile(addr as *mut u8, val);
}

#[inline(never)]
#[no_mangle]
pub unsafe fn mem_read(addr: u16) -> u8 {
    core::ptr::read_volatile(addr as *const u8)
}

#[inline(never)]
#[no_mangle]
pub unsafe fn uart_putc(ch: u16) {
    mem_write(UART_DATA, ch as u8);
}

#[inline(never)]
#[no_mangle]
pub unsafe fn accumulate(seed: u16) -> u16 {
    let a = seed + 1;
    let b = a + seed;
    let c = b + a;
    let d = c + b;
    let e = d + c;
    let result = a ^ b ^ c ^ d ^ e;
    mem_write(RESULT_ADDR, result as u8);
    uart_putc(a);
    uart_putc(b);
    uart_putc(c);
    uart_putc(d);
    uart_putc(e);
    loop {}  // halt with spill slots visible
}

#[inline(never)]
#[no_mangle]
pub unsafe fn demo_stack_vars() {
    let x = mem_read(SWITCH_ADDR) as u16;  // runtime value
    accumulate(x + 1);
}

/// Entry point
#[inline(never)]
#[no_mangle]
pub unsafe fn start() -> ! {
    demo_stack_vars();
    loop {}
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    unsafe {
        uart_putc(b'P' as u16);
        uart_putc(b'A' as u16);
        uart_putc(b'N' as u16);
        uart_putc(b'I' as u16);
        uart_putc(b'C' as u16);
        uart_putc(b'\n' as u16);
    }
    loop {}
}
