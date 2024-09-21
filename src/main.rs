#![no_std]
#![no_main]

use core::{arch::global_asm, ptr};

global_asm!(include_str!("boot.s"));

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unimplemented!()
}

fn uart_print(message: &str) {
    const UART: *mut u8 = 0x10000000 as *mut u8;

    for c in message.chars() {
        unsafe {
            ptr::write_volatile(UART, c as u8);
        }
    }
}

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    uart_print("hello from rust\n");
    loop {}
}
