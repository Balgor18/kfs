use core::arch::asm;
use directory::PageDirectory;
use page_table::PageTable;

use crate::utility::multiboot::MmapInfo;

pub mod directory;
pub mod page_table;

const PAGE_PRESENT: u32 = 0x1;   // Present
const PAGE_RW: u32 = 0x2;        // Read/Write

pub fn init(mmap_info : *mut MmapInfo, mmap_length : u32) -> PageDirectory {

	let mut page_directory = PageDirectory::new();
	let mut page_table = PageTable::new();

	let mut offset : u32 = 0;
	let mut tmp : *mut MmapInfo = mmap_info;

	unsafe {
		while offset < mmap_length {
			if (*tmp).type_ == 1 {

				// CASE OF 4Kib
				if (*tmp).base_addr < 0x00400000 {
					let start_page = (*tmp).base_addr / 0x1000;
					let end_page = ((*tmp).base_addr + (*tmp).length) / 0x1000;
	
					for page in start_page..end_page {
						page_table.map_4kb_page(page as usize, (page * 0x1000) as u32, PAGE_PRESENT | PAGE_RW);
					}
				// CASE OF 4Mib
				} else {
					let start_page = (*tmp).base_addr / 0x400000;
					let end_page = ((*tmp).base_addr + (*tmp).length) / 0x400000;
	
					for page in start_page..end_page {
						page_directory.map_4mb_page(page as usize, (page * 0x400000) as u32, PAGE_PRESENT | PAGE_RW);
					}
				}
			}
			tmp = tmp.byte_add((*tmp).size as usize + 4);
			offset += (*tmp).size + 4;
		}
	}

	// Push Page Table into the Page Directory
	page_directory.map_page_table(0, &page_table, PAGE_PRESENT | PAGE_RW);

	return page_directory;
}

pub unsafe fn activate_pagging(page_directory : u32) {
	asm!(
		// 1. Load address in CR3
		"
		mov cr3, {page_directory}
		",
		// 2. Activate extension CR4
		"
		mov {tmp}, cr4
		or {tmp}, 0x20
		mov cr4, {tmp}
		",
		// 3. Activate CR0 with PG = 1
		"
		mov {tmp}, cr0
		or {tmp}, 0x80000000
		mov cr0, {tmp}
		",
		page_directory = in(reg) page_directory,
		tmp = lateout(reg) _,
		options(nostack, preserves_flags)
	);
}