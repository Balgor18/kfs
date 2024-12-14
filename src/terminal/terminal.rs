use crate::driver::ps2::wait_for_next_scancode;
use crate::driver::vga::WIDTH;
use crate::keyboard::keyboard::Keyboard;
use crate::VGA;
use core::str;
use crate::keyboard::layout::azerty::Azerty;
use crate::keyboard::layout::qwerty::Qwerty;

pub struct Terminal {
	cmd: [u8; WIDTH],
	layout : Keyboard,
}

impl Terminal {
	pub fn new() -> Self {
		Self {
			cmd: [b'\0'; WIDTH],
			layout : Keyboard::new(),
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
		assert!(self.strlen() < WIDTH, "bad size");
		self.cmd[self.strlen()] = c;
	}

	fn clear_cmd(&mut self){
		self.cmd = [b'\0'; WIDTH];
	}

	fn delete_last_char(&mut self) {
		if self.strlen() == 0 { return; }
		self.cmd[self.strlen() - 1] = '\0' as u8;
	}

	pub fn cmd_entry(&mut self) {
		let scancode = wait_for_next_scancode();
		if let Some(c) = self.layout.scancode_to_char(scancode) {
			if c == '\x08' {
				self.delete_last_char();
				unsafe {
					VGA.erase_specific_char();
				}
			} else if c != '\n' {
				self.add_to_cmd(c as u8);
				unsafe { VGA.putchar(c as u8); }
			}
			else if c == '\n' {
				unsafe { VGA.putchar(c as u8); }
				self.submit();
				self.clear_cmd();
			}
		}
	}

	fn submit(&mut self) {
		let length = self.cmd.iter().position(|&c| c == b'\0').unwrap_or(self.cmd.len());
	
		if let Ok(cmd_str) = str::from_utf8(&self.cmd[..length]) {
			let mut parts = cmd_str.split_whitespace();
			if let Some(command) = parts.next() {
				match command {
					"help" => {
						unsafe {
							VGA.putstr(include_str!("../help.txt"));
						}
					}
					"clear" => {
						unsafe {
							VGA.reset();
						}
					}
					"setkeyboard" => {
						if let Some(layout) = parts.next() {
							if layout == "fr" {
								self.layout.set_layout("fr");
							} else if layout == "us" {
								self.layout.set_layout("us");
							} else {
								printk!("Unsupported layout: {}", layout);
							}
						} else {
							printk!("Error: No layout specified!");
						}
					}
					_ => {
						printk!("Unknown command: {}", command);
					}
				}
			}
		}
	}

}