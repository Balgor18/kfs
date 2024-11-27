#![feature(naked_functions)]
#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![allow(dead_code, static_mut_refs)]

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
mod cpu;
mod terminal;
mod memory;

use core::arch::global_asm;
use core::ptr::addr_of_mut;
use core::{panic::PanicInfo};

// Handle entry
use driver::vga::{Color, Vga};
use memory::directory::PageDirectory;
use terminal::terminal::Terminal;

// Struct for multiboot
use utility::multiboot::{BootInfo, MEMORY_MAP};

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

static mut PDE : PageDirectory = PageDirectory::new();

// fn tmp_process_struct(flags: u32, decalage: u32) -> bool {
//     return flags & decalage != 0;
// }

#[no_mangle]
pub extern "C" fn kernel_main(info : &BootInfo) -> ! {
    unsafe{
        VGA.reset();// Clear terminal
        VGA.putstr(include_str!("42.txt"));
        VGA.putchar('\n' as u8);
        cpu::gdt::init();
    }

    printk!("MemMap :{:?}", info.flags & MEMORY_MAP != 0);

    // if (info.flags & MEMORY_MAP) != 0 {

    unsafe {
        memory::init(& mut PDE);
        memory::activate_pagging(addr_of_mut!(PDE));
    }

    let mut terminal : Terminal = Terminal::new();
    loop {
        terminal.cmd_entry();
    }
}

global_asm!{include_str!("./boot.s"), options(att_syntax)}
