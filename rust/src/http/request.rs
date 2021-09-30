use super::util::{index_of, to_lines};
use super::util::{CR, LF, SP};
use super::Error;
use super::Headers;
use crate::bin::Find;
use std::io::prelude::*;
use std::net::TcpStream;

/// Incoming request
pub struct Request {
	/// Headers sent by the client
	pub headers: Headers,
	/// HTTP Method sent by the client
	pub method: String,
	/// Requested URI
	pub uri: String,
	/// HTTP Version string sent by the client
	pub http_version: String,

	stream: TcpStream,
	header_length: usize,
	body_length: usize,
	read_bytes: usize,
	body: Vec<u8>,
}

impl Request {
	/// Creates a new [Request] from an incoming stream
	pub fn from(stream: TcpStream) -> Result<Request, Box<dyn std::error::Error>> {
		let req = Request::parse_data(stream);

		if req.is_err() {
			return Err(Error::boxed(400, "Invalid request"));
		}

		let request = req?;

		Ok(request)
	}

	fn parse_data(mut stream: TcpStream) -> Result<Request, Box<dyn std::error::Error>> {
		let mut buffer = [0; 1024];
		let mut in_header = true;
		let mut header_bytes: Vec<u8> = Vec::with_capacity(1024);
		let mut body: Vec<u8> = Vec::with_capacity(1024);
		let mut header_length: usize = 0;
		let mut read_bytes: usize = 0;
		while in_header {
			let read = stream.read(&mut buffer)?;
			read_bytes += read;
			let data = &buffer[0..read];

			in_header = match data.index_of(&[CR, LF, CR, LF], 0) {
				Ok(p) => {
					header_bytes.extend(&data[0..p]);
					header_length = p + 4;

					if header_length < read {
						// Part of the body is already in the buffer
						body.extend(&data[header_length..]);
					}

					// End of header found
					false
				}
				Err(_) => match data.index_of(&[LF, LF], 0) {
					Ok(p) => {
						header_bytes.extend(&data[0..p]);
						header_length = p + 2;

						if header_length < read {
							// Part of the body is already in the buffer
							body.extend(&data[header_length..]);
						}

						// End of header found
						false
					}
					Err(_) => {
						header_bytes.extend(data);
						// Header end not yet found
						true
					}
				},
			};

			// TODO: Set maximum number of bytes to read when looking for the end of the header
		}

		let mut header_lines = to_lines(&header_bytes);

		let first_line = header_lines.remove(0);

		let mut headers = Headers::new();
		headers.case_handling = true;
		for line in header_lines {
			match index_of(&line, ':' as u8) {
				Some(i) => {
					let key = std::str::from_utf8(&line[0..i - 1]).unwrap().trim();
					let value = std::str::from_utf8(&line[i..]).unwrap().trim();
					headers.add(key, value);
				}
				None => panic!("Invalid Request"),
			}
		}

		// first_line should only be the request line
		let mut split = first_line
			.splitn(3, |c| c == &SP)
			.map(|s| String::from_utf8(Vec::from(s)).unwrap())
			.collect::<Vec<String>>();

		assert_eq!(split.len(), 3);
		let http_version = split.pop().unwrap();
		let uri = split.pop().unwrap();
		let method = split.pop().unwrap();

		// Find out if somethin from the
		let body_length = match headers.get("Content-Length") {
			Some(s) => s.parse().or::<usize>(Ok(0usize)).unwrap(),
			None => 0usize,
		};

		// TODO: What about trailers?

		Ok(Request {
			stream,
			method,
			uri,
			http_version,
			headers,
			header_length,
			body,
			read_bytes,
			body_length,
		})
	}

	/// Populates/Reads the request body and then returns a reference to it
	pub fn get_body(&mut self) -> Result<&Vec<u8>, Box<dyn std::error::Error>> {
		while self.body_length > self.read_bytes - self.header_length {
			// Not all data was read
			let mut buffer = [0; 1024];
			let read = self.stream.read(&mut buffer)?;

			self.body.extend_from_slice(&buffer[0..read]);
			self.read_bytes += read;
		}

		Ok(&self.body)
	}

	/// Returns a clone of the request TcpSstream or an error if cloning fails
	pub fn clone_stream(&self) -> Result<TcpStream, std::io::Error> {
		self.stream.try_clone()
	}
}
