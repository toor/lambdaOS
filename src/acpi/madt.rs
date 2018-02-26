use acpi::sdt::SdtHeader;
use arch::memory::paging::ActivePageTable;

#[derive(Debug)]
pub struct Madt {
    pub sdt: &'static SdtHeader,
    /// Address of LAPIC.
    pub address: u32,
    /// Flags - 1 indicates that dual legacy PICs are installed.
    pub flags: u32,
}

impl Madt {
    // TODO - this ought to find all the AP's and register them, and pass control to SMP.
    pub fn init(active_table: &mut ActivePageTable) {
    
    }
    
    pub fn new(sdt: &'static SdtHeader) -> Self {
        let local_address = unsafe { *(sdt.data_address() as *const u32) };
        let flags = unsafe { *(sdt.data_address() as *const u32).offset(1) };

        Madt {
            sdt: sdt,
            address: local_address,
            flags: flags,
        }
    }
}

/// The Local APIC.
#[repr(packed)]
pub struct LapicEntry {
    /// The ID of the parent AP.
    pub processor_id: u8,
    /// The ID of this APIC.
    pub id: u8,
    /// Flags - 1 means that the AP is enabled.
    pub flags: u32,
}

#[repr(packed)]
pub struct IoApic {
    /// The ID of this I/O APIC.
    pub id: u8,
    resv: u8,
    /// Address of this I/O APIC.
    pub address: u32,
    /// The first interrupt number this APIC handles.
    pub gsib: u32,
}

/// Mapping of IRQ source to interrupt.
#[repr(packed)]
pub struct InterruptSourceOverride {
    pub bus_source: u8,
    pub irq_source: u8,
    pub gsi: u32,
    pub flags: u16
}

/// Non-maskable interrupts.
#[repr(packed)]
pub struct ApicNMI {
    pub processor_id: u8,
    pub flags: u16,
    pub lint_no: u8,
}
