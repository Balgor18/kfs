use core::fmt::Display;

pub struct Allocator {
	pages: &'static mut [u32],
	len: usize,
}

impl Allocator {
	pub fn new(storage: &'static mut [u32]) -> Self {
		Self {
			pages: storage,
			len: 0,
		}
	}

	pub fn free(&mut self, page: u32) {
		assert!(
			self.len < self.pages.len(),
			"out of memory for the allocator"
		);

		self.pages[self.len] = page;

		self.len += 1;
	}

	pub fn allocate(&mut self) -> Result<u32, OutOfMemory> {
		if self.len == 0 {
			return Err(OutOfMemory);
		}

		self.len -= 1;
		Ok(self.pages[self.len])
	}

	pub fn get_size(&self) -> usize {
		self.len * 0x1000
	}
}

#[derive(Debug)]
pub struct OutOfMemory;

impl Display for OutOfMemory {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "out of memory")
	}
}