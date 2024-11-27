use super::page_table::PageTable;

const PAGE_4MB: u32 = 1 << 7;    // Page Size (4 MiB)

// Page Directory Structure
#[repr(C, align(4096))]
pub struct PageDirectory([u32; 1024]);

impl PageDirectory {
    pub fn new() -> Self {
        Self([0; 1024])
    }

    /// Map a Page Table in the Page Directory
    pub fn map_page_table(&mut self, index: usize, page_table: &PageTable, flags: u32) {
        let physical_address = page_table as *const _ as u32;
        self.0[index] = physical_address | flags;
    }

    /// Map a 4 MiB page
    pub fn map_4mb_page(&mut self, index: usize, physical_address: u32, flags: u32) {
        assert!(physical_address & 0x003FFFFF == 0, "Address must be 4 MiB aligned");
        self.0[index] = physical_address | flags | PAGE_4MB;
        printk!("{:?}", self.0[index]);
    }
}