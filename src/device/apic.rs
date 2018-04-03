#![allow(unused_imports)]
use x86_64::registers::msr::{rdmsr, wrmsr, IA32_APIC_BASE};
use core::ptr;
use core::sync::atomic::{AtomicU32, Ordering};
use arch::memory::paging::{Page, VirtualAddress, PhysicalAddress, ActivePageTable};
use arch::memory::paging::entry::EntryFlags;
use arch::memory::Frame;
use heapless::Vec as StaticVec;
use spin::Mutex;
use acpi::madt;

/// This will manage all the apic hardware on the system.
pub struct ApicManager {
    /// The base address of the local APIC register space.
    pub lapic_base: u32,
    pub local_apics: StaticVec<&'static madt::LapicEntry, [&'static madt::LapicEntry; 20]>,
    /// All the I/O APICs on a system. FIXME: Figure out how to set the size of the backing
    /// array dynamically.
    pub io_apics: StaticVec<&'static madt::IoApic, [&'static madt::IoApic; 10]>,
    /// All the non-maskable interrupts, specified by the MADT.
    pub nmis: StaticVec<&'static madt::ApicNMI, [&'static madt::ApicNMI; 10]>,
    /// Interrupt source overrides.
    pub isos: StaticVec<&'static madt::InterruptSourceOverride, [&'static madt::InterruptSourceOverride; 10]>,
}

impl ApicManager {
    pub fn new() -> Self {
        ApicManager {
            lapic_base: 0,
            local_apics: StaticVec::new(),
            io_apics: StaticVec::new(),
            nmis: StaticVec::new(),
            isos: StaticVec::new(),
        }
    }

    pub fn lapic_read(&self, register: u32) -> u32 {
        unsafe { ptr::read_volatile(&(self.lapic_base + register) as *const u32) }
    }

    pub fn lapic_write(&self, register: u32, value: u32) {
        unsafe { ptr::write_volatile(&mut (self.lapic_base + register) as *mut u32, value) }
    }

    pub fn lapic_set_nmi(&self, vec: u8, flags: u16, lint: u8) {
        // Set as NMI.
        let mut nmi: u32 = (800 | vec) as u32;
        // Active low.
        if flags & 2 != 0 {
            nmi |= 1 << 13;
        }

        // Level triggered.
        if flags & 8 != 0 {
            nmi |= 1 << 15;
        }
        
        println!("[ dev ] Setting NMI, {:#x}", nmi);

        match lint {
            1 => {
                self.lapic_write(0x360, nmi);
            },
            0 => {
                self.lapic_write(0x350, nmi);
            },
            _ => {},
        }       
    }

    pub fn install_nmis(&self) {
        for (i, nmi) in self.nmis.iter().enumerate() {
            println!("[ dev ] Installing NMI {}, vector offset: {:#x}", i, 0x90 + i);
            println!("[ dev ] NMI has flags: {}, using register LVT{}", nmi.flags, nmi.lint_no);
            self.lapic_set_nmi(0x90 + i as u8, nmi.flags, nmi.lint_no);
        }
    }
    
    /// Enable the Local APIC and set the spurious interrupt vector to 0xff, 255.
    pub fn lapic_enable(&self) {
        let read = self.lapic_read(0xf0);
        self.lapic_write(0xf0, read | (0x100 | 0xff));
    }

    pub fn io_apic_read(&self, reg: u32, num: usize) -> u32 {
        // First, find the base address of the I/O APIC referenced by `num`
        // in our list of entries.
        let mut addr: u32 = self.io_apics[num].address;

        unsafe {
            let val = reg;
            let ioregsel = &mut addr as *mut u32;
            // Tell the apic which register we which to use.
            ptr::write_volatile(ioregsel, val);

            let ioregwin = &mut (addr + 4) as *mut u32;
            ptr::read_volatile(ioregwin)
        }
    }

