/*
* Copyright (c) 2024, Dr. Spandan Roy
*
* This file is part of iceforge.
*
* iceforge is free software: you can redistribute it and/or modify
* it under the terms of the GNU General Public License as published by
* the Free Software Foundation, either version 3 of the License, or
* (at your option) any later version.
*
* iceforge is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU General Public License for more details.
*
* You should have received a copy of the GNU General Public License
* along with iceforge.  If not, see <https://www.gnu.org/licenses/>.
*/

use colored::Colorize;

#[derive(Debug, PartialEq, Eq)]
pub enum LogLevel {
    Debug,
    Verbose,
    Info,
    Warning,
    Error,
}

pub fn log(level: LogLevel, msg: &str) {
    match level {
        LogLevel::Debug => println!("{} {}", "DEBUG: ".blue(), msg),
        LogLevel::Verbose => println!("{} {}", "VERBOSE: ".cyan(), msg),
        LogLevel::Info => println!("{} {}", "INFO: ".green(), msg),
        LogLevel::Warning => eprintln!("{} {}", "WARNING: ".yellow(), msg),
        LogLevel::Error => eprintln!("{} {}", "ERROR: ".red(), msg),
    }
}

#[macro_export]
macro_rules! logd {
    () => {
        println!();
    };
    ($msg:expr) => {
        $crate::logger::log($crate::logger::LogLevel::Debug, $msg)
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::logger::log($crate::logger::LogLevel::Debug, &format!($fmt, $($arg)*))
    };
}

#[macro_export]
macro_rules! logv {
    () => {
        println!();
    };
    ($msg:expr) => {
        $crate::logger::log($crate::logger::LogLevel::Verbose, $msg)
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::logger::log($crate::logger::LogLevel::Verbose, &format!($fmt, $($arg)*))
    };
}

#[macro_export]
macro_rules! logi {
    () => {
        println!();
    };
    ($msg:expr) => {
        $crate::logger::log($crate::logger::LogLevel::Info, $msg)
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::logger::log($crate::logger::LogLevel::Info, &format!($fmt, $($arg)*))
    };
}

#[macro_export]
macro_rules! logw {
    () => {
        eprintln!();
    };
    ($msg:expr) => {
        $crate::logger::log($crate::logger::LogLevel::Warning, $msg)
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::logger::log($crate::logger::LogLevel::Warning, &format!($fmt, $($arg)*))
    };
}

#[macro_export]
macro_rules! loge {
    () => {
        eprintln!();
    };
    ($msg:expr) => {
        $crate::logger::log($crate::logger::LogLevel::Error, $msg)
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::logger::log($crate::logger::LogLevel::Error, &format!($fmt, $($arg)*))
    };
}
