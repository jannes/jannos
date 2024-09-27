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
    println!("I am boot core ({})", hart_id);

    // addresses of the linker defined symbols are the actual values we need
    let heap_start = unsafe { ((&_HEAP_START) as *const usize) as usize };
    let mem_end = unsafe { ((&_MEM_END) as *const usize) as usize };
    println!("init kmem");
    let mut kmem = KMEM.lock();
    kmem.init(heap_start, mem_end);

    start_other_cores(hart_id);
    panic!("kmain done");
}

// TODO:
// - actually set up a stack for other cores
// - jump directly into rust after stack is set up
fn start_other_cores(boot_core_id: u64) {
    println!("starting non-boot cores");
    let start_addr = _park_me as usize;
    for i in 0..4 {
        if i != boot_core_id {
            sbi_hart_start(i, start_addr as u64, 0);
            // busy loop to let each hart debug print its id
            #[cfg(debug_assertions)]
            let iterations = 10000;
            #[cfg(not(debug_assertions))]
            let iterations = 1000000;
            for _j in 0..iterations {
                black_box(());
            }
        }
    }
}
