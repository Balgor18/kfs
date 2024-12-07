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
    pub fn new() -> Self {
        Self {
            lshift : false,
            rshift : false,
            lctrl : false,
            rctrl : false,
            numlock : false,
            capslock : false,
            lalt : false,
            ralt : false,
            rctrl2 : false,
        }
    }

    #[inline]
    pub const fn is_shifted(&self) -> bool {
        self.lshift | self.rshift
    }

    #[inline]
    pub const fn is_ctrl(&self) -> bool {
        self.lctrl | self.rctrl
    }

    #[inline]
    pub const fn is_alt(&self) -> bool {
        self.lalt | self.ralt
    }

    #[inline]
    pub const fn is_altgr(&self) -> bool {
        self.ralt | (self.lalt & self.is_ctrl())
    }

    #[inline]
    pub const fn is_caps(&self) -> bool {
        self.is_shifted() ^ self.capslock
    }
}