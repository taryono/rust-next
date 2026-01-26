// src/macros/debug.rs
#[macro_export]
macro_rules! dd {
    () => {
        panic!("Dump and die at {}:{}", file!(), line!());
    };
    ($val:expr) => {
        panic!("{} = {:#?}\nat {}:{}", stringify!($val), $val, file!(), line!());
    };
    ($($val:expr),+ $(,)?) => {
        $(
            eprintln!("{} = {:#?}", stringify!($val), $val);
        )+
        panic!("Dump and die at {}:{}", file!(), line!());
    };
}
