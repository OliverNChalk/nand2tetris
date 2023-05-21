#[macro_export]
macro_rules! assign {
    ($text:literal) => {
        $text.parse().unwrap()
    };
}
