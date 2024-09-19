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
        // `compare_exchange` are implemented with those instructions depending on different archicture
        // x86: `XCHG`
        // ARM:
        // - compare_exchange: impl using a loop of `LDREX` and `STREX` pair
        // - compare_exchange_weak: `LDREX` `STREX` pair
        // RISC-V: `AMOSWAP`
        //
        // That's why you should almost always use `compare_exchange_weak` in a loop.
        while self
            .locked
            .compare_exchange_weak(UNLOCKED, LOCKED, Ordering::Relaxed, Ordering::Relaxed)
            .is_err()
        {
            // MESI protocol
            while self.locked.load(Ordering::Relaxed) == LOCKED {
                std::thread::yield_now();
            }
            std::thread::yield_now();
        }
        let ret = f(unsafe { &mut *self.v.get() });
        self.locked.store(UNLOCKED, Ordering::Relaxed);
        ret
    }
}
