const MAILBOX_BASE_OFFSET: usize = 0xB880;

const MAILBOX_READ: *mut u32 = (MAILBOX_BASE_OFFSET + 0x00) as *mut u32;
const MAILBOX_STATUS : *mut u32 = (MAILBOX_BASE_OFFSET + 0x18) as *mut u32;
const MAILBOX_WRITE:  *mut u32 = (MAILBOX_BASE_OFFSET + 0x20) as *mut u32;

pub unsafe fn mailbox_read() {

}