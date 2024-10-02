// https://doc.rust-lang.org/reference/items/traits.html#object-safety

pub trait Hei {
    fn hei(&self);

    // The weird method is opted out in the vtable of trait object
    // since we say self is sized.
    fn weird()
    where
        Self: Sized,
    {
    }
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

pub fn say_hei(s: &dyn Hei) {
    s.hei();
}
