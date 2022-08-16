#![warn(
	missing_docs,
	trivial_casts,
	trivial_numeric_casts,
	unsafe_code,
	unstable_features,
	unused_import_braces,
	unused_qualifications
)]

use exif;
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

fn rotate_image(img: &image::DynamicImage) -> bool {
	let mut cursor = std::io::Cursor::new(img.as_bytes());

	// Find out if image is rotated
	// let file = std::fs::File::open(source)?;
	// let mut bufreader = std::io::BufReader::new(&file);
	let exifreader = exif::Reader::new();

	let rotated = match exifreader.read_from_container(&mut cursor) {
		Ok(exif) => match exif.get_field(exif::Tag::Orientation, exif::In::PRIMARY) {
			Some(orientation) => match orientation.value.get_uint(0) {
				Some(v @ 1..=8) => v,
				_ => 0,
			},
			None => 0,
		},
		Err(_) => 0,
	};

	// Returns true if sides have been switched
	match rotated {
		1 => false, // Keep image as is.
		2 => {
			// Mirrored
			img.fliph();
			false
		}
		3 => {
			// Rotated 180°
			img.rotate180();
			false
		}
		4 => {
			// Mirrored and rotated 180°
			img.fliph();
			img.rotate180();
			false
		}
		5 => {
			// Mirrored and rotated -90°
			img.fliph();
			img.rotate270();
			true
		}
		6 => {
			// Rotated -90°
			img.rotate90();
			true
		}
		7 => {
			// Mirrored and rotated 90°
			img.fliph();
			img.rotate90();
			true
		}
		8 => {
			// Rotated -90°
			img.rotate270();
			true
		}
		0 | 9 | _ => {
			// Not rotated, Undefined or Invalid. Keep image as is.
			false
		}
	}
}

/// Resize the image at the given source path to fit into the given resolution, keeping its aspect ratio, using the
/// given method and save it with the given quality as a jpg file under the target path
pub fn resize_jpg(
	source: &PathBuf,
	target: &PathBuf,
	resolution: Resolution,
	quality: u8,
	method: &String,
) -> Result<(), Box<dyn std::error::Error>> {
	let method = match method.as_str() {
		RESIZE_METHOD_LANCZOS3 => image::imageops::FilterType::Lanczos3,
		RESIZE_METHOD_GAUSSIAN => image::imageops::FilterType::Gaussian,
		RESIZE_METHOD_NEAREST => image::imageops::FilterType::Nearest,
		RESIZE_METHOD_CUBIC => image::imageops::FilterType::CatmullRom,
		RESIZE_METHOD_LINEAR => image::imageops::FilterType::Triangle,
		_ => panic!("Invalid resize method: {}", method),
	};

	let img = image::open(source)?;
	let switch_sides = rotate_image(&img);

	let ratio = img.width() as f64 / img.height() as f64;
	let mut width = resolution.width as f64;
	let mut height = resolution.height as f64;

	if switch_sides {
		std::mem::swap(&mut width, &mut height);
	}

	if width / ratio > resolution.height as f64 {
		height = width / ratio;
	} else {
		width = height / ratio;
	}

	let new_img = image::imageops::resize(&img, width as u32, height as u32, method);

	let mut out = std::fs::File::create(target)?;
	let mut enc = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut out, quality);
	enc.encode_image(&new_img)?;

	Ok(())
}

/// Recode the image at the given source path, recode it with the given quality and save it as jpg under the given
/// target path
pub fn recode_jpg(
	source: &PathBuf,
	target: &PathBuf,
	quality: u8,
) -> Result<(), Box<dyn std::error::Error>> {
	let image = image::open(source)?;
	rotate_image(&image);

	let mut out = std::fs::File::create(target)?;
	let mut enc = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut out, quality);
	enc.encode_image(&image)?;

	Ok(())
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
