// TODO: Can we make this a const fn?

#[macro_export]
macro_rules! hack {
    ($fmt:expr) => {
        ::std::format!($fmt).parse().unwrap()
    };
    ($fmt:expr, $($arg:tt)*) => {
        ::std::format!($fmt, $($arg)*).parse().unwrap()
    }
}