    pub fn io_apic_write(&self, reg: u32, num: usize, data: u32) {
        let mut addr: u32 = self.io_apics[num].address;

        unsafe {
            let val = reg;
            let ioregsel = &mut addr as *mut u32;
            ptr::write_volatile(ioregsel, val);
            
            let ioregwin = &mut (addr + 4) as *mut u32;
            ptr::write_volatile(ioregwin, data);
        };
    }

    pub fn io_apic_from_gsi(&self, gsi: u32) -> Option<usize> {
        for (i, apic) in self.io_apics.iter().enumerate() {
            if apic.gsib < gsi && apic.gsib + self.get_max_redirect(i) > gsi {
                return Some(i);
            } else {
                continue;
            }
        }

        None
    }

    pub fn get_max_redirect(&self, num: usize) -> u32 {
        (self.io_apic_read(1, num) & 0xff0000) >> 16
    }
     
    /// Set the redirect for a given IRQ and GSI.
    pub fn set_redirect(&self, irq: u8, gsi: u32, flags: u16, id: u8) {
        let apic = self.io_apic_from_gsi(gsi);

        if apic.is_none() {
            println!("[ apic ] Error: Could not find an I/O APIC that handles GSI: {}", gsi);
            // return;
        } else {
            let io_apic = apic.unwrap();

            let mut redirection: u64 = irq as u64 + 0x30;
            if flags & 2 != 0 {
                redirection |= 1<<13;
            } else if flags & 8 != 0 {
                redirection |= 1<<15;
            }

            redirection |= (id as u64) << 56;

            let ioredtbl: u32 = (gsi - self.io_apics[io_apic].gsib) * 2 + 16;
            
            println!("[ dev ] Redirecting IRQ {}, redirection data: {}", irq, redirection);

            self.io_apic_write(ioredtbl, io_apic, redirection as u32);
            self.io_apic_write(ioredtbl + 1, io_apic, redirection as u32);
        }
    }

    pub fn install_redirects(&self) {
        for iso in self.isos.iter() {
            self.set_redirect(iso.irq_source, iso.gsi, iso.flags, self.local_apics[0].id)
        }
    }

    pub fn eoi(&self) {
        self.lapic_write(0xb0, 0);
    }
}

pub fn init(active_table: &mut ActivePageTable) {
    if let Some(ref mut apic_manager) = *APIC_MANAGER.lock() {
        println!("[ dev ] Initialising APIC, lapic base at {:#x}", apic_manager.lapic_base);
        println!("[ dev ] Mapping local APIC address space...");
        
        for (i, _) in apic_manager.io_apics.iter().enumerate() {
            println!("Max redirect for this i/o apic is {}", apic_manager.get_max_redirect(i));
        }

        {
            let page = Page::containing_address(VirtualAddress::new(apic_manager.lapic_base as usize));
            let frame = Frame::containing_address(PhysicalAddress::new(apic_manager.lapic_base as usize));
            let result = active_table.map_to(page, frame,
                                             EntryFlags::PRESENT |
                                             EntryFlags::WRITABLE |
                                             EntryFlags::NO_EXECUTE);
            result.flush(active_table);
        }

        {
            for io_apic in apic_manager.io_apics.iter() {
                let page = Page::containing_address(VirtualAddress::new(io_apic.address as usize));
                let frame = Frame::containing_address(PhysicalAddress::new(io_apic.address as usize));
                let result = active_table.map_to(page, frame,
                                                 EntryFlags::PRESENT |
                                                 EntryFlags::WRITABLE |
                                                 EntryFlags::NO_EXECUTE);
                result.flush(active_table);
            }
        }

        println!("[ dev ] Installing non-maskable interrupts...");
        apic_manager.install_nmis();
        println!("[ dev ] Installing interrupt source overrides...");
        apic_manager.install_redirects();
        println!("[ dev ] Enabling Local APIC");
        apic_manager.lapic_enable();
    }
}

pub fn eoi() {
    if let Some(ref mut apic_manager) = *APIC_MANAGER.lock() {
        apic_manager.eoi();
    } else {
        panic!("apic not initialised");
    }
}

lazy_static! {
    pub static ref APIC_MANAGER: Mutex<Option<ApicManager>> = Mutex::new(None);
}
