//! Virtual memory system
//!
//! Paging system uses Sv39 paging scheme

use core::fmt::Display;

use crate::mem::KMEM;

// one beyond the highest possible virtual address.
// MAXVA is actually one bit less than the max allowed by
// Sv39, to avoid having to sign-extend virtual addresses
// that have the high bit set.
const MAX_VA: usize = 1 << (9 + 9 + 9 + 12 - 1);
const VA_OFFSET: u8 = 12;

struct PageTable {
    entries: [PageTableEntry; 512],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
struct PageTableEntry(usize);

impl PageTableEntry {
    fn to_pa(&self) -> PhysicalAddress {
        PhysicalAddress((self.0 >> 10) << 12)
    }

    fn is_valid(&self) -> bool {
        todo!()
    }
}

struct PhysicalAddress(usize);
struct VirtualAddress(usize);
impl Display for VirtualAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        todo!()
    }
}

impl VirtualAddress {
    /// get index into next level pagetable
    fn vpn(&self, level: u8) -> usize {
        assert!(level < 3);
        (self.0 >> (VA_OFFSET + (level * 9))) & 0x1FF
    }
}

// Find PTE corresponding to given va and pagetable
// If alloc, create non-existing pagetable pages
fn walk(
    mut pagetable: *const PageTable,
    va: VirtualAddress,
    alloc: bool,
) -> Option<*const PageTableEntry> {
    if va.0 > MAX_VA {
        panic!("invalid va: {va}");
    }
    for level in (1..2).rev() {
        let index = va.vpn(level);
        let pte = unsafe { (*pagetable).entries[index] };
        if pte.is_valid() {
            pagetable = pte.to_pa().0 as *const PageTable;
        } else if alloc {
            // for own types, how to do physical memory allocation?
            // need to allocate a page here for new pagetable
            todo!()
        } else {
            return None;
        }
    }

    let index = va.vpn(0);
    let pte_ptr = &(unsafe { (*pagetable).entries[index] }) as *const PageTableEntry;
    Some(pte_ptr)
}
