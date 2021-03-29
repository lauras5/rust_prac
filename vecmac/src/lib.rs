#[macro_export]
macro_rules! avec {
    ($($elem: expr),*) => {{ // expr basically anything you can terminate w/ semicolon
        // check count is const
        const C: usize = $crate::count![@COUNT; $($elem),*];

        #[allow(unused_mut)]
        let mut vs = Vec::with_capacity(C); // known at compile
        $(vs.push($elem);)*
        vs
    }};
    ($($elem: expr,)*) => {{
        $crate::avec![$($elem),*]
    }};
    ($elem: expr; $count:expr) => {{
        let mut vs = Vec::new();
        // vs.extend(std::iter::repeat($elem).take(count)); // repeat yields clones for elem passed
        vs.resize($count, $elem); // more efficient
        vs
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! count {
    (@COUNT; $($elem: expr),*) => {
        <[()]>::len(&[$($crate::count![@SUBST; $elem]),*])
    };
    (@SUBST; $($_elem: expr),*) => {()}; // return unit not expr
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
    let _ : Vec<&'static str> = avec![
        "lakseolfkalskmdfoawijsdfl;knasodlfjasldfn",
        "lakseolfkalskmdfoawijsdfl;knasodlfjasldfn",
        "lakseolfkalskmdfoawijsdfl;knasodlfjasldfn",
        "lakseolfkalskmdfoawijsdfl;knasodlfjasldfn",
        "lakseolfkalskmdfoawijsdfl;knasodlfjasldfn",
    ];
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
