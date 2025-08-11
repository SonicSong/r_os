use core::ptr::write_volatile;
use core::ptr::read_volatile;
use core::fmt::{self, Write};

use crate::modules::uart::uart_config::Registers;
use crate::modules::uart::uart_config::Flags;

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
    write_volatile(UART0_DR, c as u32);
}

// Write a string to UART
pub fn puts(s: &str) {
    for c in s.bytes() {
        unsafe { putc(c); }
    }
}

use safe_mmio::{UniqueMmioPointer, field, field_shared};

/// Driver for a PL011 UART.
#[derive(Debug)]
pub struct Uart {
    registers: *mut Registers,
}

impl Uart {
    /// Constructs a new instance of the UART driver for a PL011 device with the
    /// given set of registers.
    ///
    /// # Safety
    ///
    /// The given pointer must point to the 8 MMIO control registers of a PL011
    /// device, which must be mapped into the address space of the process as
    /// device memory and not have any other aliases.
    pub unsafe fn new(registers: *mut Registers) -> Self {
        Self { registers }
    }

    /// Writes a single byte to the UART.
    pub fn write_byte(&mut self, byte: u8) {
        // Wait until there is room in the TX buffer.
        while self.read_flag_register().contains(Flags::TXFF) {}

        // SAFETY: We know that self.registers points to the control registers
        // of a PL011 device which is appropriately mapped.
        unsafe {
            // Write to the TX buffer.
            (&raw mut (*self.registers).dr).write_volatile(byte.into());
        }

        // Wait until the UART is no longer busy.
        while self.read_flag_register().contains(Flags::BUSY) {}
    }

    /// Reads and returns a pending byte, or `None` if nothing has been
    /// received.
    pub fn read_byte(&mut self) -> Option<u8> {
        if self.read_flag_register().contains(Flags::RXFE) {
            None
        } else {
            // SAFETY: We know that self.registers points to the control
            // registers of a PL011 device which is appropriately mapped.
            let data = unsafe { (&raw const (*self.registers).dr).read_volatile() };
            // TODO: Check for error conditions in bits 8-11.
            Some(data as u8)
        }
    }

    fn read_flag_register(&self) -> Flags {
        // SAFETY: We know that self.registers points to the control registers
        // of a PL011 device which is appropriately mapped.
        unsafe { (&raw const (*self.registers).fr).read_volatile() }
    }
}

impl Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.as_bytes() {
            self.write_byte(*c);
        }
        Ok(())
    }
}