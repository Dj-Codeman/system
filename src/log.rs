use std::{fmt, sync::RwLock};

use colored::Colorize;
use serde::{Deserialize, Serialize};

lazy_static::lazy_static! {
    static ref CURRENT_LOG_LEVEL: RwLock<LogLevel> = RwLock::new(LogLevel::Info);
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Eq, Ord, Serialize, Deserialize)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

#[macro_export]
macro_rules! log {
    ($level:expr, $($arg:tt)*) => {
        {
            let current_level = $crate::logger::get_log_level();
            if $level <= current_level {
                println!("[{}]: {}", $level, format!($($arg)*));
            }
        }
    };
}

pub fn get_log_level() -> LogLevel {
    match CURRENT_LOG_LEVEL.read() {
        Ok(log_level_guard) => *log_level_guard,
        Err(_) => LogLevel::Trace,
    }
}

pub fn set_log_level(level: LogLevel) {
    let mut log_level = match CURRENT_LOG_LEVEL.write() {
        Ok(log_level_guard) => log_level_guard,
        Err(_) => unreachable!(),
    };
    *log_level = level;
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let log_str = match self {
            LogLevel::Error => "Error".bold().red(),
            LogLevel::Warn => "Warn".bold().yellow(),
            LogLevel::Info => "Info".bold().green(),
            LogLevel::Debug => "Debug".bold().blue(),
            LogLevel::Trace => "Trace".bold().magenta(),
        };
        write!(f, "{}", log_str)
    }
}
