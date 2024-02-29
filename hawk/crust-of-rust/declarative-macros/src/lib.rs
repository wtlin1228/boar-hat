#[macro_export]
macro_rules! avec {
    ($($element:expr),*) => {{
        // check that count is const
        const C: usize = ($crate::count![@COUNT; $($element),*]);

        #[allow(unused_mut)]
        let mut vs = Vec::with_capacity(C); // to avoid allocating more space
        $(vs.push($element);)*
        vs
    }};
    ($($element:expr,)*) => {{
        $crate::avec![$($element),*]
    }};
    ($element:expr; $count:expr) => {{
        let count = $count;
        let mut vs = Vec::with_capacity(count); // to avoid allocating more space
        vs.resize(count, $element);
        // vs.extend(::std::iter::repeat($element).take(count));
        vs
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! count {
    (@COUNT; $($element:expr),* ) => {
        // use Slice's len method to get the length of reference of this Vec
        <[()]>::len(&[$($crate::count![@SUBST; $element]),*])
    };
    (@SUBST; $_element:expr) => { () };
}

#[test]
fn empty_vec() {
    let x: Vec<u32> = avec![];
    assert!(x.is_empty());
}

#[test]
fn single() {
    let x: Vec<u32> = avec![42];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 1);
    assert_eq!(x[0], 42);
}

#[test]
fn double() {
    let x: Vec<u32> = avec![42, 43];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 43);
}

#[test]
fn trailing() {
    let x: Vec<u32> = avec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30,
    ];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 30);
    assert_eq!(x[0], 1);
    assert_eq!(x[1], 2);
}

#[test]
fn clone_2() {
    let x: Vec<u32> = avec![42; 2];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 42);
}

#[test]
fn clone_2_nonliteral() {
    let mut y = Some(42);
    let x: Vec<u32> = avec![y.take().unwrap(); 2];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 42);
}

/// ```compile_fail
/// let x: Vec<u32> = vecmac::avec![42; "foo"];
/// ```
#[allow(dead_code)]
struct CompileFailTest;
