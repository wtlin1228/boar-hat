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
        // 1️⃣ Without the `Ordering::Acquire`, all the subsequent operations are not ordered.
        // so the line `let ret = f(unsafe { &mut *self.v.get() });` might not be visible to
        // all threads that are trying to store this memory with `Ordering::Release`.
        //
        // 2️⃣ The second ordering is used when you failed to do the store, in our case is when
        // you failed to take the lock.
        while self
            .locked
            .compare_exchange_weak(UNLOCKED, LOCKED, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            // MESI protocol
            while self.locked.load(Ordering::Relaxed) == LOCKED {
                std::thread::yield_now();
            }
            std::thread::yield_now();
        }
        let ret = f(unsafe { &mut *self.v.get() });
        // 👇 Without the `Ordering::Release`, all the previous operations are not ordered.
        // so the previous line `let ret = f(unsafe { &mut *self.v.get() });` might not be
        // visible to all threads that are trying to load this memory with `Ordering::Acquire`.
        self.locked.store(UNLOCKED, Ordering::Release);
        ret
    }
}
