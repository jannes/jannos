#![no_std]
#![no_main]

use core::{
    arch::{asm, global_asm},
    hint::black_box,
};

use arch::hart_id;
use mem::KMEM;
use sbi::sbi_hart_start;

mod arch;
mod lock;
mod mem;
mod print;
mod sbi;

global_asm!(include_str!("boot.s"));

extern "C" {
    fn _park_me();
    static _HEAP_START: usize;
    static _MEM_END: usize;
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("panic: {}", info);
    loop {
        unsafe {
            asm!("wfi");
        }
    }
}

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    let hart_id = hart_id();
    println!("I am boot core ({}), starting other harts", hart_id);
    let start_addr = _park_me as usize;
    // for i in 0..4 {
    //     if i != hart_id {
    //         sbi_hart_start(i, start_addr as u64, 0);
    //         // busy loop to let each hart debug print its id
    //         for _j in 0..1000 {
    //             black_box(());
    //         }
    //     }
    // }
    // addresses of the linker defined symbols are the actual values we need
    let heap_start = unsafe { ((&_HEAP_START) as *const usize) as usize };
    let mem_end = unsafe { ((&_MEM_END) as *const usize) as usize };
    println!("init kmem");
    KMEM.lock().init(heap_start, mem_end);
    panic!();
}
