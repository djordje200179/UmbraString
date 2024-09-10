use std::mem::size_of;

#[derive(Copy, Clone, Eq)]
pub(crate) union Prefix {
	pub word: u32,
	pub bytes: [u8; size_of::<u32>()],
}

impl PartialEq for Prefix {
	fn eq(&self, other: &Self) -> bool {
		unsafe { self.word == other.word }
	}
}

impl Default for Prefix {
	fn default() -> Self {
		Prefix { word: 0 }
	}
}