use core::{alloc::GlobalAlloc, ptr};

use crate::{lock::SpinLock, println};

#[global_allocator]
pub static KMEM: PhysicalMemoryManager= PhysicalMemoryManager(SpinLock::new(PhysMem::new()));

pub struct PhysicalMemoryManager(SpinLock<PhysMem>);
impl PhysicalMemoryManager {
    pub fn init(&self, start: usize, exclusive_end: usize) {
        self.0.lock().init(start, exclusive_end);
    }

    pub fn is_locked(&self) -> bool {
        self.0.is_locked()
    }
}

unsafe impl GlobalAlloc for PhysicalMemoryManager {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        if layout.size() > PAGE_SIZE || layout.align() > PAGE_SIZE {
            return ptr::null_mut()
        }
        self.0.lock().alloc_page()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        self.0.lock().free_page(ptr);
    }
}

pub const PAGE_SIZE: usize = 4096;

/// Rounds up the address to the start of next page
pub fn page_round_up(address: usize) -> usize {
    // Make sure address is within the next page (if not aligned already)
    (address + PAGE_SIZE - 1) 
    // Zero out all the lower bits (works because page size is multiple of 2)
    & !(PAGE_SIZE - 1)
}

/// Rounds down the address to the start of current page
pub fn page_round_down(address: usize) -> usize {
    // Zero out all the lower bits (works because page size is multiple of 2)
    address & !(PAGE_SIZE - 1)
}

// Rounds down to next 16 byte alignment
pub fn align_16(address: usize) -> usize {
    (address + 15) & !15
}

pub struct PhysMem {
    freelist: Option<*mut PhysPage>,
    amount_pages: usize,
}

impl PhysMem {
    const fn new() -> Self {
        Self { freelist: None, amount_pages: 0 }
    }

    // TODO: check whether this is actually safe
    fn alloc_page(&mut self) -> *mut u8 {
        let Some(head) = self.freelist else {
            return ptr::null_mut();
        };
        self.freelist = unsafe { (*head).next };
        head as *mut u8
    }

    // TODO: check whether this is actually safe
    fn free_page(&mut self, page: *mut u8) {
        let head = unsafe {
          &mut *(page as *mut PhysPage)
        };
        head.next = self.freelist;
        self.freelist = Some(head as *mut PhysPage);
    }

    fn init(&mut self, start: usize, exclusive_end: usize) {
        println!("PhysMem.init");
        println!("start address {:#x} ({})", start, start);
        println!("end address {:#x} ({})", exclusive_end, exclusive_end);
        let mut page_start_addr = page_round_up(start);
        let exclusive_end = page_round_down(exclusive_end);
        // println!("aligned start address {:#x} ({})", page_start_addr, page_start_addr);
        // println!("aligned end address {:#x} ({})", exclusive_end, exclusive_end);

        // current points to tail of the freelist
        let mut current = &mut self.freelist;
        while page_start_addr < exclusive_end {
            // construct valid FreeNode, 
            // representing page starting at page_start_addr
            let page = unsafe {
                &mut *(page_start_addr as *mut PhysPage)
            };
            page.next = None;
            *current = Some(page);
            current = &mut page.next;
            page_start_addr += PAGE_SIZE;
            self.amount_pages += 1;
        }
        println!("PhysMem initialized, {} free pages", self.amount_pages);
    }
}

struct PhysPage {
    next: Option<*mut PhysPage>,
}

// Safety: FreeNode are only constructed for use in PhysMem,
// which only has a single global instance protected by a lock
unsafe impl Send for PhysPage {}
unsafe impl Sync for PhysPage {}
unsafe impl Send for PhysMem {}
unsafe impl Sync for PhysMem {}
