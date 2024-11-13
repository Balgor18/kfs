
use core::arch::asm;

use crate::cpu::gdt::DescriptorTable;

pub unsafe fn outb(port: u16, value: u8){
	asm!("out dx, al", in("dx") port, in("al") value);
}

pub unsafe fn inb(port: u16) -> u8{
	let value : u8;
	asm!("in al, dx", in("dx") port, out("al") value);
	return value;
}

#[inline(always)]
pub unsafe fn lgdt(gdt: &DescriptorTable) {
    unsafe {
        asm!("lgdt [{}]", in(reg) gdt);
    }
}