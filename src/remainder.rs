use std::mem::size_of;

pub type RemainderType = usize;
pub const REMAINDER_SIZE: usize = size_of::<RemainderType>();

pub(crate) union Remainder {
	pub word: RemainderType,
	pub inline_data: [u8; REMAINDER_SIZE],
	pub heap_data: *const u8,
}

impl Default for Remainder {
	fn default() -> Self {
		Remainder { word: 0 }
	}
}