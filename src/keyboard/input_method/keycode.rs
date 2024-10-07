#[non_exhaustive]
pub enum KeyCode {
    Backquote,
    Backslash,
    BracketLeft,
    BracketRight,
    Comma,
    Digit0,
    Digit1,
    Digit2,
    Digit3,
    Digit4,
    Digit5,
    Digit6,
    Digit7,
    Digit8,
    Digit9,
    Equal,
    IntlBackslash,
    IntlRo,
    IntlYen,
    KeyA,
    KeyB,
    KeyC,
    KeyD,
    KeyE,
    KeyF,
    KeyG,
    KeyH,
    KeyI,
    KeyJ,
    KeyK,
    KeyL,
    KeyM,
    KeyN,
    KeyO,
    KeyP,
    KeyQ,
    KeyR,
    KeyS,
    KeyT,
    KeyU,
    KeyV,
    KeyW,
    KeyX,
    KeyY,
    KeyZ,
    Minus,
    Period,
    Quote,
    Semicolon,
    Slash,
    AltLeft,
    AltRight,
    Backspace,
    CapsLock,
    ContextMenu,
    ControlLeft,
    ControlRight,
    Enter,
    SuperLeft,
    SuperRight,
    ShiftLeft,
    ShiftRight,
    Space,
    Tab,
    Convert,
    KanaMode,
    Lang1,
    Lang2,
    Lang3,
    Lang4,
    Lang5,
    NonConvert,
    Delete,
    End,
    Help,
    Home,
    Insert,
    PageDown,
    PageUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    NumLock,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    NumpadAdd,
    NumpadBackspace,
    NumpadClear,
    NumpadClearEntry,
    NumpadComma,
    NumpadDecimal,
    NumpadDivide,
    NumpadEnter,
    NumpadEqual,
    NumpadHash,
    NumpadMemoryAdd,
    NumpadMemoryClear,
    NumpadMemoryRecall,
    NumpadMemoryStore,
    NumpadMemorySubtract,
    NumpadMultiply,
    NumpadParenLeft,
    NumpadParenRight,
    NumpadStar,
    NumpadSubtract,
    Escape,
    Fn,
    FnLock,
    PrintScreen,
    ScrollLock,
    Pause,
    BrowserBack,
    BrowserFavorites,
    BrowserForward,
    BrowserHome,
    BrowserRefresh,
    BrowserSearch,
    BrowserStop,
    Eject,
    LaunchApp1,
    LaunchApp2,
    LaunchMail,
    MediaPlayPause,
    MediaSelect,
    MediaStop,
    MediaTrackNext,
    MediaTrackPrevious,
    Power,
    Sleep,
    AudioVolumeDown,
    AudioVolumeMute,
    AudioVolumeUp,
    WakeUp,
    Meta,
    Hyper,
    Turbo,
    Abort,
    Resume,
    Suspend,
    Again,
    Copy,
    Cut,
    Find,
    Open,
    Paste,
    Props,
    Select,
    Undo,
    Hiragana,
    Katakana,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    F25,
    F26,
    F27,
    F28,
    F29,
    F30,
    F31,
    F32,
    F33,
    F34,
    F35,
}

#[derive(Default)]
pub struct ScancodeSet1 {
    e0 : bool,
    e1 : bool,
}

impl ScancodeSet1 {
    /// Construct a new [`ScancodeSet1`] decoder.
    pub const fn new() -> ScancodeSet1 {
        ScancodeSet1 {
            e0 : false,
            e1 : false,
        }
    }

