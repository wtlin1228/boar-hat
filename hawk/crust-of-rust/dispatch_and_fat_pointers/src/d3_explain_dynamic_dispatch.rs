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
    // &dyn Hei
    // stored in &
    //   1. a pointer to the actual, concrete, implementing type
    //   2. a pointer to a vtable for the referenced trait
    //
    // What is a vtable?
    // - vtables are virtual dispatch tables
    // - it's a little data structure that has pointers to each of the
    //   methods for the trait for the type.
    // - A different vtable ends up being constructed for each concrete
    //   type turned into a trait object.
    //
    // When we have a &str and want to convert it into a &dyn Hei
    // &str -> &dyn Hei
    //   1. pointer to the &str
    //   2. &HeiVtable {
    //        hei: &<&str as Hei>::hei // Line 8
    //      }
    //
    // For &String -> &dyn Hei
    //   1. pointer to the String
    //   2. &HeiVtable {
    //        hei: &<String as Hei>::hei // Line 15
    //      }
    s.hei();
    // s.vtable.hei(s.pointer)
}
