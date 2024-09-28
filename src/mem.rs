use crate::{lock::SpinLock, println};

pub static KMEM: SpinLock<PhysMem> = SpinLock::new(PhysMem::new());

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

pub struct PhysMem {
    freelist: Option<*mut FreePage>,
    amount_pages: usize,
}

impl PhysMem {
    pub const fn new() -> Self {
        Self { freelist: None, amount_pages: 0 }
    }

    pub fn init(&mut self, start: usize, exclusive_end: usize) {
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
                &mut *(page_start_addr as *mut FreePage)
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

struct FreePage {
    next: Option<*mut FreePage>,
}

// Safety: FreeNode are only constructed for use in PhysMem,
// which only has a single global instance protected by a lock
unsafe impl Send for FreePage {}
unsafe impl Sync for FreePage {}
unsafe impl Send for PhysMem {}
unsafe impl Sync for PhysMem {}
