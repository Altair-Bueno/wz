macro_rules! include_resources {
    ($($x:tt), *) => {
        $(
            const $x :&[u8] = include_bytes!(concat!("resources/", stringify!($x), ".txt"));
        )*
    };
}

//const empty: &[u8] = include_bytes!("resources/empty.txt");
include_resources!(empty, french, spanish, small, Lorem_big);
