const PRESENT :u32 = 1<<0;
const WRITABLE :u32 = 1<<1;
const USER_ACCESS :u32 = 1<<2;
const WRITE: u32 = 1<<3;
const CACHE_DISABLE: u32 = 1 << 4;
const ACCESSED: u32 = 1 << 5;
const DIRTY: u32 = 1 << 6;
const PAGESIZE4MB: u32 = 1 << 7;
const ADDRESS: u32 = 1 << 8;

/// Page Table Structure
#[repr(C, align(4096))]
pub struct PageTable([u32; 1024]);

impl PageTable {
    pub fn new() -> Self {
        Self([0; 1024])
    }

    /// Map a 4 KiB page
    pub fn map_4kb_page(&mut self, index: usize, physical_address: u32, flags: u32) {
        assert!(physical_address & 0xFFF == 0, "Address must be 4 KiB aligned");
        self.0[index] = physical_address | flags;
    }
}