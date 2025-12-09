const PERIPHERAL_BASE: usize = 0x3F000000;
const MAILBOX_OFFSET: usize = 0x0000B880;
const MAILBOX_BASE: usize = PERIPHERAL_BASE + MAILBOX_OFFSET;

const MAILBOX_READ: *mut u32 = (MAILBOX_BASE + 0x00) as *mut u32;
const MAILBOX_STATUS : *mut u32 = (MAILBOX_BASE + 0x18) as *mut u32;
const MAILBOX_WRITE:  *mut u32 = (MAILBOX_BASE + 0x20) as *mut u32;

pub unsafe fn mailbox_read() {

}