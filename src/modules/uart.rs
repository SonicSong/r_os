#![no_std]

use core::ptr::write_volatile;
use core::ptr::read_volatile;

const PERIPHERAL_BASE: usize = 0x3F000000;
const UART0_OFFSET: usize = 0x0020_1000;
const UART0_BASE: usize = PERIPHERAL_BASE + UART0_OFFSET;

const UART0_DR : *mut u32 = (UART0_BASE + 0x00) as *mut _;
const UART0_FR : *const u32 = (UART0_BASE + 0x18) as *const _;
const UART0_IBRD : *mut u32 = (UART0_BASE + 0x24) as *mut _;
const UART0_FBRD : *mut u32 = (UART0_BASE + 0x28) as *mut _;
const UART0_LCRH : *mut u32 = (UART0_BASE + 0x2C) as *mut _;
const UART0_CR : *mut u32 = (UART0_BASE + 0x30) as *mut _;
const UART0_IMSC : *mut u32 = (UART0_BASE + 0x38) as *mut _;
const UART0_ICR : *mut u32 = (UART0_BASE + 0x44) as *mut _;

pub unsafe fn uart_init() {
    // Disable UART while configuring
    write_volatile(UART0_CR, 0);

    // Clear interrupts before enabling
    write_volatile(UART0_ICR, 0x7FF);

    // Set baud rate (depends on UART clock rate)
    write_volatile(UART0_IBRD, 1);
    write_volatile(UART0_FBRD, 40);

    // 8-bit, no parity, FIFO enabled
    write_volatile(UART0_LCRH, (1 << 5) | (1 << 4) | (1 << 6));

    // Enable UART, TX, RX
    write_volatile(UART0_CR, (1 << 0) | (1 << 8) | (1 << 9));
}

pub unsafe fn init() {
    // Disable UART
    write_volatile(UART0_CR, 0);

    // Set baud rate (115200 @ 48MHz)
    write_volatile(UART0_IBRD, 26);
    write_volatile(UART0_FBRD, 3);

    // 8 bits, no parity, 1 stop bit, FIFO enabled
    write_volatile(UART0_LCRH, 0x70);

    // Enable UART, TX, RX
    write_volatile(UART0_CR, 0x301);
}

// Write a character to UART
pub unsafe fn putc(c: u8) {
    // Wait for UART to be ready
    while (read_volatile(UART0_FR) & 0x20) != 0 {}

    // Write character
    write_volatile(UART0_DR, c as u32);
}

// Write a string to UART
pub fn puts(s: &str) {
    for c in s.bytes() {
        unsafe { putc(c); }
    }
}