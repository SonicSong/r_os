#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

global_asm!(include_str!("boot.s"));

mod modules;
use modules::uart::uart;
use modules::uart::uart_config;

const PL011_BASE_ADDRESS: *mut uart_config::Registers = 0x3F000000 as _;

#[no_mangle]
pub extern "C" fn rust_main() {

    let mut uart = unsafe { uart::Uart::new(PL011_BASE_ADDRESS) };

    // unsafe {
    //     uart::init();
    //     uart::puts("rOS over UART...\n");
    //
    // }

    loop {
        if let Some(byte) = uart.read_byte() {
            uart.write_byte(byte);
            match byte {
                b'\r' => uart.write_byte(b'\n'),
                b'q' => break,
                _ => continue,
            }
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}