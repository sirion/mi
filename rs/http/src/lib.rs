#![warn(
	missing_docs,
	trivial_casts,
	trivial_numeric_casts,
	unsafe_code,
	unstable_features,
	unused_import_braces,
	unused_qualifications
)]

// Public API

/// Container for HTTP method constants
pub mod methods;

// Modules for file management purposes
mod error;
mod filehandler;
mod handler;
mod request;
mod response;
mod server;
mod traits;
mod valuesmap;

// Public structs
pub use error::Error;
pub use filehandler::FileHandler;
pub use handler::Handler;
pub use request::Request;
pub use response::Response;
pub use server::Server;
pub use traits::RequestHandler;
pub use valuesmap::ValuesMap;

// Public functions
pub use util::lookup_status_str;

// Private API
mod util;



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
