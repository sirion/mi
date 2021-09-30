// #![allow(dead_code)]

use lazy_static::lazy_static;
use std::io::Write;
use std::sync::Mutex;

/// Log debug messages in addition to infos, warnings and errors
pub const LOGLEVEL_DEBUG: u8 = 4;
/// Log infos, warnings and errors
pub const LOGLEVEL_INFO: u8 = 3;
/// Log warnings and errors
pub const LOGLEVEL_WARN: u8 = 2;
/// Only log errors
pub const LOGLEVEL_ERROR: u8 = 1;
// pub const LOGLEVEL_SILENT: u8 = 0;

lazy_static! {
	static ref _MUTEX: Mutex<i32> = Mutex::new(0);
	static ref LOG_LEVEL: Mutex<u8> = Mutex::new(LOGLEVEL_ERROR);
}

/// Set the global log level
pub fn set_level(level: u8) {
	let mut log_level = LOG_LEVEL.lock().unwrap();
	*log_level = level;
}

/// Returns the global log level
pub fn get_level() -> u8 {
	*LOG_LEVEL.lock().unwrap()
}

fn get_prefix(level: u8) -> &'static str {
	match level {
		LOGLEVEL_ERROR => "[ Error ] ",
		LOGLEVEL_WARN => "[Warning] ",
		LOGLEVEL_INFO => "[ Info  ] ",
		LOGLEVEL_DEBUG => "[ Debug ] ",
		_ => "",
	}
}

/// Print a prefixed debug message followed by a newline if the log level is high enough
pub fn debugln<S: AsRef<str>>(message: S) {
	logln(LOGLEVEL_DEBUG, message);
}
/// Print a prefixed debug message if the log level is high enough
pub fn debug<S: AsRef<str>>(message: S) {
	log(LOGLEVEL_DEBUG, message);
}

/// Print a prefixed info message followed by a newline if the log level is high enough
pub fn infoln<S: AsRef<str>>(message: S) {
	logln(LOGLEVEL_INFO, message);
}
/// Print a prefixed info message if the log level is high enough
pub fn info<S: AsRef<str>>(message: S) {
	log(LOGLEVEL_INFO, message);
}

/// Print a prefixed warning message followed by a newline if the log level is high enough
pub fn warnln<S: AsRef<str>>(message: S) {
	logln(LOGLEVEL_WARN, message);
}
/// Print a prefixed warning message if the log level is high enough
pub fn warn<S: AsRef<str>>(message: S) {
	log(LOGLEVEL_WARN, message);
}

/// Print a prefixed error message followed by a newline to stderr
pub fn errorln<S: AsRef<str>>(message: S) {
	logln(LOGLEVEL_ERROR, message);
}
/// Print a prefixed error message to stderr
pub fn error<S: AsRef<str>>(message: S) {
	log(LOGLEVEL_ERROR, message);
}

/// Print a prefixed error message based on the given level (errors are logged to stderr)
pub fn log<S: AsRef<str>>(level: u8, message: S) {
	_log(true, false, level, message);
}

/// Print a prefixed error message followed by a newline based on the given level (errors are logged to stderr)
pub fn logln<S: AsRef<str>>(level: u8, message: S) {
	_log(true, true, level, message);
}

/// Print an unprefixed error message based on the given level (errors are logged to stderr)
pub fn ulog<S: AsRef<str>>(level: u8, message: S) {
	_log(false, false, level, message);
}

/// Print an unprefixed error message followed by a newline based on the given level (errors are logged to stderr)
pub fn ulogln<S: AsRef<str>>(level: u8, message: S) {
	_log(false, true, level, message);
}

fn _log<S: AsRef<str>>(prefixed: bool, ln: bool, level: u8, message: S) {
	if level > 0 && level <= LOGLEVEL_DEBUG && get_level() >= level {
		let _guard = _MUTEX.lock();
		let prefix = match prefixed {
			true => get_prefix(level),
			false => "",
		};

		if level == LOGLEVEL_ERROR {
			if ln {
				eprintln!("{}{}", prefix, message.as_ref());
			} else {
				eprint!("{}{}", prefix, message.as_ref());
			}
			std::io::stderr().flush().ok();
		} else {
			if ln {
				println!("{}{}", prefix, message.as_ref());
			} else {
				print!("{}{}", prefix, message.as_ref());
			}
			std::io::stdout().flush().ok();
		}
	}
}

#[macro_export]
/// Print a prefixed error message followed by a newline to stderr
macro_rules! log_error {
	($($arg:tt)*) => ($crate::logger::logln($crate::logger::LOGLEVEL_ERROR, format!($($arg)*)));
}

#[macro_export]
/// Print a prefixed error message followed by a newline if the log level is high enough
macro_rules! log_warn {
	($($arg:tt)*) => ($crate::logger::logln($crate::logger::LOGLEVEL_WARN, format!($($arg)*)));
}

#[macro_export]
/// Print a prefixed error message followed by a newline if the log level is high enough
macro_rules! log_info {
	($($arg:tt)*) => ($crate::logger::logln($crate::logger::LOGLEVEL_INFO, format!($($arg)*)));
}

#[macro_export]
/// Print a prefixed error message followed by a newline if the log level is high enough
macro_rules! log_debug {
	($($arg:tt)*) => ($crate::logger::logln($crate::logger::LOGLEVEL_DEBUG, format!($($arg)*)));
}
