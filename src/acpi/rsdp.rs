use rlibc::memcmp;
use arch::memory::Frame;
use arch::memory::paging::{Page, PhysicalAddress, VirtualAddress};
use arch::memory::paging::ActivePageTable;
use arch::memory::allocator;
use arch::memory::paging::entry::EntryFlags;
use alloc::String;

#[derive(Copy, Clone, Debug)]
#[repr(packed)]
pub struct RsdpDescriptor {
    signature: [u8; 8],
    checksum: u8,
    oem_id: [u8; 6],
    pub revision: u8,
    rsdt_address: u32,
    length: u32,
    xsdt_address: u64,
    extended_checksum: u8,
    reserved: [u8; 3],
}

impl RsdpDescriptor {
    /// Search for the RSDP and return if found.
    pub fn init(active_table: &mut ActivePageTable) -> Option<Self> {
        let rsdp_start: usize = 0xe0000;
        let rsdp_end: usize = 0xf_ffff;
        
        let allocator = unsafe { allocator() };

        // Map all of the address space we search in.
        {
            let start_frame = Frame::containing_address(rsdp_start as PhysicalAddress);
            let end_frame = Frame::containing_address(rsdp_end as PhysicalAddress);

            for frame in Frame::range_inclusive(start_frame, end_frame) {
                let page = Page::containing_address(frame.start_address() as usize as VirtualAddress);
                active_table.map_to(page, frame, EntryFlags::PRESENT | EntryFlags::NO_EXECUTE, allocator);
            }
        }

        RsdpDescriptor::search(rsdp_start, rsdp_end)
    }
    
    fn search(start_addr: usize, end_addr: usize) -> Option<RsdpDescriptor> {
        for i in 0 .. (end_addr + 1 - start_addr)/16 {
            let rsdp = unsafe { &*((start_addr + i * 16) as *const RsdpDescriptor) };
            if &rsdp.signature == b"RSD PTR " {
                return Some(*rsdp);
            }
        }

        None
    }

    /// Dependent on ACPI version, return the address of the XSDT/RSDT.
    pub fn sdt(&self) -> usize {
        if self.revision >= 2 {
            self.xsdt_address as usize
        } else {
            self.rsdt_address as usize
        }
    }
}
