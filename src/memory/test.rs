#![no_std]
#![no_main]

/// Constants for page flags
const PAGE_PRESENT: u32 = 0x1;   // Page is present
const PAGE_RW: u32 = 0x2;        // Read/Write
const PAGE_USER: u32 = 0x4;      // User/Supervisor
const PAGE_4MB: u32 = 1 << 7;    // Page Size (4 MiB)

/// Multiboot memory map entry
#[repr(C, packed)]
pub struct MmapEntry {
    base_addr: u64,
    length: u64,
    r#type: u32,
    _reserved: u32,
}

/// Multiboot memory map header
#[repr(C)]
pub struct MmapHeader {
    size: u32,
    addr: u32,
}

/// Page Table Structure
#[repr(C, align(4096))]
pub struct PageTable([u32; 1024]);

impl PageTable {
    pub fn new() -> Self {
        Self([0; 1024])
    }

    /// Map a 4 KiB page
    pub fn map_4kb_page(&mut self, index: usize, physical_address: u32, flags: u32) {
        assert!(physical_address & 0xFFF == 0, "Address must be 4 KiB aligned");
        self.0[index] = physical_address | flags;
    }
}

// Page Directory Structure
#[repr(C, align(4096))]
pub struct PageDirectory([u32; 1024]);

impl PageDirectory {
    pub fn new() -> Self {
        Self([0; 1024])
    }

    /// Map a Page Table in the Page Directory
    pub fn map_page_table(&mut self, index: usize, page_table: &PageTable, flags: u32) {
        let physical_address = page_table as *const _ as u32;
        self.0[index] = physical_address | flags;
    }

    /// Map a 4 MiB page
    pub fn map_4mb_page(&mut self, index: usize, physical_address: u32, flags: u32) {
        assert!(physical_address & 0x003FFFFF == 0, "Address must be 4 MiB aligned");
        self.0[index] = physical_address | flags | PAGE_4MB;
    }
}

/// Process the memory map and set up paging structures
fn setup_paging_with_mmap(mmap_header: &MmapHeader) -> PageDirectory {
    let mut page_directory = PageDirectory::new();
    let mut page_table = PageTable::new();

    // Interpréter les entrées du mmap
    let mmap_entries = unsafe {
        core::slice::from_raw_parts(
            mmap_header.addr as *const MmapEntry,
            (mmap_header.size as usize) / core::mem::size_of::<MmapEntry>(),
        )
    };

    // Parcourir les régions de mémoire
    for entry in mmap_entries {
        if entry.r#type == 1 {
            // Mémoire utilisable
            let base_addr = entry.base_addr as u32;
            let length = entry.length as u32;

            // Si la mémoire est inférieure à 4 MiB, utiliser des pages 4 KiB
            if base_addr < 0x00400000 {
                let start_page = base_addr / 0x1000;
                let end_page = (base_addr + length) / 0x1000;

                // Mapper chaque page 4 KiB
                for page in start_page..end_page {
                    page_table.map_4kb_page(page as usize, (page * 0x1000) as u32, PAGE_PRESENT | PAGE_RW);
                }
            } else {
                // Si la mémoire est supérieure à 4 MiB, utiliser des pages 4 MiB
                let start_page = base_addr / 0x400000;
                let end_page = (base_addr + length) / 0x400000;

                // Mapper chaque page 4 MiB
                for page in start_page..end_page {
                    page_directory.map_4mb_page(page as usize, (page * 0x400000) as u32, PAGE_PRESENT | PAGE_RW);
                }
            }
        }
    }

    // Mapper la Page Table dans le Page Directory
    page_directory.map_page_table(0, &page_table, PAGE_PRESENT | PAGE_RW);

    page_directory
}

/// Kernel entry point
#[no_mangle]
pub extern "C" fn _start(multiboot_info: *const MmapHeader) -> ! {
    let mmap_header = unsafe { &*multiboot_info };
    let _page_directory = setup_paging_with_mmap(mmap_header);
    loop {}
}