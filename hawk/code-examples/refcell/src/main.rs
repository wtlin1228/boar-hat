use std::cell::RefCell;

fn ref_cell() {
    // a is not mutable, but the inner value of RefCell is mutable
    let a = RefCell::new(5);

    let a_ref = a.borrow();
    assert_eq!(*a_ref, 5);

    // borrow rules are checked dynamically
    let a_mut_ref = &mut a.borrow_mut();
    // let a_mut_ref_2 = &mut a.borrow_mut();
    // |                 ^^^^^^^^^^^^^^^^^^^ already borrowed: BorrowMutError

    **a_mut_ref = 42;

    assert_eq!(*a.borrow(), 42);
}

fn without_ref_cell() {
    let mut a = 5;

    let a_ref = &a;
    assert_eq!(*a_ref, 5);

    // borrow rules are checked statically
    let a_mut_ref = &mut a;
    // |            ------ first mutable borrow occurs here
    // let a_mut_ref_2 = &mut a;
    // |                 ^^^^^^ second mutable borrow occurs here

    *a_mut_ref = 42;

    assert_eq!(a, 42);
}

fn main() {
    ref_cell();
    without_ref_cell();
}
