// https://doc.rust-lang.org/reference/dynamically-sized-types.html

pub trait Hei {
    fn hei(&self);
}

impl Hei for &str {
    fn hei(&self) {
        println!("hei {}", self);
    }
}

impl Hei for String {
    fn hei(&self) {
        println!("hei {}", self);
    }
}

pub fn foo() {
    bar(&"S");
}

pub fn bar(s: &dyn Hei) {
    s.hei();
}

// Why this won't work?
// In this case, compiler needs to construct three pointers
//   1. pointer to the concrete type
//   2. pointer to the Hei vtable
//   3. pointer to the AsRef<str> vtable
// So it won't work.
//
// uncomment the following to see the compiler error
// pub fn bad_baz(s: &(dyn Hei + AsRef<str>)) {
//     s.hei();
//     let s = s.as_ref();
//     s.len();
// }

pub trait HeiAsRef: Hei + AsRef<str> {}

// It works because by explicitly defining a new trait compiler can
// make a vtable contains both Hei's methods and AsRef<str>'s methods.
pub fn baz(s: &dyn HeiAsRef) {
    s.hei();
    let s = s.as_ref();
    s.len();
}
