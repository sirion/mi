/// Very simple way of hashing bytes. Not safe, can have collisions.
/// This hash just sums up all values in the Vec and overflows if needed
///
/// # Example
///
/// ```
/// use mi::bin::hash_quick;
/// assert_eq!(hash_quick(vec![u64::MAX, 1u64, u64::MAX, 15u64]), 14u64);
/// ```
pub fn hash_quick<T: std::ops::Add + Into<u64>>(data: Vec<T>) -> u64 {
	let mut n = (0u64, false);

	for c in data {
		n = n.0.overflowing_add(c.into());
	}

	n.0
}

/// Contains methods to find patterns within data
pub trait Find {
	/// Finds the first index of the pattern after the given start index
	fn index_of(&self, pattern: &[u8], start: usize) -> Result<usize, usize>;
}

impl Find for Vec<u8> {
	fn index_of(&self, pattern: &[u8], start: usize) -> Result<usize, usize> {
		let mut found = false;

		let mut i = start;
		while i < self.len() {
			if self[i] == pattern[0] {
				let mut matching = true;
				for n in 1..pattern.len() {
					if self[i + n] != pattern[n] {
						matching = false;
						break;
					}
				}
				if matching {
					found = true;
					break;
				}
			}
			i = i + 1;
		}

		if found {
			Ok(i)
		} else {
			Err(0)
		}
	}
}

impl Find for [u8] {
	fn index_of(&self, pattern: &[u8], start: usize) -> Result<usize, usize> {
		Vec::from(self).index_of(pattern, start)
	}
}

/// Contains methods to replace patterns within data. Depends on [Find].
pub trait Replace {
	/// Returns a new vec with the given pattern replaced by the replacement
	fn replace(&self, pattern: &[u8], replacement: &[u8]) -> Self;

	/// Returns a new vec with the given replacement instead of the part surrounded by the two patterns (leaves the patterns in)
	fn replace_between(&self, pattern_start: &[u8], pattern_end: &[u8], replacement: &[u8])
		-> Self;

	/// Returns a new vec containing what is bewteen the given patterns
	fn between(&self, pattern_start: &[u8], pattern_end: &[u8]) -> Self;
}

impl Replace for Vec<u8> {
	fn replace(&self, pattern: &[u8], replacement: &[u8]) -> Vec<u8> {
		if pattern.len() == 0 {
			return self.clone();
		}

		let mut new: Vec<u8> = Vec::with_capacity(self.len() - pattern.len() + replacement.len());

		let mut i = 0;
		while i < self.len() {
			if self[i] == pattern[0] {
				let mut matching = true;
				for n in 1..pattern.len() {
					if self[i + n] != pattern[n] {
						matching = false;
						break;
					}
				}
				if matching {
					let mut r = Vec::from(replacement);
					new.append(&mut r);
					i = i + pattern.len();
				} else {
					new.push(self[i]);
				}
			} else {
				new.push(self[i]);
			}
			i = i + 1;
		}

		new
	}

	fn replace_between(
		&self,
		pattern_start: &[u8],
		pattern_end: &[u8],
		replacement: &[u8],
	) -> Vec<u8> {
		if pattern_start.len() == 0 || pattern_end.len() == 0 {
			return self.to_vec();
		}

		let index_start = match self.index_of(pattern_start, 0) {
			Ok(i) => i,
			Err(_) => {
				return self.to_vec();
			}
		};

		let index_end = match self.index_of(pattern_end, index_start + pattern_start.len()) {
			Ok(i) => i,
			Err(_) => {
				return self.to_vec();
			}
		};

		let mut new: Vec<u8> = Vec::with_capacity(self.len() + replacement.len());

		new.extend_from_slice(&self[0..index_start + pattern_start.len()]);
		new.extend_from_slice(replacement);
		new.extend_from_slice(&self[index_end..]);

		new
	}

	fn between(&self, pattern_start: &[u8], pattern_end: &[u8]) -> Vec<u8> {
		if pattern_start.len() == 0 || pattern_end.len() == 0 {
			return self.to_vec();
		}

		let index_start = match self.index_of(pattern_start, 0) {
			Ok(i) => i,
			Err(_) => {
				return self.to_vec();
			}
		};

		let index_end = match self.index_of(pattern_end, index_start + pattern_start.len()) {
			Ok(i) => i,
			Err(_) => {
				return self.to_vec();
			}
		};

		Vec::from(&self[index_start + pattern_start.len()..index_end])
	}
}
