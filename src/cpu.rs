use core::{
    cell::UnsafeCell,
    mem::{self, MaybeUninit},
};

use crate::arch::{disable_interrupts, enable_interrupts, hart_id, intr_get};

pub static CPUS: Cpus = Cpus::init();
pub const MAX_CPUS: usize = 24;

pub struct Cpus([UnsafeCell<Cpu>; MAX_CPUS]);
unsafe impl Sync for Cpus {}

// https://doc.rust-lang.org/core/mem/union.MaybeUninit.html#initializing-an-array-element-by-element
// https://github.com/o8vm/octox/blob/main/src/kernel/defs.rs
macro_rules! array {
    [$type:ty; $count:expr; $init:expr] => {
        {
            let mut data: [core::mem::MaybeUninit<$type>; $count] =
                [const { MaybeUninit::uninit() }; $count];

            let mut idx = 0;
            while idx < $count {
                data[idx] = core::mem::MaybeUninit::new($init);
                idx += 1;
            }

            unsafe {
                mem::transmute::<
                    [core::mem::MaybeUninit<$type>; $count],
                    [$type; $count]
                >(data)
            }
        }
    };
}

impl Cpus {
    const fn init() -> Cpus {
        Cpus(array!(UnsafeCell<Cpu>; MAX_CPUS; UnsafeCell::new(Cpu::new())))
    }

    // Safety: TODO
    pub fn current(&self) -> &mut Cpu {
        unsafe { &mut *(self.0[hart_id()].get()) }
    }
}

#[derive(Clone, Copy)]
pub struct Cpu {
    // levels of interrupt disable through push_off
    pub n_off: u8,
    // were interrupts enabled when first push_off?
    pub ir_enabled: bool,
}

impl Cpu {
    const fn new() -> Cpu {
        Cpu {
            n_off: 0,
            ir_enabled: false,
        }
    }

    /// Disable interrupts and keep track of disable nesting
    pub fn push_off(&mut self) {
        let int_enabled = intr_get();
        disable_interrupts();
        if self.n_off == 0 {
            self.ir_enabled = int_enabled;
        }
        self.n_off += 1;
    }

    /// Pop interrupt disable, enable interrupts if no disable remains
    pub fn pop_off(&mut self) {
        if intr_get() {
            panic!("interrupts enabled when calling pop_off");
        }
        if self.n_off < 1 {
            panic!("n_off non-positive when calling pop_off");
        }
        self.n_off -= 1;
        if self.n_off == 0 && self.ir_enabled {
            enable_interrupts();
        }
    }
}
