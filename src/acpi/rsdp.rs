use arch::memory::Frame;
use arch::memory::paging::{Page, PhysicalAddress, VirtualAddress};
use arch::memory::paging::ActivePageTable;
use arch::memory::paging::entry::EntryFlags;

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
    /// Map RSDP address space, search for RSDP.
    pub fn init(active_table: &mut ActivePageTable) -> Option<Self> {
        // TODO: Search in EBDA as well.

        let rsdp_start: usize = 0xe0000;
        let rsdp_end: usize = 0xf_ffff;

        // Map address space.
        {
            let start_frame = Frame::containing_address(PhysicalAddress::new(rsdp_start));
            let end_frame = Frame::containing_address(PhysicalAddress::new(rsdp_end));

            for frame in Frame::range_inclusive(start_frame, end_frame) {
                let page = Page::containing_address(VirtualAddress::new(frame.start_address().get()));
                let res = active_table.map_to(
                    page,
                    frame,
                    EntryFlags::PRESENT | EntryFlags::NO_EXECUTE, 
                );

                res.flush(active_table);
            }
        }

        RsdpDescriptor::search(rsdp_start, rsdp_end)
    }

    /// Find and parse the RSDP.
    fn search(start_addr: usize, end_addr: usize) -> Option<RsdpDescriptor> {
        for i in 0..(end_addr + 1 - start_addr) / 16 {
            let rsdp = unsafe { &*((start_addr + i * 16) as *const RsdpDescriptor) };
            if &rsdp.signature == b"RSD PTR " {
                println!(
                    "[ acpi ] Found RSDP at {:#x}",
                    rsdp as *const RsdpDescriptor as usize
                );
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
