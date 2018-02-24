use acpi::sdt::SdtHeader;

#[derive(Debug)]
pub struct Madt {
    pub sdt: &'static SdtHeader,
    /// Address of LAPIC.
    pub address: u32,
    /// Flags - 1 indicates that dual legacy PICs are installed.
    pub flags: u32,
}

/// The Local APIC.
pub struct LapicEntry {
    /// The ID of the parent AP.
    pub processor_id: u8,
    /// The ID of this APIC.
    pub id: u8,
    /// Flags - 1 means that the AP is enabled.
    pub flags: u32,
}

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
pub struct InterruptSourceOverride {
    pub bus_source: u8,
    pub irq_source: u8,
    pub gsi: u32,
    pub flags: u16
}

/// Non-maskable interrupts.
pub struct ApicNMI {
    pub processor_id: u8,
    pub flags: u16,
    pub lint_no: u8,
}
