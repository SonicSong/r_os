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
        uart_pl011::puts("Hello World.\n");
        uart_pl011::puts("Running off a custom OS!\n");

        loop {
            // uart_pl011::puts(">> ");
            let ch_tes = uart_pl011::getc();
            ch_tes
            // uart_pl011::puts(" ");
            // uart_pl011::putc(ch_tes as u8);
            // uart_pl011::puts("");
            // uart_pl011::await_enter();
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}