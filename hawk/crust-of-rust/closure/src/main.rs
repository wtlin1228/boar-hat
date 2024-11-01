fn main() {
    let f1 = || {
        println!("f1");
    };
    call_fn(f1);
    call_fn_mut(f1); // trait Fn<Args>: FnMut<Args>
    call_fn_once(f1); // trait FnMut<Args>: FnOnce<Args>

    let mut x = String::from("Hello");
    let mut f2 = || {
        x.push('🐽');
        println!("{}", x);
    };
    call_fn_mut(&mut f2);
    call_fn_once(&mut f2); // trait FnMut<Args>: FnOnce<Args>

    let y = String::from("Hello");
    let f3 = || {
        drop(y);
    };
    call_fn_once(f3);
}

fn call_fn(f: impl Fn() -> ()) {
    f();
}

fn call_fn_mut(mut f: impl FnMut() -> ()) {
    f();
}

fn call_fn_once(f: impl FnOnce() -> ()) {
    f();
}
