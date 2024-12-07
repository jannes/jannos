use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicPtr, Ordering};
use core::{hint, ptr};

use crate::arch::{disable_interrupts, intr_get};
use crate::cpu::{Cpu, CPUS};

pub struct SpinLock<T> {
    // name of lock for debugging purposes
    name: &'static str,
    // whether lock is locked and by which cpu (null if not locked)
    locked: AtomicPtr<Cpu>,
    // the actual value that the lock protects
    value: UnsafeCell<T>,
}

unsafe impl<T> Sync for SpinLock<T> where T: Send {}

impl<T> SpinLock<T> {
    pub const fn new(value: T, name: &'static str) -> Self {
        Self {
            name,
            locked: AtomicPtr::new(ptr::null_mut()),
            value: UnsafeCell::new(value),
        }
    }

    pub fn is_locked(&self) -> bool {
        self.locked.load(Ordering::Relaxed) != ptr::null_mut()
    }

    pub fn lock(&self) -> Guard<T> {
        let int_enabled = intr_get();
        disable_interrupts();
        let cpu = CPUS.current();

        // same CPU must not acquire same lock twice
        if self.locked.load(Ordering::Relaxed) == cpu {
            panic!("same CPU locked {} twice", self.name);
        }

        unsafe { (*cpu).push_off(int_enabled) };

        while self
            .locked
            .compare_exchange(ptr::null_mut(), cpu, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            hint::spin_loop();
        }

        Guard { lock: self }
    }
}

pub struct Guard<'a, T> {
    lock: &'a SpinLock<T>,
}

// Since Guard's DerefMut impl provides exclusive reference
// it should only be Sync/Send if T is as well
unsafe impl<T> Send for Guard<'_, T> where T: Send {}
unsafe impl<T> Sync for Guard<'_, T> where T: Sync {}

impl<T> Deref for Guard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        // Safety: Only one Guard can be constructed at a time,
        // it's lifetime is bound to the Spinlock being locked
        unsafe { &*self.lock.value.get() }
    }
}

impl<T> DerefMut for Guard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        // Safety: Only one Guard can be constructed at a time,
        // it's lifetime is bound to the Spinlock being locked
        unsafe { &mut *self.lock.value.get() }
    }
}

impl<T> Drop for Guard<'_, T> {
    fn drop(&mut self) {
        let cpu = self.lock.locked.swap(ptr::null_mut(), Ordering::Release);
        unsafe { (*cpu).pop_off() };
    }
}
