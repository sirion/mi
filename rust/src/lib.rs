#![warn(
	missing_docs,
	trivial_casts,
	trivial_numeric_casts,
	unsafe_code,
	unstable_features,
	unused_import_braces,
	unused_qualifications
)]

//! My personal reuse library

// mod json;

// TODO: Get rid of unwrap() calls in the code

/// Utilities for manipulating binary data
pub mod bin;

/// Utilities to work with the file system
pub mod fs;

/// HTTP server and utilities
pub mod http;

/// Utilities for image manipulation
pub mod img;

/// Logging
pub mod logger;
