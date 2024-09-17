fn main() {}

#[test]
fn test_naive_implementation() {
    use atomics::a1_naive_implementation::Mutex;
    use std::thread::spawn;

    let l: &'static _ = Box::leak(Box::new(Mutex::new(0)));
    let handles: Vec<_> = (0..10)
        .map(|_| {
            spawn(move || {
                for _ in 0..100 {
                    l.with_lock(|v| {
                        *v += 1;
                    });
                }
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
    assert_eq!(l.with_lock(|v| *v), 10 * 100);
}
