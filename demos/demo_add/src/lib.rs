//! Demo: Add Two Numbers
//! Computes 100 + 200 + 42 = 342 and stores result at address 0x0100.
//! Pipeline: this file → rustc (msp430) → .msp430.s → msp430-to-cor24 → .cor24.s

#![no_std]

/// Result stored here — visible in memory viewer at halt
const RESULT_ADDR: u16 = 0x0100;

#[inline(never)]
#[no_mangle]
pub fn demo_add() -> u16 {
    let a: u16 = 100;
    let b: u16 = 200;
    let c: u16 = 42;
    a + b + c
}

#[no_mangle]
pub unsafe fn start() -> ! {
    let result = demo_add();
    core::ptr::write_volatile(RESULT_ADDR as *mut u16, result);
    loop {}
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
