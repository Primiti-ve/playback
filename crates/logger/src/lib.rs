#![allow(unused)]

use std::{
    fmt,
    sync::atomic::{AtomicU8, Ordering},
};

static LOG_LEVEL: AtomicU8 = AtomicU8::new(Level::Info as u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    Error = 0,
    Warn = 1,
    Info = 2,
    Debug = 3,
}

impl Level {
    pub fn as_str(self) -> &'static str {
        match self {
            Level::Error => "ERROR",
            Level::Warn => "WARN",
            Level::Info => "INFO",
            Level::Debug => "DEBUG",
        }
    }

    pub fn colour(self) -> &'static str {
        match self {
            Level::Error => "\x1b[31m",
            Level::Warn => "\x1b[33m",
            Level::Info => "\x1b[32m",
            Level::Debug => "\x1b[36m",
        }
    }
}

pub fn init(level: Level) {
    LOG_LEVEL.store(level as u8, Ordering::Relaxed);
}

pub fn enabled(level: Level) -> bool {
    level as u8 <= LOG_LEVEL.load(Ordering::Relaxed)
}

pub fn log(level: Level, target: &str, args: fmt::Arguments) {
    if !enabled(level) {
        return;
    }

    let reset = "\x1b[0m";
    let dim = "\x1b[90m";
    
    let colour = level.colour();

    let level_str = format!("{:<5}", level.as_str());

    let prefix = format!("[{colour}{level_str}{reset} {dim}{target}{reset}]");
    let line = format!("{prefix} {args}");

    match level {
        Level::Error | Level::Warn => {
            eprintln!("{line}");
        }

        Level::Info | Level::Debug => {
            println!("{line}");
        }
    }
}

#[macro_export]
macro_rules! error {
    ($target:expr, $fmt:literal $(, $arg:expr)* $(,)?) => {
        $crate::log(
            $crate::Level::Error,
            $target,
            format_args!($fmt $(, $arg)*)
        )
    };
}

#[macro_export]
macro_rules! warn {
    ($target:expr, $fmt:literal $(, $arg:expr)* $(,)?) => {
        $crate::log(
            $crate::Level::Warn,
            $target,
            format_args!($fmt $(, $arg)*)
        )
    };
}

#[macro_export]
macro_rules! info {
    ($target:expr, $fmt:literal $(, $arg:expr)* $(,)?) => {
        $crate::log(
            $crate::Level::Info,
            $target,
            format_args!($fmt $(, $arg)*)
        )
    };
}

#[macro_export]
macro_rules! debug {
    ($target:expr, $fmt:literal $(, $arg:expr)* $(,)?) => {
        $crate::log(
            $crate::Level::Debug,
            $target,
            format_args!($fmt $(, $arg)*)
        )
    };
}
