use bitflags::bitflags;

bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]

    pub struct Flags: u16 {
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

use safe_mmio::fields::{ReadPure, ReadPureWrite, ReadWrite, WriteOnly};

#[repr(C, align(4))]
pub struct Registers {
    pub dr: u16,
    _reserved0: [u8; 2],
    pub rsr: u16,
    _reserved1: [u8; 19],
    pub fr: Flags,
    _reserved2: [u8; 6],
    pub ilpr: u8,
    _reserved3: [u8; 3],
    pub ibrd: u16,
    _reserved4: [u8; 2],
    pub fbrd: u8,
    _reserved5: [u8; 3],
    pub lcr_h: u8,
    _reserved6: [u8; 3],
    pub cr: u16,
    _reserved7: [u8; 3],
    pub ifls: u8,
    _reserved8: [u8; 3],
    pub imsc: u16,
    _reserved9: [u8; 2],
    pub ris: u16,
    _reserved10: [u8; 2],
    pub mis: u16,
    _reserved11: [u8; 2],
    pub icr: u16,
    _reserved12: [u8; 2],
    pub dmacr: u8,
    _reserved13: [u8; 3],
}