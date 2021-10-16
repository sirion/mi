/// A combination of matcher and handler function
pub struct Handler {
	/// Returns true if the handling method whould be called for the given request
	matcher_fn: fn(req: &super::Request) -> bool,

	/// Is called for the given request if it is the first handler that matches the request
	handler_fn: fn(req: &super::Request, res: super::Response),
}

impl Handler {
	/// Ceeate a new Handler from a matcher function and a handler function
	pub fn new(
		matcher_fn: fn(req: &super::Request) -> bool,
		handler_fn: fn(&super::Request, super::Response),
	) -> Handler {
		Handler {
			matcher_fn,
			handler_fn,
		}
	}
}

impl super::RequestHandler for Handler {
	fn matches(&self, req: &super::Request) -> bool {
		(self.matcher_fn)(req)
	}

	fn handle(&self, req: &super::Request, res: super::Response) {
		(self.handler_fn)(req, res)
	}
}
