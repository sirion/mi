use image::GenericImageView;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
// use crate::debug;

const RESOLUTION_MIN: Resolution = Resolution {
	width: 150,
	height: 150,
};
// const RESOLUTION_QFHD:Resolution = Resolution{ width: 960, height: 540 };
// const RESOLUTION_FHD:Resolution = Resolution{ width: 1920, height: 1080 };
// const RESOLUTION_QHD:Resolution = Resolution{ width: 2560, height: 1440 };
// const RESOLUTION_UHD:Resolution = Resolution{ width: 3840, height: 2160 };

/// Resize using the Lanczos filter with a window of 3
pub const RESIZE_METHOD_LANCZOS3: &str = "lanczos3";
/// Resize using the Gaussian filter
pub const RESIZE_METHOD_GAUSSIAN: &str = "gaussian";
/// Resize using nearest neighbor method
pub const RESIZE_METHOD_NEAREST: &str = "nearest";
/// Resize uding a cubic filter
pub const RESIZE_METHOD_CUBIC: &str = "cubic";
/// Resize using a linear filter
pub const RESIZE_METHOD_LINEAR: &str = "linear";

/// Resize the image at the given source path to fit into the given resolution, keeping its aspect ratio, using the
/// given method and save it with the given quality as a jpg file under the target path
pub fn resize_jpg(
	source: &PathBuf,
	target: &PathBuf,
	resolution: Resolution,
	quality: u8,
	method: &String,
) {
	let method = match method.as_str() {
		RESIZE_METHOD_LANCZOS3 => image::imageops::FilterType::Lanczos3,
		RESIZE_METHOD_GAUSSIAN => image::imageops::FilterType::Gaussian,
		RESIZE_METHOD_NEAREST => image::imageops::FilterType::Nearest,
		RESIZE_METHOD_CUBIC => image::imageops::FilterType::CatmullRom,
		RESIZE_METHOD_LINEAR => image::imageops::FilterType::Triangle,
		_ => panic!("Invalid resize method: {}", method),
	};

	let image = image::open(source).unwrap();

	let ratio = image.width() as f64 / image.height() as f64;
	let mut width = resolution.width as f64;
	let mut height = resolution.height as f64;

	if width / ratio > resolution.height as f64 {
		height = width / ratio;
	} else {
		width = height / ratio;
	}

	let new_image = image::imageops::resize(&image, width as u32, height as u32, method);

	let mut out = std::fs::File::create(target).unwrap();
	let mut enc = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut out, quality);
	enc.encode_image(&new_image).unwrap();
}

/// Recode the image at the given source path, recode it with the given quality and save it as jpg under the given
/// target path
pub fn recode_jpg(source: &PathBuf, target: &PathBuf, quality: u8) {
	let image = image::open(source).unwrap();

	let mut out = std::fs::File::create(target).unwrap();
	let mut enc = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut out, quality);
	enc.encode_image(&image).unwrap();
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
/// Width and height grouped together
pub struct Resolution {
	/// Width
	pub width: u32,
	/// Height
	pub height: u32,
}

impl std::str::FromStr for Resolution {
	type Err = String;

	fn from_str(s: &str) -> Result<Resolution, String> {
		let split: Vec<&str> = s.split('x').collect();
		if split.len() == 2 {
			// eprintln!("Res Input: {}, {}", split[0], split[1]);

			let width: u32 = match split[0].trim().parse() {
				Ok(n) => n,
				Err(_) => {
					return Err(String::from("Invalid Resolution"));
				}
			};
			let height: u32 = match split[1].trim().parse() {
				Ok(n) => n,
				Err(_) => {
					return Err(String::from("Invalid Resolution"));
				}
			};

			if width < RESOLUTION_MIN.width || height < RESOLUTION_MIN.height {
				return Err(String::from("Resolution to low"));
			}

			Ok(Resolution { width, height })
		} else {
			Err(String::from("Invalid Resolution, must be in format WxH"))
		}
	}
}
