#![no_std]
#![no_main]

extern crate alloc;

use core::{
    arch::{asm, global_asm},
    hint::black_box,
    slice,
};

use alloc::string::String;
use arch::hart_id;
use fdt::Fdt;
use mem::{align_16, KMEM, PAGE_SIZE};
use sbi::sbi_hart_start;

mod arch;
mod cpu;
mod lock;
mod mem;
mod print;
mod sbi;

global_asm!(include_str!("boot.s"));

pub const STACK_SIZE: usize = 16 * PAGE_SIZE;

extern "C" {
    fn _start_non_boot_core();
    static _HEAP_START: usize;
    static _MEM_END: usize;
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println_panic!("panic: {}", info);
    loop {
        unsafe {
            asm!("wfi");
        }
    }
}

// the first Rust code executed by boot core on temporary stack
// here we just get essential hardware info, like amount of cores and memory
#[no_mangle]
pub extern "C" fn kinit(fdt_addr: usize) -> ! {
    let hart_id = hart_id();
    println!("I am boot core ({}), fdt_addr: {:#x}", hart_id, fdt_addr);

    // determine amount of cores and memory space through devicetree
    println!("kmem addr: {:p}, locked: {}", &KMEM, KMEM.is_locked());
    let (num_cpu, mem_end) = hw_info(fdt_addr);
    println!("kmem addr: {:p}, locked: {}", &KMEM, KMEM.is_locked());

    // heap starts right after bss section, symbol exported by linker script
    // symbol address is the actual value that we set it to (confusing!)
    let heap_start = unsafe { ((&_HEAP_START) as *const usize) as usize };
    // heap should end right before kernel stacks which are at top of memory space
    let heap_end_exclusive = align_16(mem_end) - (num_cpu * STACK_SIZE);
    println!(
        "heap start: {:#x}, heap end: {:#x}",
        heap_start, heap_end_exclusive
    );

    println!("init physical memory heap");
    KMEM.init(heap_start, heap_end_exclusive);

    let _s = String::with_capacity(20);
    drop(_s);

    start_other_cores(hart_id, num_cpu, mem_end);
    panic!("kinit done, TODO: switch boot core stack and continue in kmain");
}

// second call into Rust for boot core, now on its normal stack
#[no_mangle]
pub extern "C" fn kmain() -> ! {
    unimplemented!()
}

// Non-boot cores' entry into Rust
#[no_mangle]
pub extern "C" fn start_non_boot_core() -> ! {
    println!("I am non-boot core ({}), going to sleep", hart_id());
    loop {
        unsafe {
            asm!("wfi");
        }
    }
}

// Starts non-boot cores, setting up their stacks in Assembly
// and then calling back into Rust just sleeping for now
fn start_other_cores(boot_core_id: usize, num_cpu: usize, mem_end: usize) {
    println!("starting non-boot cores");
    let start_addr = _start_non_boot_core as usize;
    for i in 0..num_cpu {
        if i != boot_core_id {
            let sp = align_16(mem_end) - (i * STACK_SIZE);
            sbi_hart_start(i, start_addr, sp);
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
