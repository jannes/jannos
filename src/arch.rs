use core::arch::asm;

pub fn hart_id() -> usize {
    let id;
    unsafe {
        asm!("mv {0}, tp", out(reg) id);
    }
    id
}

// Supervisor Interrupt Enable bitmask
const SSTATUS_SIE: u64 = 1 << 1;

// are device interrupts enabled?
pub fn intr_get() -> bool {
    sstatus_read() & SSTATUS_SIE != 0
}

pub fn enable_interrupts() {
    let sstatus = sstatus_read() | SSTATUS_SIE;
    sstatus_write(sstatus);
}

pub fn disable_interrupts() {
    let sstatus = sstatus_read() & !SSTATUS_SIE;
    sstatus_write(sstatus);
}

pub fn sstatus_read() -> u64 {
    let sstatus;
    unsafe {
        asm!("csrr {0}, sstatus", out(reg) sstatus);
    }
    sstatus
}

pub fn sstatus_write(sstatus: u64) {
    unsafe {
        asm!("csrw sstatus, {0}", in(reg) sstatus);
    }
}
