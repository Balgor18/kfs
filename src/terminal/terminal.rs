use crate::driver::ps2::wait_for_next_scancode;
use crate::driver::vga::WIDTH;
use crate::keyboard::keyboard::Keyboard;
use crate::VGA;


pub struct Terminal {
	cmd: [u8; WIDTH],
	layout : Keyboard,
}

impl Terminal {
	pub fn new() -> Self {
		Self {
			cmd: [b'\0'; WIDTH],
			layout : Keyboard::default(),
		}
	}

	fn strlen(&mut self) -> usize {
		let mut length : usize = 0;
		
		while length < self.cmd.len() && self.cmd[length] != 0 {
			length += 1;
		}	
		return length
	}

	fn add_to_cmd(&mut self, c : u8) {
		printk!("Len = {}", self.strlen());
		self.cmd[self.strlen()] = c;
	}

	fn clear_cmd(&mut self){
		self.cmd = [b'\0'; WIDTH];
	}

	pub fn cmd_entry(&mut self) {
		let scancode = wait_for_next_scancode();
		if let Some(c) = self.layout.scancode_to_char(scancode) {
			if c != '\n' {
				self.add_to_cmd(c as u8);
			}
			else if c == '\n' {
				self.submit();
				self.clear_cmd();
			}
			unsafe{
				// Saved into cmd
				// If c == \n check if cmd exist or no
					// Dump the cmd when cmd is enter
				// If c != saved char
				VGA.putchar(c as u8);
			}
		}
	}

	fn submit(&mut self) {
		match self.cmd {
			"help" => {
				unsafe{
					VGA.putstr(include_str!("../help.txt"));
				}
			}
			"clear" => {
				unsafe {
					VGA.reset();
				}
			}
			_ => (),
		}
	}

}