// Public API

/// Container for HTTP method constants
pub mod methods;

// Modules for file management purposes
mod error;
mod filehandler;
mod handler;
mod headers;
mod request;
mod response;
mod server;
mod traits;

pub use error::Error;
pub use filehandler::FileHandler;
pub use handler::Handler;
pub use headers::Headers;
pub use request::Request;
pub use response::Response;
pub use server::Server;
pub use traits::RequestHandler;

// Private API

mod util;
