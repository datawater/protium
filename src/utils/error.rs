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

#[macro_export]
macro_rules! nd_todo {
    () => {
        eprintln!("[TODO] Unimplemented! {}:{}", file!(), line!());
    };

    ($fmt:expr) => {
        eprintln!("[TODO] {}. {}:{}", $fmt, file!(), line!());
    };

    ($fmt:expr, $($arg:expr),*) => {
        eprintln!("[TODO] {}. {}:{}", format!($fmt, $($arg),*), file!(), line!());
    };
}