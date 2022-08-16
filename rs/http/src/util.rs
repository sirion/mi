/// Carriage Return byte (\r)
pub const CR: u8 = '\r' as u8;
/// Line Feed byte (\n)
pub const LF: u8 = '\n' as u8;
/// Space byte
pub const SP: u8 = ' ' as u8;
/// CR + LF (\r\n)
pub const CRLF: [u8; 2] = [CR, LF];

pub const DEFAULT_HANDLER: fn(req: &super::Request, res: super::Response) =
	|req: &super::Request, mut res: super::Response| {
		res.status_code = 404;
		res.headers.set("Content-Type", "text/html");
		let _ = res.write("Not found: ");
		let _ = res.write(&req.uri);
		match res.end() {
			Ok(_) => {}
			Err(e) => log_err(format!("Error writing to response in default handler: {}", e)),
		};
	};

pub fn to_lines(data: &[u8]) -> Vec<Vec<u8>> {
	let mut lines: Vec<Vec<u8>> = Vec::new();
	let mut current_line = Vec::new();
	for i in 0..data.len() {
		if data[i] == LF {
			if i > 0 && data[i - 1] == CR {
				current_line.pop();
			}
			lines.push(current_line);
			current_line = Vec::new();
			continue;
		}
		current_line.push(data[i]);
	}

	lines
}

pub fn index_of(data: &Vec<u8>, entry: u8) -> Option<usize> {
	for i in 0..data.len() {
		if data[i] == entry {
			return Some(i);
		}
	}
	None
}

/// Returns the status string for the given HTTP status code. If given an unknown code "Unknown" is returned.
pub fn lookup_status_str(code: u16) -> &'static str {
	match code {
		100 => "Continue",
		101 => "Switching Protocols",
		102 => "Processing",
		200 => "OK",
		201 => "Created",
		202 => "Accepted",
		203 => "Non-authoritative Information",
		204 => "No Content",
		205 => "Reset Content",
		206 => "Partial Content",
		207 => "Multi-Status",
		208 => "Already Reported",
		226 => "IM Used",
		300 => "Multiple Choices",
		301 => "Moved Permanently",
		302 => "Found",
		303 => "See Other",
		304 => "Not Modified",
		305 => "Use Proxy",
		307 => "Temporary Redirect",
		308 => "Permanent Redirect",
		400 => "Bad Request",
		401 => "Unauthorized",
		402 => "Payment Required",
		403 => "Forbidden",
		404 => "Not Found",
		405 => "Method Not Allowed",
		406 => "Not Acceptable",
		407 => "Proxy Authentication Required",
		408 => "Request Timeout",
		409 => "Conflict",
		410 => "Gone",
		411 => "Length Required",
		412 => "Precondition Failed",
		413 => "Payload Too Large",
		414 => "Request-URI Too Long",
		415 => "Unsupported Media Type",
		416 => "Requested Range Not Satisfiable",
		417 => "Expectation Failed",
		418 => "I'm a teapot",
		421 => "Misdirected Request",
		422 => "Unprocessable Entity",
		423 => "Locked",
		424 => "Failed Dependency",
		426 => "Upgrade Required",
		428 => "Precondition Required",
		429 => "Too Many Requests",
		431 => "Request Header Fields Too Large",
		444 => "Connection Closed Without Response",
		451 => "Unavailable For Legal Reasons",
		499 => "Client Closed Request",
		500 => "Internal Server Error",
		501 => "Not Implemented",
		502 => "Bad Gateway",
		503 => "Service Unavailable",
		504 => "Gateway Timeout",
		505 => "HTTP Version Not Supported",
		506 => "Variant Also Negotiates",
		507 => "Insufficient Storage",
		508 => "Loop Detected",
		510 => "Not Extended",
		511 => "Network Authentication Required",
		599 => "Network Connect Timeout Error",
		_ => "Unknown",
	}
}

pub fn unix_time() -> u64 {
	std::time::SystemTime::now()
		.duration_since(std::time::UNIX_EPOCH)
		.unwrap()
		.as_secs()
}

use std::io::Write;
pub fn log_err<S: AsRef<str>>(data: S) {
	let _ = write!(std::io::stderr().lock(), "{} {}\n", unix_time(), data.as_ref());
}


pub fn log<S: AsRef<str>>(
	w: &std::sync::Arc<std::sync::Mutex<dyn Write + Send>>,
	data: S,
) {
	let guard = w.as_ref().lock();
	match guard {
		Ok(mut g) => {
			let _ = write!(g, "{} {}\n", unix_time(), data.as_ref());
		}
		Err(_) => {}
	}
}
