//! French keyboard support

use crate::{keyboard::input_method::{keycode::KeyCode, modifiers::Modifiers}, VGA};

/// A standard French 102-key (or 105-key including Windows keys) keyboard.
///
/// The top row spells `AZERTY`.
///
/// Has a 2-row high Enter key, with Oem5 next to the left shift (ISO format).
///
/// NB: no "dead key" support for now

#[derive(Default)]
pub struct Azerty;

impl Azerty {
    pub fn map_keycode(
        &self,
        keycode: KeyCode,
        modifiers: &Modifiers,
    ) -> char {
        match keycode {
            KeyCode::Escape => 0x1B.into(),
            KeyCode::Backquote => '²',
            KeyCode::IntlBackslash => {
                if modifiers.is_shifted() {
                    '>'
                } else {
                    '<'
                }
            }
            KeyCode::Digit1 => {
                if modifiers.is_shifted() {
                    '1'
                } else {
                    '&'
                }
            }
            KeyCode::Digit2 => {
                if modifiers.is_shifted() {
                    '2'
                } else if modifiers.is_altgr() {
                    '~'
                } else {
                    'é'
                }
            }
            KeyCode::Digit3 => {
                if modifiers.is_shifted() {
                    '3'
                } else if modifiers.is_altgr() {
                    '#'
                } else {
                    '"'
                }
            }
            KeyCode::Digit4 => {
                if modifiers.is_shifted() {
                    '4'
                } else if modifiers.is_altgr() {
                    '{'
                } else {
                    '\''
                }
            }
            KeyCode::Digit5 => {
                if modifiers.is_shifted() {
                    '5'
                } else if modifiers.is_altgr() {
                    '['
                } else {
                    '('
                }
            }
            KeyCode::Digit6 => {
                if modifiers.is_shifted() {
                    '6'
                } else if modifiers.is_altgr() {
                    '|'
                } else {
                    '-'
                }
            }
            KeyCode::Digit7 => {
                if modifiers.is_shifted() {
                    '7'
                } else if modifiers.is_altgr() {
                    '`'
                } else {
                    'è'
                }
            }
            KeyCode::Digit8 => {
                if modifiers.is_shifted() {
                    '8'
                } else if modifiers.is_altgr() {
                    '\\'
                } else {
                    '_'
                }
            }
            KeyCode::Digit9 => {
                if modifiers.is_shifted() {
                    '9'
                } else if modifiers.is_altgr() {
                    '^'
                } else {
                    'ç'
                }
            }
            KeyCode::Digit0 => {
                if modifiers.is_shifted() {
                    '0'
                } else if modifiers.is_altgr() {
                    '@'
                } else {
                    'à'
                }
            }
            KeyCode::Minus => {
                if modifiers.is_shifted() {
                    '°'
                } else if modifiers.is_altgr() {
                    ']'
                } else {
                    ')'
                }
            }
            KeyCode::Equal => {
                if modifiers.is_shifted() {
                    '+'
                } else if modifiers.is_altgr() {
                    '}'
                } else {
                    '='
                }
            }
            KeyCode::Backspace => 0x08.into(),
            KeyCode::Tab => 0x09.into(),
            KeyCode::KeyQ => {
				if modifiers.is_caps() {
                    'A'
                } else {
                    'a'
                }
            }
            KeyCode::KeyW => {
                if modifiers.is_caps() {
                    'Z'
                } else {
                    'z'
                }
            }
            KeyCode::KeyE => {
                if modifiers.is_caps() {
                    'E'
                } else {
                    'e'
                }
            }
            KeyCode::KeyR => {
                if modifiers.is_caps() {
                    'R'
                } else {
                    'r'
                }
            }
            KeyCode::KeyT => {
                if modifiers.is_caps() {
                    'T'
                } else {
                    't'
                }
            }
            KeyCode::KeyY => {
                if modifiers.is_caps() {
                    'Y'
                } else {
                    'y'
                }
            }
            KeyCode::KeyU => {
                if modifiers.is_caps() {
                    'U'
                } else {
                    'u'
                }
            }
            KeyCode::KeyI => {
                if modifiers.is_caps() {
                    'I'
                } else {
                    'i'
                }
            }
            KeyCode::KeyO => {
                if modifiers.is_caps() {
                    'O'
                } else {
                    'o'
                }
            }
            KeyCode::KeyP => {
                if modifiers.is_caps() {
                    'P'
                } else {
                    'p'
                }
            }
            KeyCode::BracketLeft => {
                if modifiers.is_altgr() {
                    'ˇ'
                } else {
                    '^'
                }
            }
            KeyCode::BracketRight => {
                if modifiers.is_altgr() {
                    '¤'
                } else {
                    '$'
                }
            }
            KeyCode::Backslash => {
                if modifiers.is_shifted() {
                    'µ'
                } else {
                    '*'
                }
            }
            KeyCode::KeyA => {
                if modifiers.is_caps() {
                    'Q'
                } else {
                    'q'
                }
            }
            KeyCode::KeyS => {
                if modifiers.is_caps() {
                    'S'
                } else {
                    's'
                }
            }
            KeyCode::KeyD => {
                if modifiers.is_caps() {
                    'D'
                } else {
                    'd'
                }
            }
            KeyCode::KeyF => {
                if modifiers.is_caps() {
                    'F'
                } else {
                    'f'
                }
            }
            KeyCode::KeyG => {
                if modifiers.is_caps() {
                    'G'
                } else {
                    'g'
                }
            }
            KeyCode::KeyH => {
                if modifiers.is_caps() {
                    'H'
                } else {
                    'h'
                }
            }
            KeyCode::KeyJ => {
                if modifiers.is_caps() {
                    'J'
                } else {
                    'j'
                }
            }
            KeyCode::KeyK => {
                if modifiers.is_caps() {
                    'K'
                } else {
                    'k'
                }
            }
            KeyCode::KeyL => {
				if modifiers.is_ctrl() {
                    unsafe{
                        VGA.reset();
                    }
                    '\u{000C}'
				}
                else if modifiers.is_caps() {
                    'L'
                } else {
                    'l'
                }
            }
            KeyCode::Semicolon => {
                if modifiers.is_caps() {
                    'M'
                } else {
                    'm'
                }
            }
            KeyCode::Quote => {
                if modifiers.is_shifted() {
                    '%'
                } else {
                    'ù'
                }
            }
            // Enter gives LF, not CRLF or CR
            KeyCode::Enter => 10.into(),
            KeyCode::KeyZ => {
                if modifiers.is_caps() {
                    'W'
                } else {
                    'w'
                }
            }
            KeyCode::KeyX => {
                if modifiers.is_caps() {
                    'X'
                } else {
                    'x'
                }
            }
            KeyCode::KeyC => {
				if modifiers.is_ctrl() {
                    unsafe{
                        VGA.putchar('\n' as u8);
                    }
                    '\u{0003}'
				}
                else if modifiers.is_caps() {
                    'C'
                } else {
                    'c'
                }
            }
            KeyCode::KeyV => {
                if modifiers.is_caps() {
                    'V'
                } else {
                    'v'
                }
            }
            KeyCode::KeyB => {
                if modifiers.is_caps() {
                    'B'
                } else {
                    'b'
                }
            }
            KeyCode::KeyN => {
                if modifiers.is_caps() {
                    'N'
                } else {
                    'n'
                }
            }
            KeyCode::KeyM => {
                if modifiers.is_caps() {
                    '?'
                } else {
                    ','
                }
            }
            KeyCode::Comma => {
                if modifiers.is_shifted() {
                    '.'
                } else {
                    ';'
                }
            }
            KeyCode::Period => {
                if modifiers.is_shifted() {
                    '/'
                } else {
                    ':'
                }
            }
            KeyCode::Slash => {
                if modifiers.is_shifted() {
                    '§'
                } else {
                    '!'
                }
            }
            KeyCode::Space => ' ',
            KeyCode::Delete => 127.into(),
            KeyCode::NumpadDivide => '/',
            KeyCode::NumpadMultiply => '*',
            KeyCode::NumpadSubtract => '-',
            KeyCode::Numpad7 => {
                if modifiers.numlock {
                    '7'
                } else {
                    '\0'
                }
            }
            KeyCode::Numpad8 => {
                if modifiers.numlock {
                    '8'
                } else {
                    '\0'
                }
            }
            KeyCode::Numpad9 => {
                if modifiers.numlock {
                    '9'
                } else {
                    '\0'
                }
            }
            KeyCode::NumpadAdd => '+',
            KeyCode::Numpad4 => {
                if modifiers.numlock {
                    '4'
                } else {
                    '\0'
                }
            }
            KeyCode::Numpad5 => '5',
            KeyCode::Numpad6 => {
                if modifiers.numlock {
                    '6'
                } else {
                    '\0'
                }
            }
            KeyCode::Numpad1 => {
                if modifiers.numlock {
                    '1'
                } else {
                    '\0'
                }
            }
            KeyCode::Numpad2 => {
                if modifiers.numlock {
                    '2'
                } else {
                    '\0'
                }
            }
            KeyCode::Numpad3 => {
                if modifiers.numlock {
                    '3'
                } else {
                    '\0'
                }
            }
            KeyCode::Numpad0 => {
                if modifiers.numlock {
                    '0'
                } else {
                    '\0'
                }
            }
            KeyCode::NumpadDecimal => {
                if modifiers.numlock {
                    '.'
                } else {
                    127.into()
                }
            }
            KeyCode::NumpadEnter => 10.into(),
            _k => '\0',
        }
    }
}