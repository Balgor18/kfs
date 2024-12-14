
/*
| 63 56 | 55 52 | 51 48 | 47 40 | 39 32 | 31 16 | 15 0 |
| ----- | ----- | ----- | ----- | ----- | ----- | ---- |
| Base | Flags | Limit |  Access Byte | Base| Base | Limit |
|  00  |   c   |   f   |      9a      |  00 | 0000 |  ffff |
*/
// Bits: 0-15   | Offset (Bits 0-15)
// Bits: 16-31  | Selector (Segment Selector)
// Bits: 32-39  | Reserved (Doit être 0)
// Bits: 40-47  | Type and Attributes
// Bits: 48-63  | Offset (Bits 16-31)

use crate::{cpu::gdt::DescriptorTable, interrupt_handler, utility::instruction::igdt};

const IDT_START_ADDRESS: u64 = 0x900;
const IDT_ENTRIES: usize = 256;

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct IdtEntry {
	offset_low: u16,
	selector: u16,
	ist: u8,
	type_attr: u8,
	offset_mid: u16,
	offset_high: u32,
	zero: u32,
}

impl IdtEntry {
	pub const fn new() -> Self {
		Self {
			offset_low: 0,
			selector: 0,
			ist: 0, 
			type_attr: 0,
			offset_mid: 0,
			offset_high: 0,
			zero: 0,
		}
	}

	pub fn set_handler(&mut self, handler: u32, selector: u16, type_attr: u8) {
		self.offset_low = (handler & 0xFFFF) as u16;
		self.offset_mid = ((handler >> 16) & 0xFFFF) as u16;
		self.offset_high = 0;// ALWAYS 0
		self.selector = selector;
		self.type_attr = type_attr;
		self.zero = 0;
	}
}

#[no_mangle]
static mut IDT: [IdtEntry; IDT_ENTRIES] = [IdtEntry::new(); IDT_ENTRIES];

pub unsafe fn init() {
	IDT[0].set_handler(interrupt_handler as u32, 0x08, 0x8E);

	const IDT_CONFIG: DescriptorTable = DescriptorTable {
		limit: (IDT_ENTRIES * core::mem::size_of::<IdtEntry>() - 1) as u16,
		base: IDT_START_ADDRESS,
	};

	core::ptr::copy_nonoverlapping(IDT.as_ptr(), IDT_START_ADDRESS as *mut IdtEntry, IDT_ENTRIES);


	igdt(&IDT_CONFIG);
}
