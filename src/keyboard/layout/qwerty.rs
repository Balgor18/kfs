//! United States keyboard support

use crate::{keyboard::input_method::{keycode::KeyCode, modifiers::Modifiers}, VGA};


/// A standard United States 101-key (or 104-key including Windows keys) keyboard.
///
/// Has a 1-row high Enter key, with Oem5 above (ANSI layout).

#[derive(Default)]
pub struct Qwerty;

impl Qwerty {
    pub fn map_keycode(
        &self,
        keycode: KeyCode,
        modifiers: &Modifiers,
    ) -> char {
        match keycode {
            KeyCode::Backquote => {
                if modifiers.is_shifted() {
                    '~'
                } else {
                    '`'
                }
            }
            KeyCode::Escape => 0x1B.into(),
            KeyCode::Digit1 => {
                if modifiers.is_shifted() {
                    '!'
                } else {
                    '1'
                }
            }
            KeyCode::Digit2 => {
                if modifiers.is_shifted() {
                    '@'
                } else {
                    '2'
                }
            }
            KeyCode::Digit3 => {
                if modifiers.is_shifted() {
                    '#'
                } else {
                    '3'
                }
            }
            KeyCode::Digit4 => {
                if modifiers.is_shifted() {
                    '$'
                } else {
                    '4'
                }
            }
            KeyCode::Digit5 => {
                if modifiers.is_shifted() {
                    '%'
                } else {
                    '5'
                }
            }
            KeyCode::Digit6 => {
                if modifiers.is_shifted() {
                    '^'
                } else {
                    '6'
                }
            }
            KeyCode::Digit7 => {
                if modifiers.is_shifted() {
                    '&'
                } else {
                    '7'
                }
            }
            KeyCode::Digit8 => {
                if modifiers.is_shifted() {
                    '*'
                } else {
                    '8'
                }
            }
            KeyCode::Digit9 => {
                if modifiers.is_shifted() {
                    '('
                } else {
                    '9'
                }
            }
            KeyCode::Digit0 => {
                if modifiers.is_shifted() {
                    ')'
                } else {
                    '0'
                }
            }
            KeyCode::Minus => {
                if modifiers.is_shifted() {
                    '_'
                } else {
                    '-'
                }
            }
            KeyCode::Equal => {
                if modifiers.is_shifted() {
                    '+'
                } else {
                    '='
                }
            }
            KeyCode::Backspace => 0x08.into(),
            KeyCode::Tab => 0x09.into(),
            KeyCode::KeyQ => {
                if modifiers.is_ctrl() {
                    '\u{0011}'
                } else if modifiers.is_caps() {
                    'Q'
                } else {
                    'q'
                }
            }
            KeyCode::KeyW => {
                if modifiers.is_ctrl() {
                    '\u{0017}'
                } else if modifiers.is_caps() {
                    'W'
                } else {
                    'w'
                }
            }
            KeyCode::KeyE => {
                if modifiers.is_ctrl() {
                    '\u{0005}'
                } else if modifiers.is_caps() {
                    'E'
                } else {
                    'e'
                }
            }
            KeyCode::KeyR => {
                if modifiers.is_ctrl() {
                    '\u{0012}'
                } else if modifiers.is_caps() {
                    'R'
                } else {
                    'r'
                }
            }
            KeyCode::KeyT => {
                if modifiers.is_ctrl() {
                    '\u{0014}'
                } else if modifiers.is_caps() {
                    'T'
                } else {
                    't'
                }
            }
            KeyCode::KeyY => {
                if modifiers.is_ctrl() {
                    '\u{0019}'
                } else if modifiers.is_caps() {
                    'Y'
                } else {
                    'y'
                }
            }
            KeyCode::KeyU => {
                if modifiers.is_ctrl() {
                    '\u{0015}'
                } else if modifiers.is_caps() {
                    'U'
                } else {
                    'u'
                }
            }
            KeyCode::KeyI => {
                if modifiers.is_ctrl() {
                    '\u{0009}'
                } else if modifiers.is_caps() {
                    'I'
                } else {
                    'i'
                }
            }
            KeyCode::KeyO => {
                if modifiers.is_ctrl() {
                    '\u{000F}'
                } else if modifiers.is_caps() {
                    'O'
                } else {
                    'o'
                }
            }
            KeyCode::KeyP => {
                if modifiers.is_ctrl() {
                    '\u{0010}'
                } else if modifiers.is_caps() {
                    'P'
                } else {
                    'p'
                }
            }
            KeyCode::BracketLeft => {
                if modifiers.is_shifted() {
                    '{'
                } else {
                    '['
                }
            }
            KeyCode::BracketRight => {
                if modifiers.is_shifted() {
                    '}'
                } else {
                    ']'
                }
            }
            KeyCode::Backslash => {
                if modifiers.is_shifted() {
                    '|'
                } else {
                    '\\'
                }
            }
            KeyCode::KeyA => {
                if modifiers.is_ctrl() {
                    '\u{0001}'
                } else if modifiers.is_caps() {
                    'A'
                } else {
                    'a'
                }
            }
            KeyCode::KeyS => {
                if modifiers.is_ctrl() {
                    '\u{0013}'
                } else if modifiers.is_caps() {
                    'S'
                } else {
                    's'
                }
            }
            KeyCode::KeyD => {
                if modifiers.is_ctrl() {
                    '\u{0004}'
                } else if modifiers.is_caps() {
                    'D'
                } else {
                    'd'
                }
            }
            KeyCode::KeyF => {
                if modifiers.is_ctrl() {
                    '\u{0006}'
                } else if modifiers.is_caps() {
                    'F'
                } else {
                    'f'
                }
            }
            KeyCode::KeyG => {
                if modifiers.is_ctrl() {
                    '\u{0007}'
                } else if modifiers.is_caps() {
                    'G'
                } else {
                    'g'
                }
            }
            KeyCode::KeyH => {
                if modifiers.is_ctrl() {
                    '\u{0008}'
                } else if modifiers.is_caps() {
                    'H'
                } else {
                    'h'
                }
            }
            KeyCode::KeyJ => {
                if modifiers.is_ctrl() {
                    '\u{000A}'
                } else if modifiers.is_caps() {
                    'J'
                } else {
                    'j'
                }
            }
            KeyCode::KeyK => {
                if modifiers.is_ctrl() {
                    '\u{000B}'
                } else if modifiers.is_caps() {
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
                } else if modifiers.is_caps() {
                    'L'
                } else {
                    'l'
                }
            }
            KeyCode::Semicolon => {
                if modifiers.is_shifted() {
                    ':'
                } else {
                    ';'
                }
            }
            KeyCode::Quote => {
                if modifiers.is_shifted() {
                    '"'
                } else {
                    '\''
                }
            }
            // Enter gives LF, not CRLF or CR
            KeyCode::Enter => 10.into(),
            KeyCode::KeyZ => {
                if modifiers.is_ctrl() {
                    '\u{001A}'
                } else if modifiers.is_caps() {
                    'Z'
                } else {
                    'z'
                }
            }
            KeyCode::KeyX => {
                if modifiers.is_ctrl() {
                    '\u{0018}'
                } else if modifiers.is_caps() {
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
                } else if modifiers.is_caps() {
                    'C'
                } else {
                    'c'
                }
            }
            KeyCode::KeyV => {
                if modifiers.is_ctrl() {
                    '\u{0016}'
                } else if modifiers.is_caps() {
                    'V'
                } else {
                    'v'
                }
            }
            KeyCode::KeyB => {
                if modifiers.is_ctrl() {
                    '\u{0002}'
                } else if modifiers.is_caps() {
                    'B'
                } else {
                    'b'
                }
            }
            KeyCode::KeyN => {
                if modifiers.is_ctrl() {
                    '\u{000E}'
                } else if modifiers.is_caps() {
                    'N'
                } else {
                    'n'
                }
            }
            KeyCode::KeyM => {
                if modifiers.is_ctrl() {
                    '\u{000D}'
                } else if modifiers.is_caps() {
                    'M'
                } else {
                    'm'
                }
            }
            KeyCode::Comma => {
                if modifiers.is_shifted() {
                    '<'
                } else {
                    ','
                }
            }
            KeyCode::Period => {
                if modifiers.is_shifted() {
                    '>'
                } else {
                    '.'
                }
            }
            KeyCode::Slash => {
                if modifiers.is_shifted() {
                    '?'
                } else {
                    '/'
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
            _k => '\0'
        }
    }
}
