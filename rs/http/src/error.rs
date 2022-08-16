use super::util::lookup_status_str;

#[derive(Debug)]
/// An HTTP error that can be converted into a response
pub struct Error {
	/// The HTTP status code
	pub code: u16,
	/// The HTTP status code name
	pub status: &'static str,
	/// The error message
	pub message: String,
}

impl Error {
	/// Returns an error with the given properties
	pub fn new<S: AsRef<str>>(code: u16, message: S) -> Error {
		Error {
			code,
			/// Returns the status string for the given HTTP status code. If given an unknown code the
			status: lookup_status_str(code),
			message: String::from(message.as_ref()),
		}
	}

	/// Returns a boxed error with the given properties
	pub fn boxed<S: AsRef<str>>(code: u16, message: S) -> Box<Error> {
		Box::new(Self::new(code, message))
	}
}

impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.message)
	}
}

impl std::error::Error for Error {}
