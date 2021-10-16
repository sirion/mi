use super::util::{index_of, to_lines};
use super::util::{CR, LF, SP};
use super::Error;
use super::ValuesMap;
use crate::bin::Find;
use crate::log_error;
use std::io::prelude::*;
use std::net::TcpStream;

/// Incoming request
pub struct Request {
	/// Headers sent by the client
	pub headers: ValuesMap,
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
	query_parameters: ValuesMap,
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

		let mut headers = ValuesMap::new();
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

		// first_line must contain the request line
		let (method, uri, http_version) = split_request_line(&first_line);

		let query_parameters = uri_parameters(&uri);

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
			query_parameters,
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

	/// Returns the query parameters as a HashMap if string vectors
	pub fn get_query_parameters(&self) -> &ValuesMap {
		&self.query_parameters
	}
}

pub fn split_request_line(line: &[u8]) -> (String, String, String) {
	let mut split = line
		.splitn(3, |c| c == &SP)
		.map(|s| String::from_utf8(Vec::from(s)).unwrap())
		.collect::<Vec<String>>();

	assert_eq!(split.len(), 3);

	let http_version = split.pop().unwrap();
	let uri = split.pop().unwrap();
	let method = split.pop().unwrap();

	(method, uri, http_version)
}

fn uri_parameters(uri: &str) -> ValuesMap {
	let mut parameters = ValuesMap::new();

	if let Some(pos) = uri.find("?") {
		let (_, query_str) = uri.split_at(pos + 1);
		for pairs in query_str.split::<&str>("&") {
			let parts: Vec<&str> = pairs.splitn(2, "=").collect();
			if parts.len() == 2 {
				parameters.add(parts[0], parts[1]);
			} else if parts.len() == 1 {
				parameters.add(parts[0], "");
			} else {
				log_error!("Invalid query parameter: {}", pairs);
			}
		}
	}

	parameters
}
