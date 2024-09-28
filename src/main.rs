#![no_std]
#![no_main]

use core::{
    arch::{asm, global_asm},
    hint::black_box,
    ptr::slice_from_raw_parts,
    slice,
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

    println!("kmem addr: {:p}, locked: {}", &KMEM, KMEM.is_locked());
    // determine amount of cores and memory space through devicetree
    let (num_cpu, mem_end) = hw_info(fdt_addr);
    println!("kmem addr: {:p}, locked: {}", &KMEM, KMEM.is_locked());

    // addresses of the linker defined symbols are the actual values we need
    let heap_start = unsafe { ((&_HEAP_START) as *const usize) as usize };

    // let num_cpu = 1;
    // let mem_end = unsafe { ((&_MEM_END) as *const usize) as usize };

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

/// Obtain (num_cpus, memory_end) by parsing FDT at given address
/// IMPORTANT: the FDT parsing uses a lot of stack space,
///   which caused stack to overflow when its size was just a single page
fn hw_info(fdt_addr: usize) -> (usize, usize) {
    let fdt_header_magic_ptr = fdt_addr as *const u8;
    let fdt_header_magic = u32::from_be_bytes(
        unsafe { slice::from_raw_parts(fdt_header_magic_ptr, 4) }
            .try_into()
            .unwrap(),
    );
    let fdt_header_totalsize_ptr = unsafe { fdt_header_magic_ptr.offset(4) };
    let fdt_header_totalsize = u32::from_be_bytes(
        unsafe { slice::from_raw_parts(fdt_header_totalsize_ptr, 4) }
            .try_into()
            .unwrap(),
    );

    println!(
        "fdt header magic at {:p}: {:#x}",
        fdt_header_magic_ptr, fdt_header_magic
    );
    println!(
        "fdt header totalsize at {:p}: {}",
        fdt_header_totalsize_ptr, fdt_header_totalsize
    );
    let fdt_slice =
        unsafe { core::slice::from_raw_parts(fdt_header_magic_ptr, fdt_header_totalsize as usize) };

    let (num_cpu, mem_end) = {
        let fdt = match Fdt::new(fdt_slice) {
            // let fdt = match unsafe { Fdt::from_ptr(fdt_addr as *const u8) } {
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
        (num_cpu, mem_end)
    };
    (num_cpu, mem_end)
}
