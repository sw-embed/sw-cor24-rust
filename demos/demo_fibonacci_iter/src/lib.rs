//! Demo: Fibonacci (iterative)
//! Computes fib(10) = 89 using a simple loop with only 3 variables.
//! Uses @cor24 passthrough for the inner loop — 3 registers, no spills.
//! Pipeline: this file → rustc (msp430) → .msp430.s → msp430-to-cor24 → .cor24.s → assembler → emulator

#![no_std]
#![feature(asm_experimental_arch)]

const RESULT_ADDR: u16 = 0x0100;

#[inline(never)]
#[no_mangle]
pub unsafe fn mem_write(addr: u16, val: u8) {
    core::ptr::write_volatile(addr as *mut u8, val);
}

/// Iterative fibonacci using COR24-native inner loop.
/// r0 = n (input/counter), r1 = a, r2 = b. Returns result in r0.
/// fib(0)=1, fib(1)=1, fib(10)=89.
#[inline(never)]
#[no_mangle]
pub unsafe fn fibonacci_iter(n: u16) -> u16 {
    // r0 = n on entry (r12 mapped to r0)
    let result: u16;
    core::arch::asm!(
        // COR24: r0=n, compute fib(n) iteratively
        // Save return address, use r0-r2 for computation
        "; @cor24: push    r1",            // save return addr
        "; @cor24: lc      r1, 1",         // a = 1
        "; @cor24: lc      r2, 1",         // b = 1
        "; @cor24: ceq     r0, z",
        "; @cor24: brt     .fib_done",     // fib(0) = 1
        "; @cor24: .fib_loop:",
        "; @cor24: push    r1",            // save old a
        "; @cor24: mov     r1, r2",        // a = b
        "; @cor24: pop     r2",            // r2 = old a
        "; @cor24: add     r2, r1",        // b = old_a + new_a
        "; @cor24: add     r0, -1",        // n--
        "; @cor24: ceq     r0, z",
        "; @cor24: brf     .fib_loop",
        "; @cor24: .fib_done:",
        "; @cor24: mov     r0, r1",        // return a in r0
        "; @cor24: pop     r1",            // restore return addr
        inout("r12") n => result,
    );
    result
}

#[inline(never)]
#[no_mangle]
pub unsafe fn demo_fibonacci_iter() {
    let result = fibonacci_iter(10);  // Should be 89
    mem_write(RESULT_ADDR, result as u8);
    loop {}
}

/// Entry point
#[inline(never)]
#[no_mangle]
pub unsafe fn start() -> ! {
    demo_fibonacci_iter();
    loop {}
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! { loop {} }
