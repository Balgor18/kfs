use super::input_method::keycode::{KeyCode, ScancodeSet1};
use super::input_method::modifiers::Modifiers;
use super::layout::azerty::Azerty;
use super::layout::qwerty::Qwerty;

pub trait KeyboardLayout {
	fn map_keycode(&self, keycode: KeyCode, modifiers: &Modifiers) -> char;
}

enum AnyLayout {
	Qwerty(Qwerty),
	Azerty(Azerty),
}

impl Default for AnyLayout {
	fn default() -> Self {
		AnyLayout::Qwerty(Qwerty)
	}
}

impl KeyboardLayout for AnyLayout {
	fn map_keycode(&self, keycode: KeyCode, modifiers: &Modifiers) -> char {
		match self {
			AnyLayout::Qwerty(layout) => layout.map_keycode(keycode, modifiers),
			AnyLayout::Azerty(layout) => layout.map_keycode(keycode, modifiers),
		}
	}
}

#[derive(Default)]
pub struct Keyboard{
	layout : AnyLayout,
	modifiers : Modifiers,
	key_event : ScancodeSet1
}

impl Keyboard {

	pub fn new() -> Self {
		Self {
			layout: AnyLayout::Qwerty(Qwerty), // Layout par défaut : Qwerty
			modifiers: Modifiers::default(),
			key_event: ScancodeSet1::default(),
		}
	}

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
		let char = self.layout.map_keycode(event.key_code, &self.modifiers);
		if char == '\0' || !event.pressed {
			None
		} else {
			Some(char)
		}
	}

	pub fn set_layout(&mut self, layout: &str) {
		self.layout = match layout {
			"us" => AnyLayout::Qwerty(Qwerty),
			"fr" => AnyLayout::Azerty(Azerty),
			_ => {
				printk!("Layout not supported!");
				return;
			}
		};
	}
}