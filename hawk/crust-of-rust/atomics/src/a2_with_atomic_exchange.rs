use std::{
    cell::UnsafeCell,
    sync::atomic::{AtomicBool, Ordering},
};

const LOCKED: bool = true;
const UNLOCKED: bool = false;

pub struct Mutex<T> {
    locked: AtomicBool,
    v: UnsafeCell<T>,
}

unsafe impl<T> Sync for Mutex<T> where T: Send {}

impl<T> Mutex<T> {
    pub fn new(t: T) -> Self {
        Self {
            locked: AtomicBool::new(UNLOCKED),
            v: UnsafeCell::new(t),
        }
    }

    pub fn with_lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        // Performance problem:
        // compare_exchange is an expensive operation since OS need to coordinate
        // between all the CPUs to get the exclusive access of the specific memory
        while self
            .locked
            .compare_exchange(UNLOCKED, LOCKED, Ordering::Relaxed, Ordering::Relaxed)
            .is_err()
        {
            // MESI protocol is used to solve the performance problem of
            // compare_exchange, it uses a inner loop to read the lock.
            // So, if we failed the compare_exchange, we just spining here
            // until the lock is unlicked then try to compare_exchange again.
            while self.locked.load(Ordering::Relaxed) == LOCKED {}
        }
        // 🔓 no another thread can run here simutanuously
        // 👇 use yield_now() to confirm this situation
        std::thread::yield_now();
        let ret = f(unsafe { &mut *self.v.get() });
        self.locked.store(UNLOCKED, Ordering::Relaxed);
        ret
    }
}
