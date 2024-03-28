#![feature(dropck_eyepatch)]

use std::marker::PhantomData;
use std::ptr::NonNull;

// If the type is generic over T, then Rust Compiler will assume dropping the type will access the T
// So, here because Boks<T> implements the Drop trait, compiler will assume dropping the Boks<T> will
// use the T.
pub struct Boks<T> {
    // p: *mut T,           // this makes Boks<T> invariant over T
    p: NonNull<T>,      // this makes Boks<T> covariant over T
    _t: PhantomData<T>, // this makes dropping Boks<T> also drops the T
}

// An escape hatch for drop checking
// - https://doc.rust-lang.org/nomicon/dropck.html#an-escape-hatch
// - https://forge.rust-lang.org/libs/maintaining-std.html#is-there-a-manual-drop-implementation
unsafe impl<#[may_dangle] T> Drop for Boks<T> {
    fn drop(&mut self) {
        // Safety: p was constructed from a Box in the first place, and has not been freed,
        // otherwise since self still exists (otherwise, drop could not be called)
        unsafe {
            let _ = Box::from_raw(self.p.as_mut());
        };
    }
}

impl<T> Boks<T> {
    pub fn ny(t: T) -> Self {
        Self {
            // Safety: Box never creates a null pointer
            p: unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(t))) },
            _t: PhantomData,
        }
    }
}

impl<T> std::ops::Deref for Boks<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // Safety: is valid since it was constructed from a valid T, and turned into a pointer
        // through Box which creates aligned pointers, and hasn't been freed, since self is alive.
        unsafe { &*self.p.as_ref() }
    }
}

impl<T> std::ops::DerefMut for Boks<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Safety: is valid since it was constructed from a valid T, and turned into a pointer
        // through Box which creates aligned pointers, and hasn't been freed, since self is alive.
        // Also, since we have &mut self, no other mutable reference has been given out to p.
        unsafe { &mut *self.p.as_mut() }
    }
}

use std::fmt::Debug;
struct TaPå<T: Debug>(T);

impl<T: Debug> Drop for TaPå<T> {
    fn drop(&mut self) {
        println!("{:?}", self.0);
    }
}

fn main() {
    // ----------------------------------------
    // --- Base case: just to test it works ---
    // ----------------------------------------
    let x = 42;
    let b = Boks::ny(x);
    println!("{:?}", *b);

    // ----------------------------------------------------------------------------------------
    // --- 1st case: why do we need to add #[may_dangle] on the `impl<T> Drop for Boks<T>`? ---
    // ----------------------------------------------------------------------------------------
    let mut x = 42;
    let b = Box::new(&mut x);
    println!("{:?}", x); // This is compiled (Good)
                         // Because Box<T> doesn't use the T when Box<T> is dropped so the
                         // lifetime of b can be shorten

    let mut x = 42;
    let b = Boks::ny(&mut x);
    println!("{:?}", x); // This isn't compiled (Bad)
                         // Since Boks can touch the T when it's dropped so the lifetime of b cannot
                         // be shorten
                         //
                         // The trick here is telling the compiler Boks<T> won't use the T when it's dropped.
                         // See #[may_dangle] on the impl Drop for Boks<T>

    // ----------------------------------------------------------------------------
    // --- 2nd case: why do we need to add PhantomData on the `struct Boks<T>`? ---
    // ----------------------------------------------------------------------------
    // let mut x = 42;
    // let b = Box::new(TaPå(&mut x));
    // println!("{:?}", x); // This isn't compiled (Good)
    //                      // Since TaPå<T> will use the T on drop, so the compiler can't shorten the
    //                      // lifetime of x, then causes the borrow checker error.

    // let mut x = 42;
    // let b = Boks::ny(TaPå(&mut x));
    // println!("{:?}", x); // This is compiled (Bad)
    //                      // Since we tell compiler that Boks<T> won't use the T when it's dropped by
    //                      // adding the #[may_dangle]. And compiler assumes that T won't be dropped,
    //                      // but actually the T will be dropped.
    //                      //
    //                      // We don't have to deal with this problem before is because accessing the T
    //                      // on drop is stronger than dropping the T. But now we ensure the compiler we
    //                      // won't access the T, so compiler assumes that we also don't drop the T.
    //                      //
    //                      // The trick here is telling the compiler dropping the Boks<T> will also drop the T.
    //                      // See PhantomData on the struct Boks<T>

    // --------------------------
    // --- 3rd case: Variance ---
    // --------------------------
    let s = String::from("hei");
    let mut box1 = Box::new(&*s);
    let box2: Box<&'static str> = Box::new("heisann");
    box1 = box2; // This is compiled (Good)
                 // Since Box<T> is covariant over T, so we can provide it with a more useful type.

    let s = String::from("hei");
    let mut boks1 = Boks::ny(&*s);
    let boks2: Boks<&'static str> = Boks::ny("heisann");
    boks1 = boks2; // This isn't compiled (Bad)
                   // Since *mut T makes T invariant, so we can only provide it with the same type.
                   //
                   // The trick here is using NonNull<T> instead of *mut T in the struct Boks<T>
                   // to make it Boks<T> covariant over T. That's because NonNull<T> makes T covariant.
}
