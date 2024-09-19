#[test]
fn z_can_be_0_or_1_or_2() {
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::thread::spawn;

    let x: &'static _ = Box::leak(Box::new(AtomicBool::new(false)));
    let y: &'static _ = Box::leak(Box::new(AtomicBool::new(false)));
    let z: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));

    let _tx = spawn(move || {
        // Ordering::Release ensure all the operations before are seen by
        // the x.load(Ordering::Acquire), but here is no operations before
        // the store. So, after t1 saw x is true, t1 can see any value in
        // the MO(y).
        x.store(true, Ordering::Release);
    });
    let _ty = spawn(move || {
        // Same as _tx
        y.store(true, Ordering::Release);
    });
    let t1 = spawn(move || {
        while !x.load(Ordering::Acquire) {}
        // t1 can see MO(y), so y could be either false or true.
        if y.load(Ordering::Acquire) {
            z.fetch_add(1, Ordering::Relaxed);
        }
    });
    let t2 = spawn(move || {
        while !y.load(Ordering::Acquire) {}
        // t2 can see MO(x), so x could be either false or true.
        if x.load(Ordering::Acquire) {
            z.fetch_add(1, Ordering::Relaxed);
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();
    let z = z.load(Ordering::SeqCst);
    // What are the possible values for z?
    // - Is 0 possible?
    //   Resctirctions:
    //     we know that t1 must run "after" tx
    //     we know that t2 must run "after" ty
    //   Seems impossible to have a thread schedule where z == 0, but it could be
    //
    //   MO = modification order
    //
    //           t2   t1,t2
    //   MO(x): false true
    //
    //           t1   t1,t2
    //   MO(y): false true
    //
    // - Is 1 possible?
    //   Yes: tx, t1, ty, t2
    // - Is 2 possible?
    //   Yes: tx, ty, t1, t2
}

#[test]
fn z_can_not_be_0() {
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::thread::spawn;

    let x: &'static _ = Box::leak(Box::new(AtomicBool::new(false)));
    let y: &'static _ = Box::leak(Box::new(AtomicBool::new(false)));
    let z: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));

    let _tx = spawn(move || {
        x.store(true, Ordering::SeqCst);
    });
    let _ty = spawn(move || {
        y.store(true, Ordering::SeqCst);
    });
    let t1 = spawn(move || {
        while !x.load(Ordering::SeqCst) {}
        if y.load(Ordering::SeqCst) {
            z.fetch_add(1, Ordering::Relaxed);
        }
    });
    let t2 = spawn(move || {
        while !y.load(Ordering::SeqCst) {}
        if x.load(Ordering::SeqCst) {
            z.fetch_add(1, Ordering::Relaxed);
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();
    let z = z.load(Ordering::SeqCst);
    // Ordering::SeqCst guarantee that all threads see all sequentially
    // consistent operations in the same order. So, z can never be 0 since
    // t1 and t2 sees the same values of x and y
}
