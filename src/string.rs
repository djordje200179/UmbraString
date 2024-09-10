use core::slice;
use std::{cmp::Ordering, fmt::{Debug, Display, Formatter}, ptr::slice_from_raw_parts, str::from_utf8_unchecked};

use crate::{prefix::Prefix, remainder::Remainder};

pub struct String {
	len: u32,
	prefix: Prefix,
	remainder: Remainder,
}

impl String {
	fn prefix_str(&self) -> &str {
		unsafe { from_utf8_unchecked(&self.prefix.bytes[..self.len.min(4) as usize]) }
	}

	fn remainder_str(&self) -> &str {
		unsafe { from_utf8_unchecked(
			match self.len {
				0..=4 => &[],
				5..=12 => &self.remainder.inline_data[..(self.len - 4) as usize],
				_ => self.heap_data(),
			}
		)}
	}

	unsafe fn heap_data(&self) -> &[u8] {
		slice::from_raw_parts(self.remainder.heap_data, (self.len - 4) as usize)
	}

	pub fn len(&self) -> u32 {
		self.len
	}
}

impl Default for String {
	fn default() -> Self {
		String {
			len: 0,
			prefix: Prefix::default(),
			remainder: Remainder::default(),
		}
	}
}

impl From<&str> for String {
	fn from(mut str_slice: &str) -> Self {
		let mut ustr = String::default();
		ustr.len = str_slice.len() as u32;

		unsafe {
			match str_slice.len() {
				0..=4 => {
					ustr.prefix.bytes[..str_slice.len()].copy_from_slice(str_slice.as_bytes());
				},
				5..=12 => {
					ustr.prefix.bytes.copy_from_slice(&str_slice.as_bytes()[0..4]);
					str_slice = &str_slice[4..];
					ustr.remainder.inline_data[..str_slice.len()].copy_from_slice(str_slice.as_bytes());
				},
				_ => {
					ustr.prefix.bytes.copy_from_slice(&str_slice.as_bytes()[0..4]);
					str_slice = &str_slice[4..];

					let heap_data = Box::new(str_slice.as_bytes().to_vec().into_boxed_slice());
					ustr.remainder.heap_data = Box::leak(heap_data).as_ptr()
				},
			}
		}

		ustr
	}
}

impl Clone for String {
	fn clone(&self) -> Self {
		String {
			remainder: unsafe { if self.len > 12 {
				let cloned_heap_data = self.heap_data().to_vec().into_boxed_slice();
				Remainder { heap_data: Box::leak(Box::new(cloned_heap_data)).as_ptr() }
			} else {
				Remainder { inline_data: self.remainder.inline_data }
			}},
			..*self
		}
	}
}

impl Drop for String {
	fn drop(&mut self) {
		if self.len > 12 {
			unsafe {
				let heap_slice = slice_from_raw_parts(self.remainder.heap_data, (self.len - 4) as usize);
				drop(Box::from_raw(heap_slice as *const [u8] as *mut [u8]));
			}
		}
	}
}

impl Display for String {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		if self.len <= 4 {
			return write!(f, "{}", self.prefix_str());
		} else {
			write!(f, "{}{}", self.prefix_str(), self.remainder_str())?;
			Ok(())
		}
	}
}

impl Debug for String {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "String({})", self)
	}
}

impl PartialEq for String {
	fn eq(&self, other: &Self) -> bool {
		if self.len != other.len || self.prefix != other.prefix {
			return false;
		}

		unsafe {
			match self.len {
				0..=4 => true,
				5..=12 => self.remainder.word == other.remainder.word,
				_ => self.heap_data() == other.heap_data(),
			}
		}
	}
}

impl Eq for String {}

impl Ord for String {
	fn cmp(&self, other: &Self) -> Ordering {
		unsafe {
			if self.prefix.word > other.prefix.word {
				return Ordering::Greater;
			} else if self.prefix.word < other.prefix.word {
				return Ordering::Less;
			}

			match self.len {
				0..=4 => Ordering::Equal,
				5..=12 => self.remainder.word.cmp(&other.remainder.word),
				_ => self.heap_data().cmp(other.heap_data()),
			}
		}
	}
}

impl PartialOrd for String {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}