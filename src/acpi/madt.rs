use acpi::sdt::SdtHeader;
use arch::memory::paging::ActivePageTable;
use core::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use core::mem;
use spin::Mutex;
use alloc::Vec;

static CPUS: AtomicUsize = ATOMIC_USIZE_INIT;

lazy_static! {
    pub static ref IO_APICS: Mutex<Vec<&'static IoApic>> = Mutex::new(Vec::new());
}

#[derive(Debug, Clone, Copy)]
pub struct Madt {
    pub sdt: &'static SdtHeader,
    /// Address of LAPIC.
    pub address: u32,
    /// Flags - 1 indicates that dual legacy PICs are installed.
    pub flags: u32,
}

impl Madt {
    /// Initialise all the MADT entries.
    pub fn init(&mut self, active_table: &mut ActivePageTable) {
        for entry in self.iter() {
            // TODO Here we should check if the APIC we find is the BSP LAPIC.
            match entry {
                MadtEntry::Lapic(local_apic) => {
                    // New core?
                    if local_apic.flags & 1 == 1 {
                        println!("Found another core, id: {}", local_apic.id);
                        CPUS.fetch_add(1, Ordering::SeqCst);

                        // TODO: smp::init(CPUS.load(Ordering::SeqCst));

                    } else {
                        println!("Found disabled core, id: {}", local_apic.id);
                    }
                },

                MadtEntry::IoApic(io_apic) => {
                    IO_APICS.lock().push(io_apic);  
                },

                _ => {
                    println!("No more entries...");
                    return;
                },
            }
        }
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

    fn iter(&self) -> MadtIter {
        MadtIter {
            sdt: self.sdt,
            i: 8, /* Skip laddr and flags */
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
    pub flags: u16,
}

/// Non-maskable interrupts.
#[repr(packed)]
pub struct ApicNMI {
    pub processor_id: u8,
    pub flags: u16,
    pub lint_no: u8,
}

pub enum MadtEntry {
    Lapic(&'static LapicEntry),
    InvalidLapic(usize),
    IoApic(&'static IoApic),
    InvalidIoApic(usize),
    Iso(&'static InterruptSourceOverride),
    InvalidIso(usize),
    Nmi(&'static ApicNMI),
    Unknown(u8),
}

struct MadtIter {
    sdt: &'static SdtHeader,
    i: usize,
}

impl Iterator for MadtIter {
    type Item = MadtEntry;
    fn next(&mut self) -> Option<Self::Item> {
        if self.i + 1 < self.sdt.data_len() {
            let ty = unsafe { *(self.sdt.data_address() as *const u8).offset(self.i as isize) };
            let len = unsafe { *(self.sdt.data_address() as *const u8).offset(self.i as isize + 1) } as usize;

            if self.i + len <= self.sdt.data_len() {
                let item = match ty {
                    0 => if len == mem::size_of::<LapicEntry>() + 2 {
                        MadtEntry::Lapic(unsafe { &*((self.sdt.data_address() + self.i + 2) as *const LapicEntry) })
                    } else {
                        MadtEntry::InvalidLapic(len)
                    },
                    1 => if len == mem::size_of::<IoApic>() + 2 {
                        MadtEntry::IoApic(unsafe { &*((self.sdt.data_address() + self.i + 2) as *const IoApic) })
                    } else {
                        MadtEntry::InvalidIoApic(len)
                    },
                    2 => if len == mem::size_of::<InterruptSourceOverride>() + 2 {
                        MadtEntry::Iso(unsafe { &*((self.sdt.data_address() + self.i + 2) as *const InterruptSourceOverride) })
                    } else {
                        MadtEntry::InvalidIso(len)
                    },
                    _ => MadtEntry::Unknown(ty)
                };

                self.i += len;

                Some(item)
            } else {
                None
            }
        } else {
            None
        }
    }
}
