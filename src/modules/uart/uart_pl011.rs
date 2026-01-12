use core::arch::asm;
use core::ptr::write_volatile;
use core::ptr::read_volatile;

//TODO: Implement working FIFO for UART PL011

// By some weird miracle or not... I am somehow interacting with GPU peripherals based on BCM2836 peripherals documentation...
// No idea how it works in there... And I've been reading BCM2837 peripherals!!! But in theory they both are compatible...
// BUT THE DOCS AREN'T. FOR THE LOVE OF GOD. 0x3E00_0000 .. 0x3FFF_FFFF THOSE ADDRESSES ARE FOR GPU PERIPHERAL ACCESS

const PERIPHERAL_BASE: usize = 0x3F000000;
const GPPUD_OFFSET: usize = 0x200000;
const UART0_OFFSET: usize = 0x0020_1000;
const UART0_BASE: usize = PERIPHERAL_BASE + UART0_OFFSET;
const GPPUD_BASE: usize = PERIPHERAL_BASE + GPPUD_OFFSET;

// Controls actuation of pull up/down to ALL GPIO pins.
// GPPUD = (GPIO_BASE + 0x94),
const GPPUD : *mut u32 = (GPPUD_BASE + 0x94) as *mut _;

// Controls actuation of pull up/down for specific GPIO pin.
// GPPUDCLK0 = (GPIO_BASE + 0x98),
const GPPUDCLK0: *mut u32 = (GPPUD_BASE + 0x98) as *mut _;

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

// The offsets for Mailbox registers
// MBOX_BASE    = 0xB880,
// MBOX_READ    = (MBOX_BASE + 0x00),
// MBOX_STATUS  = (MBOX_BASE + 0x18),
// MBOX_WRITE   = (MBOX_BASE + 0x20)

const MBOX_BASE: usize = PERIPHERAL_BASE + 0xB880;
const MBOX_READ : *mut u32 = (MBOX_BASE + 0x00) as *mut _;
const MBOX_STATUS : *mut u32 = (MBOX_BASE + 0x18) as *mut _;
const MBOX_WRITE : *mut u32 = (MBOX_BASE + 0x20) as *mut _;

pub unsafe fn delay(mut count: u32) {
    asm!(
    "1:",
    "subs {cnt}, {cnt}, #1",
    "bne 1b",
    cnt = inout(reg) count => count,
    options(nostack, nomem, preserves_flags),
    );
}

// A Mailbox message with set clock rate of PL011 to 3MHz tag
#[repr(align(16))]
pub struct AlignedBuf([u32; 9]);
pub static mut MBOX: AlignedBuf = AlignedBuf([9*4, 0, 0x38002, 12, 8, 2, 3_000_000, 0, 0]);

pub unsafe fn init() {
    // Disable UART
    write_volatile(UART0_CR, 0x00000000);
    delay(150);
    write_volatile(GPPUD, 0x00000000);

    write_volatile(GPPUDCLK0, (1 << 14) | (1 << 15));
    delay(150);

    write_volatile(GPPUDCLK0, 0x00000000);

    write_volatile(UART0_ICR, 0x7FF);

    // if (raspi >= 3) {
    //     // UART_CLOCK = 30000000;
    //     unsigned int r = (((unsigned int)(&mbox) & ~0xF) | 8);
    //     // wait until we can talk to the VC
    //     while ( mmio_read(MBOX_STATUS) & 0x80000000 ) { }
    //     // send our message to property channel and wait for the response
    //     mmio_write(MBOX_WRITE, r);
    //     while ( (mmio_read(MBOX_STATUS) & 0x40000000) || mmio_read(MBOX_READ) != r ) { }
    // }

    write_volatile(UART0_IBRD, 1);
    write_volatile(UART0_FBRD, 40);

    // 8 bits, no parity, 1 stop bit, FIFO enabled
    write_volatile(UART0_LCRH, (1 << 4) | (1 << 5) | (1 << 6));
    write_volatile(UART0_IMSC, (1 << 1) | (1 << 4) | (1 << 5) | (1 << 6) | (1 << 7) | (1 << 8) | (1 << 9) | (1 << 10));

    // Enable UART, TX, RX
    write_volatile(UART0_CR, (1 << 0) | (1 << 8) | (1 << 9));
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
pub unsafe fn getc() -> u8{
    // getc should also return u32 to be able to add that character or symbol to the str buffer so it can be "interpreted" by proto_shell to call for example for echo or help.
    while (UART0_FR.read_volatile() & 1<<4) != 0 {}
    let enter: u8 = read_volatile(UART0_DR) as u8;
    if (enter == 0x0A || enter == 0x0D) {
        puts("\r\n");
        enter
    } else {
        putc(enter);
        enter
    }
}