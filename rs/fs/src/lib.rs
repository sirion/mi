use std::path::{Path, PathBuf};
use std::io::Write;

/// Copies a source to a target path. If source is a directory
pub fn copy_recursively(from: &PathBuf, to: &PathBuf) -> std::io::Result<()> {
	if from.is_file() {
		match std::fs::copy(from, to) {
			Ok(_) => Ok(()),
			Err(e) => Err(e),
		}
	} else if from.is_dir() {
		let entries = match std::fs::read_dir(from) {
			Ok(f) => f,
			Err(e) => {
				panic!("{}", e);
			}
		};

		if !to.is_dir() {
			std::fs::create_dir_all(to)?;
		}

		for entry in entries {
			let path = match entry {
				Ok(f) => f.path(),
				Err(e) => {
					eprintln!(
						"Warning: Could not copy entry in dir {}: {}",
						from.to_string_lossy(),
						e
					);
					continue;
				}
			};

			let target_filename = match path.file_name() {
				Some(f) => f,
				None => {
					eprintln!("Warning: Could not copy {}.", path.to_string_lossy());
					continue;
				}
			};

			copy_recursively(&path, &to.join(target_filename))?;
		}

		Ok(())
	} else {
		Err(std::io::Error::new(
			std::io::ErrorKind::InvalidInput,
			format!(
				"Unsupported type of source for copying: {}",
				from.to_string_lossy()
			),
		))
	}
}

/// Reads a directory into a vector of [std::path::PathBuf]. Always returns a Vec<PathBuf>, skipping all entries that
/// cannot be read, returning an empty one if the dir is not a directory.
pub fn list_dir(dir: &Path) -> Vec<PathBuf> {
	let mut files: Vec<PathBuf> = Vec::new();

	if !dir.is_dir() {
		let _ = write!(std::io::stderr().lock(), "Not a directory: {}", dir.to_string_lossy());
		return files;
	};

	// Validate input folders can be accessed
	let entries = match std::fs::read_dir(dir) {
		Ok(f) => f,
		Err(_) => {
			return files;
		}
	};

	for file in entries {
		// let mut path = dir.clone();
		match file {
			Ok(f) => {
				files.push(f.path());
			}
			Err(e) => {
				let _ = write!(std::io::stderr().lock(), "Could not read file: {}", e);
				continue;
			}
		};
	}

	files
}

/// Creates a [String] that can be safely used as a file name. Turns the str to lowercase, replaces whitespace
/// characters with "-" and everything else that is not alphanumeric with "_".
///
/// # Example
///
/// ```
/// use mi::fs::sanitize;
/// assert_eq!(&sanitize("/ect/passwd"), "_ect_passwd");
/// assert_eq!(&sanitize("Oh no!🔥!"), "oh-no___");
///
/// ```
pub fn sanitize(s: &str) -> String {
	let mut cleaned = String::new();

	for mut c in s.trim().chars() {
		c = c.to_ascii_lowercase();
		if c == '.' || c == '_' {
			// Keep c as is
		} else if c.is_whitespace() {
			c = '-';
		} else if !c.is_ascii_alphanumeric() {
			c = '_';
		}
		cleaned.push(c);
	}

	//log_debug!("Sanitized: \"{}\" => \"{}\"", s, cleaned);
	cleaned
}

