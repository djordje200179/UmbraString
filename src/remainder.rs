pub(crate) union Remainder {
	pub word: u64,
	pub inline_data: [u8; 8],
	pub heap_data: *const u8,
}

impl Default for Remainder {
	fn default() -> Self {
		Remainder { word: 0 }
	}
}