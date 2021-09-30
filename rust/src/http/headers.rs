use std::collections::HashMap;

/// Header map for [Request]s and [Response]s
pub struct Headers {
	/// If case_handling is set to true, header keys will be changed to the de-facto standard of starting with an
	/// upper-case letter at the beginning and after every dash.
	/// For performance reasons this defaults to false for [Response]s and for compatibility reasons this is set to
	/// true for incoming [Request]s
	pub case_handling: bool,
	values: HashMap<String, Vec<String>>,
}

impl Headers {
	/// Creates new [Headers]
	pub fn new() -> Headers {
		Headers {
			case_handling: false,
			values: HashMap::new(),
		}
	}

	/// Returns a reference to all currently stored headers as a map of string vectors
	pub fn all(&self) -> &HashMap<String, Vec<String>> {
		&self.values
	}

	/// Returns the last value stored under the given name if any are set
	pub fn get(&self, k: &str) -> Option<&str> {
		let key = match self.case_handling {
			false => String::from(k),
			true => self.header_case(k),
		};

		match self.values.get(&key) {
			Some(v) => match v.last() {
				Some(v) => Some(v.as_str()),
				None => None,
			},
			None => None,
		}
	}

	/// Sets the given header and replaces any values previously set for this key
	pub fn set(&mut self, k: &str, v: &str) {
		let key = match self.case_handling {
			false => String::from(k),
			true => self.header_case(k),
		};

		self.values.remove(&key);
		self.values.insert(key, vec![String::from(v)]);
	}

	/// Sets the given header value and adds to any previously existing values
	pub fn add(&mut self, k: &str, v: &str) {
		let key = match self.case_handling {
			false => String::from(k),
			true => self.header_case(k),
		};

		if !self.values.contains_key(&key) {
			self.values.insert(key, vec![String::from(v)]);
		} else {
			self.values.get_mut(&key).unwrap().push(String::from(v));
		}
	}

	fn header_case(&self, k: &str) -> String {
		let mut key = String::with_capacity(k.len());
		let mut up = true;
		for c in k.chars() {
			if up {
				key.extend(c.to_uppercase());
				up = false;
			} else {
				key.push(c);
				if c == '-' {
					up = true;
				}
			}
		}
		key
	}
}
