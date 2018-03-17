use arch::memory::paging::{ActivePageTable, Page, VirtualAddress, PhysicalAddress};
use arch::memory::Frame;
use arch::memory::paging::entry::EntryFlags;
use spin::Mutex;
use alloc::btree_map::BTreeMap;
use core::mem;

pub mod rsdp;
pub mod sdt;
pub mod rsdt;
pub mod xsdt;
pub mod madt;

/// Retrieve an SDT from a pointer found using the RSDP
fn get_sdt(address: usize, active_table: &mut ActivePageTable) -> &'static sdt::SdtHeader {
    {
        let page = Page::containing_address(VirtualAddress::new(address));
        if active_table.translate_page(page).is_none() {
            let frame = Frame::containing_address(PhysicalAddress::new(page.start_address().get()));
            let result = active_table.map_to(page, frame, EntryFlags::PRESENT | EntryFlags::NO_EXECUTE);
            result.flush(active_table);
        }
    }

    let sdt = unsafe { &*(address as *const sdt::SdtHeader) };

    {   
        // Map next page, and all pages within the range occupied by the data table.
        let start_page = Page::containing_address(VirtualAddress::new(address + 4096));
        let end_page = Page::containing_address(VirtualAddress::new(address + sdt.length as usize));
        for page in Page::range_inclusive(start_page, end_page) {
            // Check if this page has already been mapped to a frame.
            if active_table.translate_page(page).is_none() {
                let frame = Frame::containing_address(PhysicalAddress::new(page.start_address().get()));
                let result = active_table.map_to(page, frame, EntryFlags::PRESENT | EntryFlags::NO_EXECUTE);
                result.flush(active_table);
            }
        }
    }

    sdt
}

pub unsafe fn init(active_table: &mut ActivePageTable) {
    let rsdp = rsdp::RsdpDescriptor::init(active_table).expect("Could not find rsdp, aborting ...");
    let sdt = get_sdt(rsdp.sdt(), active_table);
    let rsdt = rsdt::Rsdt::new(sdt);

    println!(
        "[ apci ] Found RSDT at address {:#x}",
        rsdt.sdt as *const sdt::SdtHeader as usize
    );

    println!(
        "[ acpi ] RSDT length {}, data length {}",
        rsdt.sdt.length,
        rsdt.sdt.length as usize - mem::size_of::<sdt::SdtHeader>()
    );

    println!(
        "[ acpi ] RSDT points to {} tables",
        rsdt.other_entries.len()
    );

    // let mut madt: madt::Madt = unsafe { *(&*(0 as *const madt::Madt)) };
    match rsdt.find_sdt(b"APIC") {
        Some(rsdt::TableType::Madt(mut m)) => {
            println!(
                "[ apci ] Found MADT at address {:#x}",
                m.sdt as *const sdt::SdtHeader as usize
            );

            m.init(active_table);
        }
        _ => println!("Could not find MADT."),
    }
}
