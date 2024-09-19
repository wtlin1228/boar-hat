#[test]
fn too_relaxed() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::thread::spawn;

    let x: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));
    let y: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));
    let t1 = spawn(move || {
        let r1 = y.load(Ordering::Relaxed);
        x.store(r1, Ordering::Relaxed);
        r1
    });
    let t2 = spawn(move || {
        // Just like compiler can optimize the code by reordering.
        // CPU can also change the execution order to make it more performant
        // if the instruction are independent.
        //
        // So, y could be stored with 42 before r2 read from x.
        // Like this:
        //   y.store(42, Ordering::Relaxed);
        //   let r2 = x.load(Ordering::Relaxed);
        //
        let r2 = x.load(Ordering::Relaxed);
        y.store(42, Ordering::Relaxed);
        r2
    });

    let r1 = t1.join().unwrap();
    let r2 = t2.join().unwrap();

    // this can happen: r1 == r2 == 42 🚨
}
