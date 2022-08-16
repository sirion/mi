use super::util::lookup_status_str;
use super::util::CRLF;
use super::ValuesMap;
use std::io::prelude::*;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

/// Outgoing response to an incoming [super::Request]
/// TODO: Chunked encoding is currently not supported
pub struct Response {
	/// The headers to send to the client. By default [super::Request] headers don't perform any case handling
	pub headers: ValuesMap,
	/// The
	pub status_code: u16,
	/// The status string to send along the status code
	pub status: &'static str,

	stream: TcpStream,
	body: Vec<u8>,
	header_sent: bool,
	closed: bool,

	log_error: Arc<Mutex<dyn Write + Send>>,

	request_method: String,
	request_uri: String,
}

impl Response {
	/// Creates a new Response that writes to the given TCPStream
	pub fn new_for(
		stream: TcpStream,
		req: &super::Request,
		log_error: Arc<Mutex<dyn Write + Send>>,
	) -> Response {
		Response {
			stream,
			request_method: req.method.clone(),
			request_uri: req.uri.clone(),
			headers: ValuesMap::new(),
			body: Vec::new(),
			status: "",
			status_code: 200,
			closed: false,
			header_sent: false,
			log_error,
		}
	}

	/// Convenience method to allow writing and ignoring the result
	pub fn w<S: AsRef<[u8]>>(&mut self, data: S) {
		let _ = self.write(data);
	}

	/// Used by the write! macro
	pub fn write_fmt(&mut self, args: std::fmt::Arguments<'_>) -> Result<(), std::io::Error> {
		self.write(format!("{}", args))?;
		Ok(())
	}

	/// Write data into the [Response] body
	pub fn write<S: AsRef<[u8]>>(&mut self, data: S) -> Result<usize, std::io::Error> {
		if self.closed {
			super::util::log(
				&self.log_error,
				format!(
					"Write to closed connection ignored for {} {}",
					self.request_method, self.request_uri
				),
			);
			return Err(std::io::Error::new(
				std::io::ErrorKind::NotConnected,
				"Connection closed",
			));
		}

		self.body.extend_from_slice(data.as_ref());
		Ok(data.as_ref().len())
	}

	/// Clears the currently buffered body content that was adde since the last send
	pub fn clear(&mut self) {
		self.body.clear();
	}

	/// Sends the headers and all currently available data in the body to the client without closing the connection.
	pub fn send(&mut self) -> Result<(), std::io::Error> {
		if !self.header_sent {
			self.send_headers()?;
		}

		self.stream.write(&self.body)?;
		self.body.clear();

		Ok(())
	}

	fn send_headers(&mut self) -> Result<(), std::io::Error> {
		let mut head: Vec<u8> = Vec::new();
		head.extend("HTTP/1.1 ".as_bytes());
		head.extend(format!("{} ", self.status_code).as_bytes());

		if self.status.len() == 0 {
			// Use Default status string if none is set
			self.status = lookup_status_str(self.status_code);
		}

		if self.status_code >= 400 {
			// Log errors
			super::util::log(
				&self.log_error,
				format!(
					"Error {} {}, for {} {}",
					self.status_code, self.status, self.request_method, self.request_uri
				),
			);
		}

		head.extend(self.status.as_bytes());
		head.extend(CRLF);

		for (k, vs) in self.headers.all() {
			for v in vs {
				head.extend(k.as_bytes());
				head.extend(": ".as_bytes());
				head.extend(v.as_bytes());
				head.extend(&CRLF);
			}
		}

		self.stream.write(&head)?;
		self.stream.write(&CRLF)?;

		self.stream.write(&self.body)?;
		self.body.clear();

		Ok(())
	}

	/// End the response and ignore the result. like w() for write()
	pub fn e(&mut self) {
		let _ = self.end();
	}

	/// Send all remaining data and closes the connection.
	pub fn end(&mut self) -> Result<(), std::io::Error> {
		if self.closed {
			super::util::log(
				&self.log_error,
				format!(
					"Write to closed connection for {} {}",
					self.request_method, self.request_uri
				),
			);
			return Err(std::io::Error::new(
				std::io::ErrorKind::NotConnected,
				"Connection closed",
			));
		}
		self.closed = true;

		if !self.header_sent {
			// If we did not send the header before, we now know the length of the body
			self.headers
				.set("Content-Length", format!("{}", self.body.len()).as_str());
			self.send_headers()?;
		}

		self.stream.write(&self.body)?;

		self.stream.flush()?;
		self.stream.shutdown(std::net::Shutdown::Both)?;

		Ok(())
	}
}

impl Drop for Response {
	/// Automatically end the response when is is dropped
	fn drop(&mut self) {
		if !self.closed {
			match self.end() {
				Err(e) => {
					let mut g = self.log_error.lock().unwrap();
					let _ = write!(
						g,
						"Auto ending request on drop failed for {}: {}",
						self.request_uri, e
					);
				}
				Ok(_) => {}
			}
		}
	}
}
