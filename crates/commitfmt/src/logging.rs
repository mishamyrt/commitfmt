// Colorized wrappers for logging

use fern::Dispatch;

#[macro_export]
macro_rules! print_error {
    ($($arg:tt)*) => {
        log::info!("{}", format!($($arg)*).bright_red());
    }
}

#[macro_export]
macro_rules! print_info {
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

pub(crate) fn pluralize(count: usize, singular: &str, plural: &str) -> String {
    if count == 1 {
        singular.to_string()
    } else {
        plural.to_string()
    }
}

pub(crate) fn setup_logger(verbose: bool, no_color: bool) {
    let log_level = if verbose { log::LevelFilter::Debug } else { log::LevelFilter::Info };
    Dispatch::new()
        .level(log_level)
        .chain(std::io::stdout())
        .apply()
        .expect("Unable to set up logger");

    if no_color {
        colored::control::set_override(false);
    }
}
