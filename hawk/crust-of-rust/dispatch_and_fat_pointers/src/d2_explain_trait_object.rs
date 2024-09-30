// https://doc.rust-lang.org/book/ch17-02-trait-objects.html

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
    bar(&[Box::new("J"), Box::new("Jon")]);
    bar(&[Box::new(String::from("J")), Box::new(String::from("Jon"))]);
    bar(&[Box::new("J"), Box::new(String::from("Jon"))]);

    bar2(&[&"J", &"Jon"]);
    bar2(&[&String::from("J"), &String::from("Jon")]);
    bar2(&[&"J", &String::from("Jon")]);
}

// We want to create a vector that each element has the same behavior
// without caring about it concrete type.
//
// We can use Box or reference to make each element in the slice has
// the same size. Or the compiler might not be able to compile this code.
//
// Box itself is Sized, but the T in Box<T> doesn't need to be Sized.
pub fn bar(arr: &[Box<dyn Hei>]) {
    for e in arr {
        e.hei();
    }
}
// And a reference is always Sized, it just a pointer (u8).
pub fn bar2(arr: &[&dyn Hei]) {
    for e in arr {
        e.hei();
    }
}
