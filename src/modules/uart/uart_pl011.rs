use core::ptr::write_volatile;
use core::ptr::read_volatile;
use core::fmt::{self, Write};

//TODO: Implement working FIFO for UART PL011

// By some weird miracle or not... I am somehow interacting with GPU peripherals based on BCM2836 peripherals documentation...
// No idea how it works in there... And I've been reading BCM2837 peripherals!!! But in theory they both are compatible...
// BUT THE DOCS AREN'T. FOR THE LOVE OF GOD. 0x3E00_0000 .. 0x3FFF_FFFF THOSE ADDRESSES ARE FOR GPU PERIPHERAL ACCESS

const PERIPHERAL_BASE: usize = 0x3F000000;
const UART0_OFFSET: usize = 0x0020_1000;
const UART0_BASE: usize = PERIPHERAL_BASE + UART0_OFFSET;

const UART0_DR : *mut u32 = (UART0_BASE + 0x00) as *mut _;
const UART0_FR : *const u32 = (UART0_BASE + 0x18) as *const _;
const UART0_IBRD : *mut u32 = (UART0_BASE + 0x24) as *mut _;
const UART0_FBRD : *mut u32 = (UART0_BASE + 0x28) as *mut _;
const UART0_LCRH : *mut u32 = (UART0_BASE + 0x2C) as *mut _;
const UART0_CR : *mut u32 = (UART0_BASE + 0x30) as *mut _;

//FIFO
const UART0_IFLS : *mut u32 = (UART0_BASE + 0x34) as *mut _;

const UART0_IMSC : *mut u32 = (UART0_BASE + 0x38) as *mut _;

const UART0_ICR : *mut u32 = (UART0_BASE + 0x44) as *mut _;

pub unsafe fn init() {
    // Disable UART
    write_volatile(UART0_CR, 0);

    /*
    Baud = 921600bps; UART_CLOCK = 48000000
    Divider = UART_CLOCK/(16 * Baud)
    Fraction part register = (Fractional part * 64) + 0.5

    Div = 48000000/(16 * 921600) = 3,2552083333 ~= 3
    Frac = (.255 * 64) + 0.5 = 16,82 ~= 16 = 0x10
    */
    write_volatile(UART0_IBRD, 3);
    write_volatile(UART0_FBRD, 0x10);

    // 8 bits, no parity, 1 stop bit, FIFO enabled
    write_volatile(UART0_LCRH, 0x70);

    // Enable UART, TX, RX
    write_volatile(UART0_CR, 0x301);
}

// Write a character to UART
pub unsafe fn putc(c: u8) {
    write_volatile(UART0_DR, c as u32);
}

// Write a string to UART
pub fn puts(s: &str) {
    for c in s.bytes() {
        unsafe {
            putc(c);
        }
    }
}