    /// Implements the single byte codes for Set 1.
    fn map_scancode(code: u8) -> Option<KeyCode> {
        match code {
            0x01 => Some(KeyCode::Escape),
            0x02 => Some(KeyCode::Digit1),
            0x03 => Some(KeyCode::Digit2),
            0x04 => Some(KeyCode::Digit3),
            0x05 => Some(KeyCode::Digit4),
            0x06 => Some(KeyCode::Digit5),
            0x07 => Some(KeyCode::Digit6),
            0x08 => Some(KeyCode::Digit7),
            0x09 => Some(KeyCode::Digit8),
            0x0A => Some(KeyCode::Digit9),
            0x0B => Some(KeyCode::Digit0),
            0x0C => Some(KeyCode::Minus),
            0x0D => Some(KeyCode::Equal),
            0x0E => Some(KeyCode::Backspace),
            0x0F => Some(KeyCode::Tab),
            0x10 => Some(KeyCode::KeyQ),
            0x11 => Some(KeyCode::KeyW),
            0x12 => Some(KeyCode::KeyE),
            0x13 => Some(KeyCode::KeyR),
            0x14 => Some(KeyCode::KeyT),
            0x15 => Some(KeyCode::KeyY),
            0x16 => Some(KeyCode::KeyU),
            0x17 => Some(KeyCode::KeyI),
            0x18 => Some(KeyCode::KeyO),
            0x19 => Some(KeyCode::KeyP),
            0x1A => Some(KeyCode::BracketLeft),
            0x1B => Some(KeyCode::BracketRight),
            0x1C => Some(KeyCode::Enter),
            0x1D => Some(KeyCode::ControlLeft),
            0x1E => Some(KeyCode::KeyA),
            0x1F => Some(KeyCode::KeyS),
            0x20 => Some(KeyCode::KeyD),
            0x21 => Some(KeyCode::KeyF),
            0x22 => Some(KeyCode::KeyG),
            0x23 => Some(KeyCode::KeyH),
            0x24 => Some(KeyCode::KeyJ),
            0x25 => Some(KeyCode::KeyK),
            0x26 => Some(KeyCode::KeyL),
            0x27 => Some(KeyCode::Semicolon),
            0x28 => Some(KeyCode::Quote),
            0x29 => Some(KeyCode::Backquote),
            0x2A => Some(KeyCode::ShiftLeft),
            0x2B => Some(KeyCode::Backslash),
            0x2C => Some(KeyCode::KeyZ),
            0x2D => Some(KeyCode::KeyX),
            0x2E => Some(KeyCode::KeyC),
            0x2F => Some(KeyCode::KeyV),
            0x30 => Some(KeyCode::KeyB),
            0x31 => Some(KeyCode::KeyN),
            0x32 => Some(KeyCode::KeyM),
            0x33 => Some(KeyCode::Comma),
            0x34 => Some(KeyCode::Period),
            0x35 => Some(KeyCode::Slash),
            0x36 => Some(KeyCode::ShiftRight),
            0x37 => Some(KeyCode::NumpadMultiply),
            0x38 => Some(KeyCode::AltLeft),
            0x39 => Some(KeyCode::Space),
            0x3A => Some(KeyCode::CapsLock),
            0x3B => Some(KeyCode::F1),
            0x3C => Some(KeyCode::F2),
            0x3D => Some(KeyCode::F3),
            0x3E => Some(KeyCode::F4),
            0x3F => Some(KeyCode::F5),
            0x40 => Some(KeyCode::F6),
            0x41 => Some(KeyCode::F7),
            0x42 => Some(KeyCode::F8),
            0x43 => Some(KeyCode::F9),
            0x44 => Some(KeyCode::F10),
            0x45 => Some(KeyCode::NumLock),
            0x46 => Some(KeyCode::ScrollLock),
            0x47 => Some(KeyCode::Numpad7),
            0x48 => Some(KeyCode::Numpad8),
            0x49 => Some(KeyCode::Numpad9),
            0x4A => Some(KeyCode::NumpadSubtract),
            0x4B => Some(KeyCode::Numpad4),
            0x4C => Some(KeyCode::Numpad5),
            0x4D => Some(KeyCode::Numpad6),
            0x4E => Some(KeyCode::NumpadAdd),
            0x4F => Some(KeyCode::Numpad1),
            0x50 => Some(KeyCode::Numpad2),
            0x51 => Some(KeyCode::Numpad3),
            0x52 => Some(KeyCode::Numpad0),
            0x53 => Some(KeyCode::NumpadDecimal),
            // 0x54 => Some(KeyCode::AltPrint),
            // 0x55 is unused?
            0x56 => Some(KeyCode::IntlBackslash),
            0x57 => Some(KeyCode::F11),
            0x58 => Some(KeyCode::F12),
            _ => None,
        }
    }

