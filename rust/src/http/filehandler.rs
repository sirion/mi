use crate::{log_error, log_info};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Default implementation for a simple file system based handler. Serves the files under the given root path for the
/// request uri removing the given uri prefix.
/// If list_dirs is set to true, simple directory index pages will be generated.
/// The mime types for files are guessed from their extension. A known list of extensions is available and can be
/// augmented by adding to the ext2mime HashMap.
pub struct FileHandler<'a> {
	/// Whether or not to generate directory listings
	pub list_dirs: bool,
	index: Vec<&'a str>,
	uri_prefix: &'a str,
	root: PathBuf,
	ext2mime: HashMap<&'a str, &'a str>,
}

impl<'a> FileHandler<'a> {
	/// Returns a new [super::RequestHandler] that serves files for the URLs that start with uri_prefix relative to the given
	/// root path.
	pub fn new<P: AsRef<Path>>(uri_prefix: &str, root: P) -> FileHandler {
		FileHandler {
			uri_prefix,
			root: PathBuf::from(root.as_ref()),
			list_dirs: false,
			index: vec!["index.html"],
			ext2mime: HashMap::new(),
		}
	}

	fn mimetype_for_extension<P: AsRef<Path>>(&self, path: P) -> String {
		let ext = match path.as_ref().extension() {
			None => String::from(""),
			Some(e) => {
				let low = e.to_ascii_lowercase();
				let ext = low.to_string_lossy().into_owned();
				String::from(ext)
			}
		};

		let ext = ext.as_str();
		String::from(match ext {
			// Default extensions
			"jpg" => "image/jpeg",
			"jpeg" => "image/jpeg",
			"png" => "image/png",
			"gif" => "image/gif",
			"html" => "text/html",
			"htm" => "text/html",
			"txt" => "text/plain",
			"css" => "text/css",
			"json" => "application/json",
			"js" => "application/javascript",
			".bz" => "application/x-bzip",
			".bz2" => "application/x-bzip2",
			".csv" => "text/csv",
			".eot" => "application/vnd.ms-fontobject",
			".epub" => "application/epub+zip",
			".gz" => "application/gzip",
			".gif" => "image/gif",
			".ico" => "image/vnd.microsoft.icon",
			".ics" => "text/calendar",
			".jar" => "application/java-archive",
			".mjs" => "text/javascript",
			".mp3" => "audio/mpeg",
			".mp4" => "video/mp4",
			".mpeg" => "video/mpeg",
			".odp" => "application/vnd.oasis.opendocument.presentation",
			".ods" => "application/vnd.oasis.opendocument.spreadsheet",
			".odt" => "application/vnd.oasis.opendocument.text",
			".oga" => "audio/ogg",
			".ogv" => "video/ogg",
			".ogx" => "application/ogg",
			".opus" => "audio/opus",
			".otf" => "font/otf",
			".png" => "image/png",
			".pdf" => "application/pdf			",
			".rar" => "application/vnd.rar",
			".rtf" => "application/rtf",
			".svg" => "image/svg+xml",
			".tif" => "image/tiff			",
			".ttf" => "font/ttf",
			".wav" => "audio/wav",
			".weba" => "audio/webm",
			".webm" => "video/webm",
			".webp" => "image/webp",
			".woff" => "font/woff",
			".woff2" => "font/woff2",
			".xhtml" => "application/xhtml+xml",
			".xml" => "application/xml			",
			".zip" => "application/zip",
			".7z" => "application/x-7z-compressed",

			// Synamic extensions
			_ => match self.ext2mime.get(ext) {
				Some(v) => v,
				None => "application/octet-stream",
			},
		})
	}

	fn list_dir(&self, mut res: super::Response, path: PathBuf) -> Result<(), std::io::Error> {
		let entries = match std::fs::read_dir(&path) {
			Ok(e) => e,
			Err(e) => {
				log_error!(
					"Internal Sever Error - Cannot list directory {}: {}",
					path.to_string_lossy(),
					e
				);
				return self.serve_error(res, 500, "Internal Server Error");
			}
		};

		res.headers.set("Content-Type", "text/html");

		res.write("<!DOCTYPE html><body><ul>")?;
		for entry in entries {
			let file = match entry {
				Ok(f) => f,
				Err(e) => {
					log_error!("Error reading directory index: {}", e);
					continue;
				}
			};

			let file_name = file.file_name();
			let name = match file_name.to_str() {
				Some(s) => s,
				None => {
					log_error!(
						"Error reading directory index: {}",
						file.path().to_string_lossy()
					);
					continue;
				}
			};

			log_info!(" - {}", name);

			res.write("<li>")?;
			res.write("<a href=\"")?;
			res.write(name.replace("\"", "\\\""))?;
			res.write("\">")?;
			res.write(name)?;
			res.write("</a>")?;
			res.write("</li>")?;
		}
		res.write("</ul></body>")?;
		//let _ = res.end();
		Ok(())
	}

	fn serve_error(
		&self,
		mut res: super::Response,
		code: u16,
		message: &str,
	) -> Result<(), std::io::Error> {
		res.headers.set("Content-Type", "text/plain");
		res.status_code = code;
		res.status = super::util::lookup_status_str(code);
		res.clear();
		res.write(message)?;
		res.end()
	}

	fn serve_file(&self, mut res: super::Response, path: PathBuf) -> Result<(), std::io::Error> {
		match std::fs::read(&path) {
			Ok(data) => {
				res.status_code = 200;
				// Find content type
				res.headers
					.set("Content-Type", &self.mimetype_for_extension(path));

				// TODO: Maybe think about ading a method for writing into the stream directly for performance reasons...?
				res.write(&data)?;

				return res.end();
			}
			Err(e) => {
				log_error!("Internal Sever Error: {}", e);
				return self.serve_error(res, 500, "Internal Server Error");
			}
		}
	}

	fn serve(
		&self,
		res: super::Response,
		path: PathBuf,
		uri_path: &str,
	) -> Result<(), std::io::Error> {
		if path.exists() {
			let is_dir = path.is_dir();
			let has_slash = uri_path.ends_with("/") || uri_path == "";

			if is_dir && has_slash {
				for f in &self.index {
					let path = path.join(f);
					if path.exists() {
						return self.serve(res, path, uri_path);
					}
				}

				if self.list_dirs {
					return self.list_dir(res, path);
				} else {
					return self.serve_error(res, 404, &format!("Not found: {}", uri_path));
				}
			} else if is_dir {
				return self.serve_error(res, 404, &format!("Not found: {}", uri_path));
			} else if path.is_file() {
				return self.serve_file(res, path);
			} else {
				log_error!(
					"Internal Sever Error - path is neither file nor directory: {}",
					path.to_string_lossy()
				);
				self.serve_error(res, 500, "Internal Server Error")
			}
		} else {
			return self.serve_error(res, 404, &format!("Not found: {}", uri_path));
		}
	}
}

impl<'a> super::RequestHandler for FileHandler<'a> {
	fn matches(&self, req: &super::Request) -> bool {
		req.uri.starts_with(self.uri_prefix)
	}

	fn handle(&self, req: &super::Request, res: super::Response) {
		let mut uri_path = &req.uri[self.uri_prefix.len()..];
		while uri_path.starts_with("/") && uri_path.len() > 0 {
			uri_path = &uri_path[1..];
		}

		let path = self.root.join(PathBuf::from(uri_path));
		let _ = self.serve(res, path, uri_path);
	}
}
