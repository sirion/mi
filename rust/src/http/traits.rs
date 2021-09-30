/// A combination of matcher and handler function for [Request]s. A standard implementation is available with [Handler].
pub trait RequestHandler: Sync + Send {
	/// Returns true if the handling method whould be called for the given request
	fn matches(&self, req: &super::Request) -> bool;

	/// Is called for the given request if it is the first Requesthandler that matches the request
	fn handle(&self, req: &super::Request, res: super::Response);
}