    /// Implements the extended byte codes for set 1 (prefixed with E0)
    fn map_extended_scancode(code: u8) -> Option<KeyCode> {
        match code {
            0x10 => Some(KeyCode::MediaTrackPrevious),
            //0x11
            //0x12
            //0x13
            //0x14
            //0x15
            //0x16
            //0x17
            //0x18
            0x19 => Some(KeyCode::MediaTrackNext),
            //0x1A
            //0x1B
            0x1C => Some(KeyCode::NumpadEnter),
            0x1D => Some(KeyCode::ControlRight),
            //0x1E
            //0x1F
            0x20 => Some(KeyCode::AudioVolumeMute),
            0x21 => Some(KeyCode::LaunchApp2),
            0x22 => Some(KeyCode::MediaPlayPause),
            //0x23
            0x24 => Some(KeyCode::MediaStop),
            //0x25
            //0x26
            //0x27
            //0x28
            //0x29
            0x2A => Some(KeyCode::AltRight),
            //0x2B
            //0x2C
            //0x2D
            0x2E => Some(KeyCode::AudioVolumeDown),
            //0x2F
            0x30 => Some(KeyCode::AudioVolumeUp),
            //0x31
            0x32 => Some(KeyCode::BrowserHome),
            //0x33
            //0x34
            0x35 => Some(KeyCode::NumpadDivide),
            //0x36
            0x37 => Some(KeyCode::PrintScreen),
            0x38 => Some(KeyCode::AltRight),
            //0x39
            //0x3A
            //0x3B
            //0x3C
            //0x3D
            //0x3E
            //0x3F
            //0x40
            //0x41
            //0x42
            //0x43
            //0x44
            //0x45
            //0x46
            0x47 => Some(KeyCode::Home),
            0x48 => Some(KeyCode::ArrowUp),
            0x49 => Some(KeyCode::PageUp),
            //0x4A
            0x4B => Some(KeyCode::ArrowLeft),
            //0x4C
            0x4D => Some(KeyCode::ArrowRight),
            //0x4E
            0x4F => Some(KeyCode::End),
            0x50 => Some(KeyCode::ArrowDown),
            0x51 => Some(KeyCode::PageDown),
            0x52 => Some(KeyCode::Insert),
            0x53 => Some(KeyCode::Delete),
            0x5B => Some(KeyCode::SuperLeft),
            0x5C => Some(KeyCode::SuperRight),
            // 0x5D => Some(KeyCode::Apps),
            // 0x5E ACPI Power
            // 0x5F ACPI Sleep
            // 0x60
            // 0x61
            // 0x62
            // 0x63 ACPI Wake
            // 0x64
            // 0x65 WWW Search
            // 0x66 WWW Favourites
            // 0x67 WWW Refresh
            // 0x68 WWW Stop
            // 0x69 WWW Forward
            // 0x6A WWW Back
            // 0x6B My Computer
            // 0x6C Email
            // 0x6D Media Select
            // 0x70 => Some(KeyCode::Oem11),
            // 0x73 => Some(KeyCode::Oem12),
            // 0x79 => Some(KeyCode::Oem10),
            // 0x7B => Some(KeyCode::Oem9),
            // 0x7D => Some(KeyCode::Oem13),
            _ => None,
        }
    }

    /// Implements the extended byte codes for set 1 (prefixed with E1)
    fn map_extended2_scancode(code: u8) -> Option<KeyCode> {
        match code {
            0x1D => Some(KeyCode::Pause),
            _ => None,
        }
    }
}

pub struct KeyEvent {
    pub key_code : KeyCode,
    pub pressed : bool,
}

impl ScancodeSet1 {
    /// Implements state logic for scancode set 1
    ///
    /// ## Start:
    /// * `E0` => Goto Extended
    /// * `E1` => Goto Extended 2
    /// * `< 0x80` => Key Down
    /// * `>= 0x80` => Key Up
    ///
    /// ## Extended:
    /// * `< 0x80` => Extended Key Down
    /// * `>= 0x80` => Extended Key Up
    ///
    /// ## Extended 2:
    /// * `< 0x80` => Extended 2 Key Down
    /// * `>= 0x80` => Extended 2 Key Up
    pub fn advance_state(&mut self, code: u8) -> Option<KeyEvent> {
        if self.e0 {
            self.e0 = false;
            match code {
                0x80..=0xFF => {
                    // Extended break codes
                    Some(KeyEvent {
                        key_code: Self::map_extended_scancode(code - 0x80)?,
                        pressed: false,
                    })
                }
                _ => {
                    // Extended make codes
                    Some(KeyEvent {
                        key_code: Self::map_extended_scancode(code)?,
                        pressed: true,
                    })
                }
            }
        } else if self.e1 {
            self.e1 = false;
            match code {
                0x80..=0xFF => {
                    // Extended 2 break codes
                    Some(KeyEvent {
                        key_code: Self::map_extended2_scancode(code- 0x80)?,
                        pressed: false,
                    })
                }
                _ => {
                    // Extended 2 make codes
                    Some(KeyEvent {
                        key_code: Self::map_extended2_scancode(code)?,
                        pressed: true,
                    })
                }
            }
        } else {
            match code {
                0xe0 => {
                    self.e0 = true;
                    None
                }
                0xe1 => {
                    self.e1 = true; //DecodeState::Extended2;
                    None
                }
                0x80..=0xFF => {
                    // Break codes
                    Some(KeyEvent {
                        key_code: Self::map_scancode(code - 0x80)?,
                        pressed: false,
                    })
                }
                _ => {
                    // Make codes
                    Some(KeyEvent {
                        key_code: Self::map_scancode(code)?,
                        pressed: true,
                    })
                }
            }
        }
    }
}
