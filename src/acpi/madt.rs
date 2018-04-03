use acpi::sdt::SdtHeader;
use arch::memory::paging::ActivePageTable;
use core::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use core::mem;
use spin::Mutex;
use heapless::Vec as StaticVec;
use alloc::Vec;
use device::{apic, pic};
use raw_cpuid::CpuId;

static CPUS: AtomicUsize = ATOMIC_USIZE_INIT;

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
        let mut local_apics: StaticVec<&'static LapicEntry, [&'static LapicEntry; 20]>
            = StaticVec::new();
        let mut nmis: StaticVec<&'static ApicNMI, [&'static ApicNMI; 10]> = StaticVec::new();
        let mut io_apics: StaticVec<&'static IoApic, [&'static IoApic; 10]> = StaticVec::new();
        let mut isos: StaticVec<&'static InterruptSourceOverride,
                    [&'static InterruptSourceOverride; 10]>
            = StaticVec::new();
        
        let mut apic_manager = apic::ApicManager::new();

        for entry in self.iter() {
            match entry {
                MadtEntry::Lapic(local_apic) => {
                    use x86_64::registers::msr::{rdmsr, IA32_APIC_BASE};

                    // Check if this local APIC corresponds to an active application processor.
                    if local_apic.flags & 1 == 1 {
                        println!(
                            "[ dev ] Found local APIC, id: {}, processor id: {}",
                            local_apic.id, local_apic.processor_id
                        );
                        if rdmsr(IA32_APIC_BASE) & (1 << 8) == local_apic.id as u64 {
                            println!("[ dev ] Found the BSP local APIC, id: {}", local_apic.id);
                        } else {
                            CPUS.fetch_add(1, Ordering::SeqCst);
                        }
                    } 
                    else {
                        println!("Found disabled core, id: {}", local_apic.id);
                    }
                    
                    local_apics.push(local_apic).expect("Failed to push element to static vector");
                }

                MadtEntry::IoApic(io_apic) => {
                    println!(
                        "[ dev ] Found I/O APIC, id: {}, register base: {:#x}, gsib: {}",
                        io_apic.id, io_apic.address, io_apic.gsib
                    );
                    io_apics.push(io_apic).expect("Failed to push element to static vector");
                }

                MadtEntry::Iso(iso) => {
                    println!(
                        "[ dev ] Found interrupt source override,\n overrides IRQ {},\n gsi: {}",
                        iso.irq_source, iso.gsi
                    );
                    isos.push(iso).expect("Failed to push element to static vector");
                }

                MadtEntry::Nmi(nmi) => {
                    println!("[ dev ] APIC NMI with flags: {}, LINT: {}",
                             nmi.flags,
                             nmi.lint_no);
                    nmis.push(nmi).expect("Failed to push element to static vector.");
                }

                _ => {
                    println!("[ acpi ] No more MADT entries...");
                    return;
                }
            }
        }
        
        apic_manager.lapic_base = self.address;
        
        apic_manager.local_apics = local_apics;
        apic_manager.io_apics = io_apics;
        apic_manager.nmis = nmis;
        apic_manager.isos = isos;


        *apic::APIC_MANAGER.lock() = Some(apic_manager);
        
        unsafe { pic::PICS.lock().init() };

        if CpuId::new().get_feature_info().unwrap().has_apic() {
           apic::init(active_table);
        }

        println!("[ smp ] Found {} APs", CPUS.load(Ordering::SeqCst));
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
    _resv: u8,
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
    InvalidNmi(usize),
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
            let len = unsafe { *(self.sdt.data_address() as *const u8).offset(self.i as isize + 1) }
                as usize;

            if self.i + len <= self.sdt.data_len() {
                let item = match ty {
                    0 => if len == mem::size_of::<LapicEntry>() + 2 {
                        MadtEntry::Lapic(unsafe {
                            &*((self.sdt.data_address() + self.i + 2) as *const LapicEntry)
                        })
                    } else {
                        MadtEntry::InvalidLapic(len)
                    },
                    1 => if len == mem::size_of::<IoApic>() + 2 {
                        MadtEntry::IoApic(unsafe {
                            &*((self.sdt.data_address() + self.i + 2) as *const IoApic)
                        })
                    } else {
                        MadtEntry::InvalidIoApic(len)
                    },
                    2 => if len == mem::size_of::<InterruptSourceOverride>() + 2 {
                        MadtEntry::Iso(unsafe {
                            &*((self.sdt.data_address() + self.i + 2)
                                as *const InterruptSourceOverride)
                        })
                    } else {
                        MadtEntry::InvalidIso(len)
                    },
                    4 => if len == mem::size_of::<ApicNMI>() + 2 {
                        MadtEntry::Nmi(unsafe {
                            &*((self.sdt.data_address() + self.i + 2)
                               as *const ApicNMI)
                        })
                    } else {
                        MadtEntry::InvalidNmi(len)
                    },                   
                    _ => MadtEntry::Unknown(ty),
                };

                println!("[ acpi ] MADT entry at address: {:#x}", self.sdt.data_address() + self.i + 2);

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
