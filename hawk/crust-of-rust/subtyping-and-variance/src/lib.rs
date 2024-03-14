// # Prerequisite
//
// - 'static is a subtype of any lifetime
// - Functions which can operate on elements of the supertype,
//   can also operate on elements of the subtype.
// - T is a subtype of U, if T is at least as useful as U.
//
// # Covariance
//
// F<T> is covariant over T if T being a subtype of U implies that
// F<T> is a subtype of F<U> (subtyping "passes through")
//
// ## Try to understand
//
// fn foo(&'a str) {}
// let x: &'a str
//
// foo(&'a str) ✅
// foo(&'static str) ✅
// x = &'a str ✅
// x = &'static str ✅
//
// # Contravariance
//
// F<T> is contravariant over T if T being a subtype of U implies
// that F<U> is a subtype of F<T>
//
// ## Try to understand
//
// foo here expects to be able to use bar with something with
// particular lifetime. So we can't give foo a function which
// is expecting a more useful type as its argument.
//
// fn foo(bar: Fn(&'a str) -> ()) {
//     bar("" /* 'a */)
// }
// let x: Fn(&'a str) -> ()
//
// foo(fn(&'static str) {}) ❌
// x = fn(&'static str) {} ❌
//
// Which one is more useful?
// - Fn(&'static str)
// - Fn(&'a str) 🎖️
//
// Which one is more useful?
// - &'static str 🎖️
// - &'a str
//
// # Invariance
//
// F<T> is invariant over T otherwise (no subtyping relation
// can be derived)
//
// ## Why T is invariant in &'a mut T
//
// fn foo(s: &mut &'a str, x: &'a str) {
//     *s = x;
// }
//
// let mut x: &'static str = "hello world";
// let z = String::new();
// foo(&mut x, &z);
//   // foo(&mut &'a      str, &'a str)
//   // foo(&mut &'static str, &'a str)
//   // ❌ not compiled since T is invariant in &'a mut T
// drop(z);
// println!("{}", x);
//
// ## Why 'a is covariant in &'a mut T
//
// fn foo() {
//     let mut y = true;
//     let mut z /* &'y mut bool */ = &mut y;
//
//     let x = Box::new(true);
//     let x: &'static mut bool = Box::leak(x);
//
//     z = x; // &'y mut bool = &'static mut bool
// }
//
// # Reference
//
// - https://en.wikipedia.org/wiki/Subtyping
// - https://doc.rust-lang.org/nomicon/subtyping.html
// - https://doc.rust-lang.org/nightly/reference/subtyping.html

// The reason for using those different shape of PhantomData<T> fields
// here is because of variance.
use std::marker::PhantomData;
struct Deserializer<T> {
    // some fields
    _t: PhantomData<T>, // indicate the struct contains the T
                        // means that this struct will drop the T
                        // this one is covariant
}
struct Deserializer2<T> {
    // some fields
    _t: PhantomData<fn() -> T>, // indicate the struct doesn't contains the T
                                // means that this struct can't drop the T
                                // this one is covariant
}
struct Deserializer3<T> {
    // some fields
    _t: PhantomData<fn(T)>, // this one is contravariant
}
struct Deserializer4<T> {
    // some fields
    _t: PhantomData<fn(T) -> T>, // this one is invariant
                                 // since it's both covariant and contravariant
}

pub fn bad_strtok<'a>(s: &'a mut &'a str, delimiter: char) -> &'a str {
    ""
}

pub fn strtok<'a, 'b>(s: &'a mut &'b str, delimiter: char) -> &'b str {
    if let Some(i) = s.find(delimiter) {
        let prefix = &s[..i];
        let suffix = &s[(i + delimiter.len_utf8())..];
        *s = suffix;
        return prefix;
    } else {
        let prefix = *s;
        *s = "";
        prefix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_not_works() {
        let mut x: &'static str = "hello world";
        // <'a> &'a      mut &'a      str // argument
        //      &'static mut &'static str
        // Since T of &'a mut T is invariant compiler can't shrink
        // the type of x from &'static str to &'a str.
        // Then due to the generic argument, compiler expects &mut x
        // to be 'static, but &mut x is scoped locally.
        // That's why it's not compiled.
        let hello = bad_strtok(&mut x, ' ');
        //                     ^^^^^^ borrowed value does not live long enough
        assert_eq!(hello, "hello");
    }

    #[test]
    fn it_works() {
        let mut x: &'static str = "hello world";
        let hello = strtok(&mut x, ' ');
        assert_eq!(hello, "hello");
        assert_eq!(x, "world");
    }
}
