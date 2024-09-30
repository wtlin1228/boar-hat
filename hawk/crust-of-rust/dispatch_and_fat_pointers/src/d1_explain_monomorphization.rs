// https://doc.rust-lang.org/book/ch10-01-syntax.html

pub fn strlen(s: impl AsRef<str>) -> usize {
    s.as_ref().len()
}

// The compiler actually generates two copies of the `strlen`, it takes
// the generic definition of `strlen` and turn it into multiple non-generic
// implementations. This is the process of monomorphization.
//
// This is also one of the reason why Rust can't just compiles Rust code then
// ship people with the binary that people can use as a library. Because Rust
// need the source in order to generate all those implementations.
//
// This process is great because you end up producing much efficient code.

// compiler will generate this 👇
pub fn strlen_refstr(s: &str) -> usize {
    s.len()
}
// compiler will generate this 👇
pub fn strlen_string(s: String) -> usize {
    s.len()
}
