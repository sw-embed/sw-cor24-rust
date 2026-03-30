//! Demo: Drop (RAII)
//! Demonstrates Rust's automatic destructor calls on stack variables.
//! No allocator needed — Drop works on stack-allocated values.
//!
//! A `Guard` writes 1 to its address on creation and 0 on drop.
//! The compiler inserts the drop call when the variable goes out of scope.
//!
//! Memory at 0x0100 goes: 0 → 1 (guard created) → 0 (guard dropped) → 0xFF (done)
//! Pipeline: this file → rustc (msp430) → .msp430.s → msp430-to-cor24 → .cor24.s

#![no_std]

const STATUS_ADDR: u16 = 0x0100;

/// Write a byte to a memory address (volatile to prevent optimization)
#[inline(never)]
#[no_mangle]
pub unsafe fn mem_write(addr: u16, val: u8) {
    core::ptr::write_volatile(addr as *mut u8, val);
}

/// A guard that writes 0 to its address when dropped.
/// This is Rust's RAII pattern — the compiler generates the cleanup code.
pub struct Guard {
    addr: u16,
}

impl Guard {
    #[inline(never)]
    #[no_mangle]
    pub fn guard_new(addr: u16) -> Guard {
        unsafe { mem_write(addr, 1); } // mark: guard is alive
        Guard { addr }
    }
}

impl Drop for Guard {
    #[inline(never)]
    fn drop(&mut self) {
        unsafe { mem_write(self.addr, 0); } // mark: guard is gone
    }
}

#[no_mangle]
pub unsafe fn start() -> ! {
    // Before: STATUS_ADDR = 0 (uninitialized memory)

    {
        let _g = Guard::guard_new(STATUS_ADDR);
        // During: STATUS_ADDR = 1 (guard is alive)
    }
    // After: STATUS_ADDR = 0 (compiler called drop here)

    // Write 0xFF to prove we continued past the drop
    mem_write(STATUS_ADDR, 0xFF);

    loop {}
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
