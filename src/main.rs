#![no_std]
#![no_main]

mod modules;

use core::panic::PanicInfo;

use modules::uart;

mod boot {
    use core::arch::global_asm;

    global_asm!(
        ".section .test._start"
    );
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe{
        uart::uart_init();
        uart::uart_putc(1);
    }
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}