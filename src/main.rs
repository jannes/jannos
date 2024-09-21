#![no_std]
#![no_main]

use core::arch::global_asm;

use sbi::sbi_dbg_write;

mod sbi;

global_asm!(include_str!("boot.s"));

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    sbi_dbg_write("hello from rust (SBI)\n".as_bytes());
    loop {}
}
