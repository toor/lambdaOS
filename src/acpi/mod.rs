use arch::memory::paging::{ActivePageTable, Page, VirtualAddress};
use arch::memory::Frame;
use arch::memory::paging::entry::EntryFlags;
use arch::memory::allocator;

pub mod rsdp;
pub mod sdt;
pub mod rsdt;
pub mod xsdt;

/// Retrieve an SDT from a pointer found using the RSDP
fn get_sdt(address: usize, active_table: &mut ActivePageTable) -> &'static sdt::SdtHeader {
    let allocator = unsafe { allocator() };

    {
        let frame = Frame::containing_address(address);
        active_table.identity_map(frame, EntryFlags::PRESENT | EntryFlags::NO_EXECUTE, allocator);
    }

    // Cast physical address to usable object.
    let sdt = unsafe { &*(address as *const sdt::SdtHeader) };

    sdt
}

pub unsafe fn init(active_table: &mut ActivePageTable) {
    let rsdp = rsdp::RsdpDescriptor::init(active_table).expect("Could not find rsdp, aborting ...");

    let sdt = get_sdt(rsdp.sdt(), active_table);

    if rsdp.revision >= 2 {
        println!("[ OK ] ACPI: Found XSDT at {:#x}", rsdp.sdt());
    } else {
        println!("[ OK ] ACPI: Found RSDT at {:#x}", rsdp.sdt());
    }
}
