use std::sync::OnceLock;

use colored::Colorize;

pub static DEBUG: OnceLock<bool> = OnceLock::new();
pub static QUIET: OnceLock<bool> = OnceLock::new();

pub enum LogLevel {
    Debug,
    Info,
    Error,
}

impl LogLevel {
    pub fn to_str(&self) -> &'static str {
        match self {
            LogLevel::Debug => "[DEBUG]",
            LogLevel::Info => "[INFO ]",
            LogLevel::Error => "[ERROR]",
        }
    }
}

pub fn log(level: LogLevel, msg: String) {
    if *QUIET.get().unwrap_or(&false) {
        return;
    }

    match level {
        LogLevel::Debug => {
            if *DEBUG.get().unwrap_or(&false) {
                println!("{} {}", level.to_str().green(), msg);
            }
        }
        LogLevel::Info => println!("{} {}", level.to_str().blue(), msg),
        LogLevel::Error => println!("{} {}", level.to_str().red(), msg),
    }
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::log::log($crate::log::LogLevel::Debug, format!($($arg)*))
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::log::log($crate::log::LogLevel::Info, format!($($arg)*))
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::log::log($crate::log::LogLevel::Error, format!($($arg)*))
    };
}
