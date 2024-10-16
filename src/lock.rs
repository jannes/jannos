use core::cell::UnsafeCell;
use core::hint;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicBool, Ordering};

use crate::cpu::CPUS;
use crate::println;

pub struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

unsafe impl<T> Sync for SpinLock<T> where T: Send {}

impl<T> SpinLock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    pub fn is_locked(&self) -> bool {
        self.locked.load(Ordering::Relaxed)
    }

    pub fn lock(&self) -> Guard<T> {
        CPUS.current().push_off();
        println!("lock val: {}", self.locked.load(Ordering::Relaxed));
        while self.locked.swap(true, Ordering::Acquire) {
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
        self.lock.locked.store(false, Ordering::Release);
        CPUS.current().pop_off();
    }
}
