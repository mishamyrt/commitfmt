// Colorized wrappers for logging

#[macro_export]
macro_rules! print_error {
    ($($arg:tt)*) => {
        log::info!("{}", format!($($arg)*).bright_red());
    }
}

#[macro_export]
macro_rules! print_stdout {
    ($($arg:tt)*) => {
        log::info!("{}", format!($($arg)*));
    }
}

#[macro_export]
macro_rules! print_debug {
    ($($arg:tt)*) => {
        log::debug!("{}", format!($($arg)*).dimmed());
    }
}

#[macro_export]
macro_rules! print_warning {
    ($($arg:tt)*) => {
        log::info!("{}", format!($($arg)*).bright_yellow());
    }
}
