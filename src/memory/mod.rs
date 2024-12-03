use core::arch::asm;
use directory::PageDirectory;

pub mod directory;
pub mod page_table;

const PAGE_PRESENT: u32 = 0x1;   // Present
const PAGE_RW: u32 = 0x2;        // Read/Write
const ACCESSED: u32 = 0x5;
const DIRTY: u32 = 0x6;
const ADRESS_4MB : u32 = 0x00400000;

pub fn init(pde : & mut PageDirectory) {
	let start_page = 0 as u32;
	let end_page = 1024 as u32;

	for page in start_page..end_page {
		pde.map_4mb_page(page as usize, page * ADRESS_4MB, PAGE_PRESENT | PAGE_RW | ACCESSED | DIRTY);
	}
}

pub unsafe fn activate_pagging(page_directory : * mut PageDirectory) {
	asm!(
		// 1. Load address in CR3
		"
		mov cr3, {page_directory}
		",
		// 2. Activate extension CR4
		"
		mov {tmp}, cr4
		or {tmp}, 0x10
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