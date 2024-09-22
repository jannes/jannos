#![no_std]
#![no_main]

use core::{arch::global_asm, hint::black_box};

use arch::hart_id;
use sbi::sbi_hart_start;

mod arch;
mod print;
mod sbi;

global_asm!(include_str!("boot.s"));

extern "C" {
    fn _park_me();
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    let hart_id = hart_id();
    println!("I am boot core ({}), starting other harts", hart_id);
    let start_addr = _park_me as u64;
    for i in 0..4 {
        if i != hart_id {
            // busy loop to let each hart debug print its id
            for _j in 0..10000000 {
                black_box(());
            }
            sbi_hart_start(i, start_addr, 0);
        }
    }
    loop {}
}
