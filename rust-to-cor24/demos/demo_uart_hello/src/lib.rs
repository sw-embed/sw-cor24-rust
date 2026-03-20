//! Demo: UART Hello World
//! Sends "Hello\n" character by character to UART data register.
//! Pipeline: this file → rustc (msp430) → .msp430.s → msp430-to-cor24 → .cor24.s → assembler → emulator

#![no_std]

const UART_DATA: u16 = 0xFF01;
const UART_STAT: u16 = 0xFF02;

#[inline(never)]
#[no_mangle]
pub unsafe fn mmio_write(addr: u16, val: u16) {
    core::ptr::write_volatile(addr as *mut u8, val as u8);
}

#[inline(never)]
#[no_mangle]
pub unsafe fn mmio_read(addr: u16) -> u8 {
    core::ptr::read_volatile(addr as *const u8)
}

#[inline(never)]
#[no_mangle]
pub unsafe fn uart_putc(ch: u16) {
    // Poll TX busy (bit 7 of status register) before writing
    while (mmio_read(UART_STAT) & 0x80) != 0 {}
    mmio_write(UART_DATA, ch);
}

#[no_mangle]
pub unsafe fn demo_uart_hello() {
    uart_putc(b'H' as u16);
    uart_putc(b'e' as u16);
    uart_putc(b'l' as u16);
    uart_putc(b'l' as u16);
    uart_putc(b'o' as u16);
    uart_putc(b'\n' as u16);
    loop {}
}

/// Entry point
#[inline(never)]
#[no_mangle]
pub unsafe fn start() -> ! {
    demo_uart_hello();
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
