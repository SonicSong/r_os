#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

global_asm!(include_str!("boot.s"));

mod modules;
use modules::uart::uart_pl011;

#[no_mangle]
pub extern "C" fn rust_main() {

    unsafe {
        uart_pl011::init();
        uart_pl011::puts("HelloWorld");

    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}