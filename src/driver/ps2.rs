use crate::utility::instruction::inb;

const PORT_STATUS: u16 = 0x64;
/// The address of the VGA buffer.
const PORT_DATA : u16 = 0x60;

pub fn wait_for_next_scancode() -> u8 {
	unsafe {
		while inb(PORT_STATUS) & 1 == 0 {}// Check status of bit to get the data.
	 	return inb(PORT_DATA)
	}
}
