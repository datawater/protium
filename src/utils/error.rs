#[macro_export]
macro_rules! pt_error {
    ($fmt:expr) => {
        {
            eprintln!("[ERROR] {}", $fmt);
            std::process::abort();
        }
    };

    ($fmt:expr, $($arg:expr),*) => {
        {
            eprintln!("[ERROR] {}", format!($fmt, $($arg),*));
            std::process::abort();
        }
    }
}