//! Demo: Countdown
//! Counts down from 10 to 0, storing each value to memory at 0x0100.
//! Pipeline: this file → rustc (msp430) → .msp430.s → msp430-to-cor24 → .cor24.s → assembler → emulator

#![no_std]

const RESULT_ADDR: u16 = 0x0100;

#[inline(never)]
#[no_mangle]
pub unsafe fn mem_write(addr: u16, val: u8) {
    core::ptr::write_volatile(addr as *mut u8, val);
}

#[inline(never)]
#[no_mangle]
pub fn delay(mut n: u16) {
    while n != 0 {
        unsafe { core::ptr::write_volatile(&mut n as *mut u16, n - 1); }
    }
}

#[no_mangle]
pub unsafe fn demo_countdown() {
    let mut count: u16 = 10;
    while count != 0 {
        mem_write(RESULT_ADDR, count as u8);
        delay(10);
        count -= 1;
    }
    mem_write(RESULT_ADDR, 0);
    loop {}
}

/// Entry point
#[inline(never)]
#[no_mangle]
pub unsafe fn start() -> ! {
    demo_countdown();
    loop {}
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! { loop {} }
