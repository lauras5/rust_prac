#[macro_export]
macro_rules! avec {
    () => {
        Vec::new()      // doesnt need block
    };
    ($($elem: expr), + $(,)?) => {{ // expr basically anything you can terminate w/ semicolon
        let mut vs = Vec::new();
        $(vs.push($elem);)+
        vs
    }};
    ($elem: expr; $count:expr) => {{ // expr basically anything you can terminate w/ semicolon
        let mut vs = Vec::new();
        let x = $elem;
        for _ in 0..$count {
            vs.push(x.clone());
        }
        vs
    }};

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
    let mut x: Vec<u32> = avec![y.take().unwrap(); 2];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 42);
}