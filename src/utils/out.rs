#[macro_export]
macro_rules! cli_stdout_printline {
    () => {
        std::print!("\n")
    };
    ($($arg:tt)*) => {{
        // let str = format!($($arg)*);
        // let mut log = paris::Logger::new();
        // log.info(str);
        println!($($arg)*);
    }};
}

#[macro_export]
macro_rules! cli_stderr_printline {
    () => {
        std::eprint!("\n")
    };
    ($($arg:tt)*) => {{
        eprintln!($($arg)*);
    }};
}
