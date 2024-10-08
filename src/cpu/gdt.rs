use core::arch::asm;

use crate::utility::instruction::lgdt;

const START_ADDRESS: u64 = 0x800;

// /// The offset of the kernel data segment within the kernel's GDT.
pub const DATA_OFFSET: u16 = 0x10;
// /// The offset of the kernel code segment within the kernel's GDT.
pub const CODE_OFFSET: u16 = 0x08;

/// The GDT that will be copied and loaded.
const GDT: [u64; 7] = [
    // Null Segment
    0u64,
    // Kernel Mode Code Segment
    0x00cf9a000000ffff,
    // Kernel Mode Data Segment
    0x00cf92000000ffff,
    // Kernel Mode Stack Segment
    0x00cf92000000ffff,
    // User Mode Code Segment
    0x00cffa000000ffff,
    // User Mode Data Segment
    0x00cff2000000ffff,
    // User Mode Stack Segment
    0x00cff2000000ffff,
];

#[derive(Debug, Clone, Copy)]
#[repr(packed, C)]
pub struct DescriptorTable {
    pub limit: u16,
    pub base: u64,
}

// unsafe impl Send for DescriptorTable {}
// unsafe impl Sync for DescriptorTable {}

const GDT_CONFIG : DescriptorTable = DescriptorTable {
    limit: 7 * 8 - 1, //Size: The size of the table in bytes subtracted by 1. 
    base: START_ADDRESS,
};

/// Installs the kernel's GDT.
///
/// # Safety
///
/// The memory address where the GDT is installed must not currently be in use.
pub unsafe fn init() {
    core::ptr::copy_nonoverlapping(GDT.as_ptr(), START_ADDRESS as *mut u64, 7);

    lgdt(&GDT_CONFIG);

    // Reload the data segment registers.
    asm!(
        "
        mov {tmp:x}, {offset_data}
        mov ds, {tmp:x}
        mov es, {tmp:x}
        mov fs, {tmp:x}
        mov gs, {tmp:x}
        mov ss, {tmp:x}
        ",
        tmp = lateout(reg) _, 
        offset_data = const DATA_OFFSET, options(preserves_flags, nostack, nomem)
    );

    asm!(
        "jmp ${offset_code}, $2f; 2:",
        offset_code = const CODE_OFFSET, options(att_syntax)
    );
}