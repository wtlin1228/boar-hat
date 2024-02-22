use std::cell::UnsafeCell;

pub struct Cell<T> {
    value: UnsafeCell<T>,
}

// implied by UnsafeCell
// impl<T> !Sync for Cell<T> {}

#[allow(dead_code)]
impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        // SAFETY: we know no-one else is concurrently mutating self.value (because !Sync)
        // SAFETY: we know we're not invalidating any references, because we never give any out
        unsafe {
            *self.value.get() = value;
        }
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        // SAFETY: we know no-one else is modifying this value, since only this thread can mutate
        // (because !Sync), and it is executing this function instead.
        unsafe { *self.value.get() }
    }

    #[cfg(test)]
    // This method is for illustration purposes only.
    // Cell should not give out any reference of its holding value.
    fn get_ref(&self) -> &T {
        unsafe { &*self.value.get() }
    }
}

#[cfg(test)]
mod test {
    use super::Cell;

    #[test]
    fn show_why_giving_out_reference_is_not_ok() {
        let x = Cell::new(vec![vec![42]]);
        let first: &Vec<i32> = &x.get_ref()[0];
        eprintln!("{:?}", first); // [42]

        x.set(vec![vec![10, 11, 12]]);
        eprintln!("{:?}", first); // ❓ []
                                  // the original value vec![42] is deallocated
                                  // but first still hold a reference to somewhere

        x.set(vec![vec![1, 2, 3]]);
        eprintln!("{:?}", first); // ❓ [1, 2, 3]
                                  // now the first is pointing to the memory of vec![1, 2, 3]
                                  // it's unstable and broken
                                  // that's why Cell should never give out &T
    }
}
