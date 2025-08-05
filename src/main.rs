#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

global_asm!(include_str!("boot.s"));

mod modules;
use modules::uart;

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    unsafe {
        uart::init();
        uart::puts("ROS booting on Raspberry Pi Zero 2 W...\n");
    }

    loop {

    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}