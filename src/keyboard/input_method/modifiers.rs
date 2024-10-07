#[derive(Default)]
pub struct Modifiers {
    pub lshift: bool,
    pub rshift: bool,
    pub lctrl: bool,
    pub rctrl: bool,
    pub numlock: bool,
    pub capslock: bool,
    pub lalt: bool,
    pub ralt: bool,
    pub rctrl2: bool,
}

impl Modifiers {
    pub const fn is_shifted(&self) -> bool {
        self.lshift | self.rshift
    }

    pub const fn is_ctrl(&self) -> bool {
        self.lctrl | self.rctrl
    }

    pub const fn is_alt(&self) -> bool {
        self.lalt | self.ralt
    }

    pub const fn is_altgr(&self) -> bool {
        self.ralt | (self.lalt & self.is_ctrl())
    }

    pub const fn is_caps(&self) -> bool {
        self.is_shifted() ^ self.capslock
    }
}