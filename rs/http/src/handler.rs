/// A combination of matcher and handler function

use std::sync::{Arc, Mutex};

pub struct Handler {
	/// Returns true if the handling method whould be called for the given request
	matcher_fn: Arc<Mutex<dyn FnMut(&super::Request) -> bool + Send + Sync>>,

	/// Is called for the given request if it is the first handler that matches the request
	handler_fn: Arc<Mutex<dyn FnMut(&super::Request, super::Response) + Send + Sync>>,
}

impl Handler {
	/// Ceeate a new Handler from a matcher function and a handler function
	pub fn new(
		matcher_fn: Arc<Mutex<dyn FnMut(&super::Request) -> bool + Send + Sync>>,
		handler_fn: Arc<Mutex<dyn FnMut(&super::Request, super::Response) + Send + Sync>>,
	) -> Handler {
		Handler {
			matcher_fn: matcher_fn.clone(),
			handler_fn: handler_fn.clone(),
		}
	}
}

impl super::RequestHandler for Handler {
	fn matches(&self, req: &super::Request) -> bool {
		(self.matcher_fn.lock().unwrap())(req)
	}

	fn handle(&self, req: &super::Request, res: super::Response) {
		(self.handler_fn.lock().unwrap())(req, res)
	}
}
