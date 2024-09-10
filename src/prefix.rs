use std::mem::size_of;

pub type PrefixType = u32;
pub const PREFIX_SIZE: usize = size_of::<PrefixType>();

#[derive(Copy, Clone, Eq)]
pub union Prefix {
	pub word: PrefixType,
	pub bytes: [u8; PREFIX_SIZE],
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