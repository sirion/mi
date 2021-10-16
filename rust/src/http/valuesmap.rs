use std::collections::HashMap;

/// Header map for [super::Request]s and [super::Response]s
pub struct ValuesMap {
	/// If case_handling is set to true, header keys will be changed to the de-facto standard for headers of starting
	/// with an upper-case letter at the beginning and after every dash.
	/// For performance reasons this defaults to false for ValuesMaps used for headers in [super::Response]s and for
	/// compatibility reasons this is set to true for ones used in incoming [super::Request]s
	pub case_handling: bool,
	values: HashMap<String, Vec<String>>,
}

impl ValuesMap {
	/// Creates new [super::ValuesMap]
	pub fn new() -> ValuesMap {
		ValuesMap {
			case_handling: false,
			values: HashMap::new(),
		}
	}

	/// Returns a reference to all currently stored values as a map of string vectors
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

	/// Returns a vector of all values stored for the given key
	pub fn get_all(&self, k: &str) -> Option<&Vec<String>> {
		self.values.get(k)
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

	/// Returns true is no values are stored
	pub fn is_empty(&self) -> bool {
		self.values.is_empty()
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
