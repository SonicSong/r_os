use core::ptr::write_volatile;
use core::ptr::read_volatile;
use core::fmt::{self, Write};
use bitflags::bitflags;

//TODO: Implement working FIFO for UART PL011

// By some weird miracle or not... I am somehow interacting with GPU peripherals based on BCM2836 peripherals documentation...
// No idea how it works in there... And I've been reading BCM2837 peripherals!!! But in theory they both are compatible...
// BUT THE DOCS AREN'T. FOR THE LOVE OF GOD. 0x3E00_0000 .. 0x3FFF_FFFF THOSE ADDRESSES ARE FOR GPU PERIPHERAL ACCESS

const PERIPHERAL_BASE: usize = 0x3F000000;
const UART0_OFFSET: usize = 0x0020_1000;
const UART0_BASE: usize = PERIPHERAL_BASE + UART0_OFFSET;

/// Data Register
const UART0_DR : *mut u32 = (UART0_BASE + 0x00) as *mut _;
/// Flag Register
const UART0_FR : *const u32 = (UART0_BASE + 0x18) as *const _;
/// Integer Baud rate divisor
const UART0_IBRD : *mut u32 = (UART0_BASE + 0x24) as *mut _;
///Fractional Baud rate divisor
const UART0_FBRD : *mut u32 = (UART0_BASE + 0x28) as *mut _;
/// Line control register
const UART0_LCRH : *mut u32 = (UART0_BASE + 0x2C) as *mut _;
/// Control register
const UART0_CR : *mut u32 = (UART0_BASE + 0x30) as *mut _;

//FIFO
/// Interupt FIFO Level Select Register
const UART0_IFLS : *mut u32 = (UART0_BASE + 0x34) as *mut _;
/// Interupt Mask Set Clear Register
const UART0_IMSC : *mut u32 = (UART0_BASE + 0x38) as *mut _;
/// Interupt Clear Register
const UART0_ICR : *mut u32 = (UART0_BASE + 0x44) as *mut _;

bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]

    pub struct Flags: u32 {
        /// Clear to send.
        const CTS = 1 << 0;
        /// Data set ready.
        const DSR = 1 << 1;
        /// Data carrier detect.
        const DCD = 1 << 2;
        /// UART busy transmitting data.
        const BUSY = 1 << 3;
        /// Receive FIFO is empty.
        const RXFE = 1 << 4;
        /// Transmit FIFO is full.
        const TXFF = 1 << 5;
        /// Receive FIFO is full.
        const RXFF = 1 << 6;
        /// Transmit FIFO is empty.
        const TXFE = 1 << 7;
        /// Ring indicator.
        const RI = 1 << 8;
    }
}

pub unsafe fn init() {
    // Disable UART
    write_volatile(UART0_CR, 0);

    /*
    Baud = 115200bps; UART_CLOCK = 48000000
    Divider = UART_CLOCK/(16 * Baud)
    Fraction part register = (Fractional part * 64) + 0.5

    Div = 48000000/(16 * 115200) = 26.04166666666667 ~= 26 = 0x1A
    Frac = (.0416 * 64) + 0.5 = 3.1624 ~= 3 = 3
    */
    write_volatile(UART0_IBRD, 0x1A);
    write_volatile(UART0_FBRD, 3);

    // 8 bits, no parity, 1 stop bit, FIFO enabled
    write_volatile(UART0_LCRH, 0x70);

    // Enable UART, TX, RX
    write_volatile(UART0_CR, 0x301);
}

// Write a character to UART
pub fn putc(c: u8) {
    unsafe {
        while (UART0_FR.read_volatile() & 1<<5) != 0 {}
        write_volatile(UART0_DR, c as u32);
    }
}

// Write a string to UART
pub fn puts(s: &str) {
    for c in s.bytes() {
        putc(c);
    }
}

// Read from UART
pub unsafe fn getc() {
    while (UART0_FR.read_volatile() & 1<<4) != 0 {}
    let enter: u32 = read_volatile(UART0_DR);
    if (enter == 0x0A || enter == 0x0D) {
        puts("\r\n");
    } else {
        putc(enter as u8);
    }
}

pub unsafe fn await_enter() {
    while (UART0_FR.read_volatile() & 1<<4) != 0 {}

}