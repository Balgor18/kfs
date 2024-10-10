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

/// Installs the kernel's GDT.
///
/// # Safety
///
/// The memory address where the GDT is installed must not currently be in use.
pub unsafe fn init() {
    core::ptr::copy_nonoverlapping(GDT.as_ptr(), START_ADDRESS as *mut u64, 7);

    const GDT_CONFIG : DescriptorTable = DescriptorTable {
        limit: 7 * 8 - 1, //Size: The size of the table in bytes subtracted by 1. 
        base: START_ADDRESS,
    };

    lgdt(&GDT_CONFIG);

    // Reload the data segment registers.
    asm!(
        "
        mov {rgs_tmp:x}, {offset_data}
        mov ds, {rgs_tmp:x}
        mov es, {rgs_tmp:x}
        mov fs, {rgs_tmp:x}
        mov gs, {rgs_tmp:x}
        mov ss, {rgs_tmp:x}
        jmp ${offset_code}, $2f; 2:
        ",
        rgs_tmp = lateout(reg) _, // Create a tmp variable to pass the same CPU register 
        //`reg` means rust take only one register never use before
        // _ This symbols means to drop the variable we don't need it anymore
        offset_data = const DATA_OFFSET,
        offset_code = const CODE_OFFSET, options(att_syntax)
    );
}