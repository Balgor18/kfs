use super::input_method::keycode::{KeyCode, ScancodeSet1};
use super::input_method::modifiers::Modifiers;
use super::layout::qwerty::Qwerty;

#[derive(Default)]
 pub struct Keyboard{
	layout : Qwerty,
	modifiers : Modifiers,
	key_event : ScancodeSet1
}

impl Keyboard {
	pub fn scancode_to_char(&mut self, scancode: u8) -> Option<char> {
		let event = self.key_event.advance_state(scancode)?;
		match event.key_code {
			KeyCode::ShiftLeft => {
				self.modifiers.lshift = event.pressed;
			}
			KeyCode::ShiftRight => {
				self.modifiers.rshift = event.pressed;
			}
			KeyCode::ControlLeft => {
				self.modifiers.lctrl = event.pressed;
			}
			KeyCode::ControlRight => {
				self.modifiers.rctrl = event.pressed;
			}
			KeyCode::CapsLock => {
				if event.pressed {
					self.modifiers.capslock = !self.modifiers.capslock;
				}
			}
			KeyCode::AltLeft => {
				self.modifiers.lalt = event.pressed;
			}
			KeyCode::AltRight => {
				self.modifiers.ralt = event.pressed;
			}
			KeyCode::NumLock => {
				if event.pressed {
					self.modifiers.numlock = !self.modifiers.numlock;
				}
			}
			_ => {}
		}
		let c = self.layout.map_keycode(event.key_code, &self.modifiers);
		if c == '\0' || !event.pressed {
			None
		} else {
			Some(c)
		}
	}
}