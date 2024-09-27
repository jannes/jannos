#![no_std]
#![no_main]

use core::{
    arch::{asm, global_asm},
    hint::black_box,
};

use arch::hart_id;
use fdt::Fdt;
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
pub extern "C" fn kmain(fdt_addr: usize) -> ! {
    let hart_id = hart_id();
    println!("I am boot core ({}), fdt_addr: {:#x}", hart_id, fdt_addr);

    // determine amount of cores and memory space through devicetree
    let fdt = match unsafe { Fdt::from_ptr(fdt_addr as *const u8) } {
        Ok(fdt) => fdt,
        Err(err) => panic!("failed to parse fdt, err: {}", err),
    };

    let num_cpu = fdt.cpus().count();
    println!("Total cores: {}", num_cpu);

    assert_eq!(1, fdt.memory().regions().count());
    let mem_region = fdt.memory().regions().next().unwrap();
    let mem_start = mem_region.starting_address as usize;
    let mem_end = mem_start + mem_region.size.unwrap();
    println!("memory start: {:#x}, end: {:#x}", mem_start, mem_end);

    // addresses of the linker defined symbols are the actual values we need
    let heap_start = unsafe { ((&_HEAP_START) as *const usize) as usize };
    println!("init kmem");
    let mut kmem = KMEM.lock();
    kmem.init(heap_start, mem_end);

    start_other_cores(hart_id, num_cpu);
    panic!("kmain done");
}

// TODO:
// - actually set up a stack for other cores
// - jump directly into rust after stack is set up
fn start_other_cores(boot_core_id: usize, num_cpu: usize) {
    println!("starting non-boot cores");
    let start_addr = _park_me as usize;
    for i in 0..num_cpu {
        if i != boot_core_id {
            sbi_hart_start(i, start_addr, 0);
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
