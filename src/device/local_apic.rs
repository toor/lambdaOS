//! Code for the BSP (bootstrap processor)'s local APIC.

use x86_64::registers::msr::{rdmsr, wrmsr, IA32_APIC_BASE};
use arch::memory::paging::{ActivePageTable, PhysicalAddress};
use arch::memory::paging::entry::EntryFlags;
use arch::memory::allocator;

#[derive(Debug)]
pub struct LocalApic {
    /// Base address of the LAPIC registers.
    address: usize,
    /// Whether this device is an X2APIC or not.
    is_x2: bool,
}

impl LocalApic {
    /// Initialise the APIC. `address` is derived from the possible 64-bit APIC override address,
    /// specified in the MADT, that exists on newer long mode CPUs.
    pub fn init(&mut self, active_table: &mut ActivePageTable, address: Option<usize>) {
        if let Some(addr) = address {
            self.address = addr;
        } else {
            // Fallback to address read from MSR.
            self.address = rdmsr(IA32_APIC_BASE);
        }

        // TODO: Check X2APIC feature.

        let allocator = unsafe {
            allocator()
        };
        
        // APIC registers are mapped in physical page 0xFE*****, identity map this physical page.
        {
            let frame = Frame::containing_address(self.address as PhysicalAddress);
            active_table.identity_map(frame, EntryFlags::PRESENT | EntryFlags::WRITABLE, allocator);
        }
    }
}

pub static mut LOCAL_APIC: LocalApic = LocalApic {
    address: 0,
    is_x2: false,
};

pub fn apic_init(address: usize) {
    LOCAL_APIC.init();
}
