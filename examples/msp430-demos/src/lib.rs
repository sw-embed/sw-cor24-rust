//! COR24 demo programs compiled via MSP430 target
//!
//! These functions are compiled to MSP430 assembly, then translated
//! to COR24 assembly by the msp430-to-cor24 translator.
//!
//! Hardware addresses (COR24-TB):
//!   0xFF0000 (IO_LEDSWDAT) - LED D2 (write bit 0) / Button S2 (read bit 0)
//!   0xFF0100 (IO_UARTDATA) - UART data register
//!   0xFF0101 (IO_UARTSTAT) - UART status (bit 1 = TX ready)

#![no_std]

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// --- Hardware abstraction ---

/// Write a byte to a memory-mapped I/O address
#[inline(never)]
#[no_mangle]
pub unsafe fn mmio_write(addr: u16, val: u16) {
    core::ptr::write_volatile(addr as *mut u16, val);
}

/// Read a byte from a memory-mapped I/O address
#[inline(never)]
#[no_mangle]
pub unsafe fn mmio_read(addr: u16) -> u16 {
    core::ptr::read_volatile(addr as *const u16)
}

/// Busy-wait delay loop (volatile to prevent optimization)
#[inline(never)]
#[no_mangle]
pub fn delay(mut n: u16) {
    while n != 0 {
        unsafe { core::ptr::write_volatile(&mut n as *mut u16, n - 1); }
    }
}

// --- I/O addresses (16-bit subset for MSP430 compilation) ---
// Real COR24 addresses are 24-bit (0xFF0000, 0xFF0100, 0xFF0101)
// The translator maps 16-bit MSP430 addresses; we use the low 16 bits
// and rely on the translator/linker to fix up to 24-bit.
// For now, we use addresses that fit in 16 bits for the MSP430 target,
// and the translator handles the mapping.

const LED_ADDR: u16 = 0xFF00;    // maps to 0xFF0000 in COR24
const UART_DATA: u16 = 0xFF01;   // maps to 0xFF0100 in COR24
const UART_STAT: u16 = 0xFF02;   // maps to 0xFF0101 in COR24

// ============================================================
// Demo 1: Blinky - Toggle LED with delay
// ============================================================

#[no_mangle]
pub unsafe fn demo_blinky() -> ! {
    loop {
        mmio_write(LED_ADDR, 1);   // LED on
        delay(5000);
        mmio_write(LED_ADDR, 0);   // LED off
        delay(5000);
    }
}

// ============================================================
// Demo 2: UART Hello World - Write "Hello" to UART
// ============================================================

/// Send a single byte via UART
#[inline(never)]
#[no_mangle]
pub unsafe fn uart_putc(ch: u16) {
    mmio_write(UART_DATA, ch);
}

#[no_mangle]
pub unsafe fn demo_uart_hello() {
    // Write "Hello\n" character by character
    uart_putc(b'H' as u16);
    uart_putc(b'e' as u16);
    uart_putc(b'l' as u16);
    uart_putc(b'l' as u16);
    uart_putc(b'o' as u16);
    uart_putc(b'\n' as u16);
    // Halt (self-branch detected by emulator)
    loop {}
}

// ============================================================
// Demo 3: Compute - Add numbers and store result
// ============================================================

#[no_mangle]
pub fn demo_add() -> u16 {
    let a: u16 = 100;
    let b: u16 = 200;
    let c: u16 = 42;
    a + b + c
}

// ============================================================
// Demo 4: Countdown - Count down, store in memory
// ============================================================

#[no_mangle]
pub unsafe fn demo_countdown() {
    let mut count: u16 = 10;
    while count != 0 {
        // Write count to LED register so we can see it
        mmio_write(LED_ADDR, count);
        delay(1000);
        count -= 1;
    }
    mmio_write(LED_ADDR, 0);  // Clear LED
    loop {}  // Halt
}

// ============================================================
// Demo 5: Button Echo - Read button, echo to LED
// ============================================================

#[no_mangle]
pub unsafe fn demo_button_echo() -> ! {
    loop {
        let btn = mmio_read(LED_ADDR);  // Read button state
        let led = btn & 1;              // Mask to bit 0
        mmio_write(LED_ADDR, led);      // Echo to LED
    }
}

// ============================================================
// Demo 6: Fibonacci - Compute fib(10) = 55
// ============================================================

#[no_mangle]
pub fn fibonacci(n: u16) -> u16 {
    if n <= 1 {
        return n;
    }
    let mut a: u16 = 0;
    let mut b: u16 = 1;
    let mut i: u16 = 2;
    while i <= n {
        let tmp = a + b;
        a = b;
        b = tmp;
        i += 1;
    }
    b
}

#[no_mangle]
pub unsafe fn demo_fibonacci() {
    let result = fibonacci(10);  // Should be 55
    // Store result in LED register to verify
    mmio_write(LED_ADDR, result);
    loop {}  // Halt
}

// ============================================================
// Demo 7: Nested Calls - show stack frames from A→B→C
// ============================================================
// Entry calls level_a(5), which calls level_b(15), which calls
// level_c(115) which writes to LED and halts.
// At halt, all three stack frames are live and visible in memory.

#[inline(never)]
#[no_mangle]
pub unsafe fn level_c(x: u16, y: u16) -> u16 {
    // Write both values to LED and UART so they're visible
    mmio_write(LED_ADDR, x);
    uart_putc(y);
    loop {}  // halt — three stack frames are now live
}

#[inline(never)]
#[no_mangle]
pub unsafe fn level_b(x: u16) -> u16 {
    let doubled = x + x;      // r12 math, might stay in registers
    let offset = doubled + 3;  // more computation
    level_c(offset, x)         // pass two args so both show up
}

#[inline(never)]
#[no_mangle]
pub unsafe fn level_a(x: u16) -> u16 {
    let y = x + 10;    // add constant
    level_b(y)          // call next level
}

#[no_mangle]
pub unsafe fn demo_nested() {
    let btn = mmio_read(LED_ADDR);  // runtime value → prevents constant folding
    level_a(btn + 5);
}

// ============================================================
// Demo 8: Stack-heavy - many locals that spill to stack
// ============================================================
// Accumulates values across many variables, forcing the compiler
// to use callee-saved registers (r4-r11, r15 on MSP430) which
// get spilled to fp-relative slots in COR24 translation.
// At halt, the spill slots contain the intermediate values.

#[inline(never)]
#[no_mangle]
pub unsafe fn accumulate(seed: u16) -> u16 {
    // Use enough variables to exhaust caller-saved registers (r12-r15)
    // and force use of callee-saved registers (r4-r11)
    let a = seed + 1;
    let b = a + seed;
    let c = b + a;
    let d = c + b;
    let e = d + c;
    // Keep all variables live — compiler can't discard any
    let result = a ^ b ^ c ^ d ^ e;
    mmio_write(LED_ADDR, result);
    // Also store intermediate values to UART so compiler keeps them
    uart_putc(a);
    uart_putc(b);
    uart_putc(c);
    uart_putc(d);
    uart_putc(e);
    loop {}  // halt with spill slots visible
}

#[no_mangle]
pub unsafe fn demo_stack_vars() {
    let x = mmio_read(LED_ADDR);  // runtime value
    accumulate(x + 1);
}
