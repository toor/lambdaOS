use arch::memory::paging::{ActivePageTable, Page, VirtualAddress};
use arch::memory::Frame;
use arch::memory::paging::entry::EntryFlags;
use arch::memory::allocator;
use spin::Mutex;
use alloc::btree_map::BTreeMap;
use alloc::String;
use core::str;
use core::mem;

pub mod rsdp;
pub mod sdt;
pub mod rsdt;
pub mod xsdt;
pub mod madt;

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

    let rsdt = rsdt::Rsdt::new(sdt);
    println!("[ OK ] ACPI: Found RSDT at address {:#x}", rsdt.sdt as *const sdt::SdtHeader as usize);

    println!("[ OK ] ACPI: RSDT length {}, data length {}", rsdt.sdt.length, rsdt.sdt.length as usize - mem::size_of::<sdt::SdtHeader>());

    println!("[ DEBUG ] ACPI: RSDT points to {} tables", rsdt.other_entries.len());

    let mut madt: madt::Madt;
    
    match rsdt.find_sdt(b"APIC") {
        Some(rsdt::TableType::Madt(m)) => {
            println!(
            "[ OK ] ACPI: Found MADT at address {:#x}", 
            m.sdt as *const sdt::SdtHeader as usize);

            madt = m
        },
        _ => println!("Could not find MADT."),
    }
}
