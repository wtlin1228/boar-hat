use crate::cell::Cell;
use std::{marker::PhantomData, ptr::NonNull};

// Where do we keep the reference count? We can't keep it in
// the `Rc<T>` because doing so makes each clone of `Rc<T>` has
// its own reference count. So, how do we ever know when the
// counter is 0? Instead, the reference count should in the
// value that is shared among all the copies of `Rc<T>`. So we
// create a `RcInner<T>` here.
struct RcInner<T> {
    value: T,
    refcount: Cell<usize>,
}

// Value can't be stored on the stack, since when stack
// goes away, the T goes away. So value should be allocated
// in the heap. It seems `Box<T>` is a good choice. But not
// actually, because when you clone the `RC<T>`, you clone
// the `Box<T>`. And when you clone the `Box<T>`, you clone
// the `T`. So we use a raw pointer here.
//
// One way to type a raw pointer is using `*const` or `*mut`
// But `*const` and `*mut` pointers can be pointing to null.
// So we use `std::ptr::NonNull` here.
//
// The `_marker` is telling the compiler that `Rc<T>` owns the
// `RcInner<T>`. If not doing so, compiler only knows `Rc<T>`
// has a pointer to `RcInner<T>`, so when `Rc<T>` gets dropped
// `RcInner<T>` won't be dropped, so the `T` won't be dropped
// either. See https://doc.rust-lang.org/nomicon/dropck.html and
// https://doc.rust-lang.org/std/marker/struct.PhantomData.html.
pub struct Rc<T> {
    inner: NonNull<RcInner<T>>,
    _marker: PhantomData<RcInner<T>>,
}

impl<T> Rc<T> {
    pub fn new(value: T) -> Self {
        let inner = Box::new(RcInner {
            value,
            refcount: Cell::new(1),
        });

        // Can't use `&*inner` because when this scope ends,
        // the box gets dropped, and the memory gets freed.
        // So we use `Box::into_raw(inner)` here.
        let ptr: *mut RcInner<T> = Box::into_raw(inner);

        Rc {
            // SAFETY: Box does not give us a null pointer.
            inner: unsafe { NonNull::new_unchecked(ptr) },
            _marker: PhantomData,
        }
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { self.inner.as_ref() };
        inner.refcount.set(inner.refcount.get() + 1);
        Rc {
            inner: self.inner,
            _marker: PhantomData,
        }
    }
}

impl<T> std::ops::Deref for Rc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY:
        // self.inner is a Box that is only deallocated when
        // the last Rc goes away. We have an Rc, therefore the
        // Box has not been deallocated, so deref is fine.
        &unsafe { self.inner.as_ref() }.value
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.inner.as_ref() };
        if inner.refcount.get() == 1 {
            // SAFETY: we are the _only_ Rc left, and we are
            // being dropped. therefore, after us, there will
            // be no Rc's, and no references to T.
            let _ = unsafe { Box::from_raw(self.inner.as_ptr()) };
        } else {
            // there are other Rcs, so don't drop the Box!
            inner.refcount.set(inner.refcount.get() - 1);
        }
    }
}
