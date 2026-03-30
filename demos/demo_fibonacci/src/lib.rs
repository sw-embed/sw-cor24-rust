//! Demo: Fibonacci (recursive)
//! Computes fib(10) = 89 using recursion, stores result to memory at 0x0100.
//! Recursive style uses stack frames naturally — no register spill issues.
//! Pipeline: this file → rustc (msp430) → .msp430.s → msp430-to-cor24 → .cor24.s → assembler → emulator

#![no_std]

const RESULT_ADDR: u16 = 0x0100;

#[inline(never)]
#[no_mangle]
pub unsafe fn mem_write(addr: u16, val: u8) {
    core::ptr::write_volatile(addr as *mut u8, val);
}

/// Recursive fibonacci — matches the reference COR24 C implementation (fib.c).
/// Uses stack frames for each recursive call, so only r0-r2 are needed at any point.
#[inline(never)]
#[no_mangle]
pub fn fibonacci(n: u16) -> u16 {
    if n < 2 {
        return 1;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

#[inline(never)]
#[no_mangle]
pub unsafe fn demo_fibonacci() {
    let result = fibonacci(10);  // Should be 89
    mem_write(RESULT_ADDR, result as u8);
    loop {}
}

/// Entry point
#[inline(never)]
#[no_mangle]
pub unsafe fn start() -> ! {
    demo_fibonacci();
    loop {}
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! { loop {} }
