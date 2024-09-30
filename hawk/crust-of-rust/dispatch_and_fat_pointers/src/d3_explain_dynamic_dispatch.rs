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
