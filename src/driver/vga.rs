
use crate::utility::instruction::outb;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}


/// The width of the VGA buffer.
pub const WIDTH: usize = 80;
/// The height of the VGA buffer.
const HEIGHT: usize = 25;
/// The address of the VGA buffer.
const ADDRESS: *mut u16 = 0xB8000 as *mut u16;
pub struct Vga {
    foreground: Color,
    x: usize,
    y: usize,
}

impl Vga {
    pub const fn new() -> Self {
        // SAFETY: The assertion above ensures that x is in bounds.
        Self {
            foreground: Color::White,
            x: 0,
            y: 0,
        }
    }

    /// # Panics
    /// 
    /// The provided string must not be larger than 80*25.
    pub fn putstr(&mut self, string: &str) {
        for char in string.bytes() {
            self.putchar(char as u8);
        }
    }

    pub fn putchar(&mut self, char: u8) {
        if char == b'\n' {
            assert!(self.x < WIDTH && self.y < HEIGHT, "bad size");
            self.x = 0;
            self.y += 1;
            self.print_cursor();
            return; 
        }

        if !char.is_ascii_graphic() && char != b' '{
            return
        }

        if self.x == WIDTH {
            self.x = 0;
            self.y += 1;
        }
        unsafe {
            assert!(self.x < WIDTH && self.y < HEIGHT, "bad size");
            ADDRESS.add(self.y * WIDTH + self.x).write_volatile(char as u16 | (self.foreground as u16) << 8);
            self.x += 1;
            self.print_cursor();
        }
    }

    fn print_cursor(&mut self) {
        let cursor_pos = self.y * WIDTH + self.x;
        unsafe {
            outb(0x3D4, 0x0F);
            outb(0x3D5, cursor_pos as u8);
            outb(0x3D4, 0x0E);
            outb(0x3D5, (cursor_pos >> 8) as u8);
        }
    }

    pub fn erase_specific_char(&mut self) {
        let value = b' ' as u16 | (self.foreground as u16) << 8;

        if self.x == 0 {
            return 
        }
        self.x -= 1;
        unsafe {
            ADDRESS.add(self.y * WIDTH + self.x).write_volatile(value);
        }
        self.print_cursor();
    }

    #[inline(always)]
    pub fn set_color(&mut self, color: Color) {
        self.foreground = color;
    }

    pub fn reset(&mut self) {
        self.x = 0;
        self.y = 0;
        self.foreground = Color::White;

        let value = b' ' as u16 | (self.foreground as u16) << 8;
        unsafe {
            for i in 0..WIDTH * HEIGHT {
                ADDRESS.add(i).write_volatile(value);
            }
        }
        self.print_cursor();
    }
}