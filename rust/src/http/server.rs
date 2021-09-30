use super::util::log;
use crate::log_info;
use std::io::prelude::*;
use std::net::SocketAddr;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;

/// A simple HTTP Server. Dispatches incoming requests to the given handler functions based on their associated matchers.
///
/// # Examples
///
/// ```
/// use mi::http::*;
/// let mut server = Server::new();
///
/// server.handle(|r| r.uri == "/a", handle_a);
///
/// server.handle(|r| r.uri == "/b", |_, mut res: Response| {
/// 	res.headers.set("Content-Type", "text/html");
/// 	res.write("Hello B".as_bytes());
/// 	let _ = res.end();
/// });
///
/// fn handle_a(_: &Request, mut res: Response) {
/// 	res.headers.set("Content-Type", "text/html");
/// 	res.write("Hello A".as_bytes());
/// 	let _ = res.end();
/// }
/// ```
///
/// Alternatively, you can add a handler which implements the [RequestHandler] trait. The Default implementation is
/// [Handler] and will automatically be created when using handle().
///
/// ```
/// use mi::http::*;
/// use std::sync::Arc;
/// let mut server = Server::new();
///
/// // Tee following two lines do the same thing:
/// server.handle(|r| r.uri == "/a", |_, mut res: Response| { res.write(&[33, 33, 33]); res.end(); });
/// server.handler(Arc::new(Handler::new(|r| r.uri == "/a", |_, mut res: Response| { res.write(&[33, 33, 33]); res.end(); })));
/// ```
pub struct Server {
	/// The number of threads to use for listening to incoming connections
	pub num_threads: usize,
	/// Timeout duration for reading from incoming connections
	pub read_timeout: Option<std::time::Duration>,
	/// Writer to which to log read access. Defaults to ignored
	pub log_access: Arc<Mutex<dyn Write + Send>>,
	/// Writer to which to log errors. Defaults to stderr
	pub log_errors: Arc<Mutex<dyn Write + Send>>,
	running: bool,
	handlers: Vec<Arc<dyn super::RequestHandler>>,
}

impl Server {
	/// Creates a new server. Before adding handlers ir responds to all requests with the default 404 response
	pub fn new() -> Server {
		Server {
			num_threads: num_cpus::get(),
			read_timeout: Some(std::time::Duration::new(30, 0)),
			log_access: Arc::new(Mutex::new(std::io::sink())),
			log_errors: Arc::new(Mutex::new(std::io::stderr())),
			running: true,
			handlers: Vec::new(),
		}
	}

	/// Adds a handler function to the server along with a matcher function. A [RequestHandler] is created and then
	/// added via [Server.handler].
	/// The matcher function is used to check if it matches the request in the order they were added to the server.
	/// That means if the first matcher function matches everything, the other ones will never be called.
	///
	/// # Example
	///
	/// ```
	/// use mi::http::*;
	/// let mut server = Server::new();
	///
	/// server.handle(|r: &Request| r.uri == "/", |req, mut res| {
	/// 	res.headers.set("Content-Type", "text/html");
	/// 	res.write("Hello from ".as_bytes());
	/// 	res.write(req.uri.as_bytes());
	/// 	let _ = res.end();
	/// });
	/// ```
	///
	/// Example with handler that will never be called:
	///
	/// ```
	/// use mi::http::*;
	/// let mut server = Server::new();
	///
	/// server.handle(|_| true, |_, mut res| {
	/// 	res.write("All".as_bytes());
	/// 	res.end();
	/// });
	///
	/// server.handle(|_| true, |_, mut res| {
	/// 	res.write("Never called".as_bytes());
	/// 	res.end();
	/// });
	/// ```
	pub fn handle(
		&mut self,
		matcher_fn: fn(req: &super::Request) -> bool,
		handler_fn: fn(&super::Request, super::Response),
	) {
		self.handler(Arc::new(super::Handler::new(matcher_fn, handler_fn)));
	}

	/// Adds a [RequestHandler] to the server.
	pub fn handler(&mut self, handler: Arc<dyn super::RequestHandler>) {
		self.handlers.push(handler);
	}

	/// Opens the given port for listening to incoming connections. Returns an error if the port cannot be opened.
	pub fn listen(&mut self, port: u16) -> Result<(), Box<dyn std::error::Error>> {
		let pool = ThreadPool::new(self.num_threads);
		let listener = TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], port)))?;

		for st in listener.incoming() {
			let stream = match st {
				Ok(s) => s,
				Err(e) => {
					log(
						&self.log_errors,
						format!("Incoming connection error: {}", e),
					);
					continue;
				}
			};

			self.handle_connection(&pool, stream);

			if !self.running {
				log_info!("Stopping Server");
				break;
			}
		}

		pool.join();

		Ok(())
	}

	fn handle_connection(&mut self, pool: &ThreadPool, stream: TcpStream) {
		match stream.set_read_timeout(self.read_timeout) {
			Ok(()) => (),
			Err(e) => log(
				&self.log_errors,
				format!("Error setting read timeout: {}", e),
			),
		};

		let r = super::Request::from(stream);

		if !r.is_ok() {
			log(
				&self.log_errors,
				format!("x: Invalid Request: {}", r.err().unwrap()),
			);
			return;
		}

		let req = r.unwrap();

		log(&self.log_access, format!("{} {}", req.method, req.uri));

		// Find longest matching handler
		let mut matched_handler: Option<Arc<dyn super::RequestHandler>> = None;
		for i in 0..self.handlers.len() {
			let matched = self.handlers[i].matches(&req);
			if matched {
				matched_handler = Some(self.handlers[i].clone());
				break;
			}
		}

		let response_stream = match req.clone_stream() {
			Ok(s) => s,
			Err(e) => {
				log(
					&self.log_access,
					format!("Could not clone response stream: {}", e),
				);
				return;
			}
		};

		let res = super::Response::new_for(response_stream, &req, self.log_errors.clone());

		if matched_handler.is_some() {
			let handler = matched_handler.unwrap();
			pool.execute(move || handler.handle(&req, res));
		} else {
			super::util::DEFAULT_HANDLER(&req, res);
		}
	}
}
