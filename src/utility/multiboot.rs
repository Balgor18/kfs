#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct BootInfo {

    pub flags: u32,// 0 -- 4

    pub useless_info : [u32; 10], // 4 -- 44

    pub mmap_length: u32, // 44 -- 48
    pub mmap_addr: * mut MmapInfo, // 48 -- 52

    // pub mmap_lentgh: * mut MmapInfo,
    // pub mmap_addr: u32,
    // pub mem_lower: u32,
    // pub mem_upper: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MmapInfo {
    pub size: u32, // 0 4         -4 -- 0
    pub base_addr: u64, // 4 12      0 -- 8
    pub length: u64, // 12 20    8 -- 16
    pub type_: u32, // 24     ?????
    // pub size: u32,
}

// Used for the flags of BootInfo structure
pub const MEMORY : u32 = 1 << 0;
pub const BOOT_DEVICE : u32 = 1 << 1;
pub const CMDLINE : u32 = 1 << 2;
pub const MODULES : u32 = 1 << 3;
pub const MEMORY_MAP : u32 = 1 << 6;
pub const BOOTLOADER_NAME : u32 = 1 << 9;
