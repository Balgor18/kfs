
use core::fmt::Write;

use super::vga::Vga;

impl Write for Vga {

    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.putstr(s);
        Ok(())
    }
}




