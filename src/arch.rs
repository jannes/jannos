use core::arch::asm;

pub fn hart_id() -> usize {
    let id;
    unsafe {
        asm!("mv {0}, tp", out(reg) id);
    }
    id
}
