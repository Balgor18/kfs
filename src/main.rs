#![feature(naked_functions)]
#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![allow(dead_code)]

macro_rules! printk {
    ($($args:tt)*) => {
        unsafe {
            use core::fmt::Write;
            writeln!(crate::VGA, $($args)*).unwrap();
        }
    }
}

mod driver;
mod keyboard;
mod utility;
use core::arch::global_asm;
use core::{panic::PanicInfo};
use keyboard::keyboard::Keyboard;

// Handle entry
use driver::vga::{Color, Vga};
use driver::ps2::wait_for_next_scancode;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    unsafe {
        VGA.reset();
        VGA.set_color(Color::Red);
    }
    printk!("{info}");
    loop {}
}

static mut VGA: Vga = Vga::new();

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    unsafe{
        VGA.reset();// Clear terminal
        VGA.putstr(include_str!("42.txt"));
        VGA.putchar('\n' as u8);
    }
    let mut keyboard = Keyboard::default();
    loop {
        let scancode = wait_for_next_scancode();
        if let Some(c) = keyboard.scancode_to_char(scancode) {
            unsafe{
                VGA.putchar(c as u8);
            }
        }

    }
}

global_asm!{include_str!("./boot.s"), options(att_syntax)}
