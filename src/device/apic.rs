#![allow(unused_imports)]
use x86_64::registers::msr::{rdmsr, wrmsr, IA32_APIC_BASE};
use core::ptr;
use core::sync::atomic::{AtomicU32, Ordering};
use acpi::madt::{IO_APICS};

lazy_static! {
    static ref BASE: AtomicU32 = {
        // Calculate base address.
        let address = rdmsr(IA32_APIC_BASE) & 0xffff0000;
        AtomicU32::new(address as u32)
    };
}

/// Interface to a local APIC.
pub struct LocalApic {
    /// The id of the APIC.
    pub id: u8,
    /// The id of the parent core.
    pub processor_id: u8,
    /// Flags.
    pub flags: u32
}

impl LocalApic {
    /// Read from a register of the Local APIC.
    pub fn lapic_read(&mut self, which_reg: u32) -> u32 {
        let base = BASE.load(Ordering::SeqCst) as u32;
        unsafe {
            ptr::read_volatile(&(base as u32 + which_reg) as *const u32)
        }
    }
    
    /// Write to a register of the Local APIC.
    pub fn lapic_write(&mut self, which_reg: u32, value: u32) {
        let base = BASE.load(Ordering::SeqCst) as u32;
        unsafe {
            ptr::write_volatile(&mut (base + which_reg) as *mut u32, value)
        };
    }
    
    pub fn id(&self) -> u8 {
       self.id
    }

    pub fn ap_id(&self) -> u8 {
       self.processor_id
    }

    pub fn apic_flags(&self) -> u32 {
       self.flags
    }
}

pub struct IoApic {
    pub id: u8,
    _resv: u8,
    pub address: u32,
    pub gsib: u32,
}

impl IoApic {
    /// Get the I/O APIC that handles this GSI.
    pub fn io_apic_from_gsi(gsi: u32) -> Option<usize> {
        for apic in 0 .. IO_APICS.lock().len() {
            if IO_APICS.lock()[apic].gsib < gsi && IO_APICS.lock()[apic].gsib + IoApic::get_max_redirect(apic) > gsi {
                return Some(apic);
            } else {
                continue;
            }
        }

        None
    }
    
    /// Set the redirect for a given IRQ.
    #[allow(exceeding_bitshifts)]
    pub fn set_redirect(irq: u8, gsi: u32, flags: u16, id: u8) {
        let apic = IoApic::io_apic_from_gsi(gsi);
        
        if apic.is_none() {
            println!("[ ERROR ] I/O APIC: Failed to find redirect for IRQ: {}", irq);
            return;
        } else {
            let io_apic = apic.unwrap();

            // Map IRQS: INT48 .. INT64
            let mut redirection: u64 = irq as u64 + 0x30;

            if flags & 2 == 0 {
                redirection |= 1 << 13;
            } else if flags & 8 == 0 {
                redirection |= 1 << 15;
            }

            redirection |= (id as u64) << 56;

            let ioredtbl: u32 = (gsi - IO_APICS.lock()[io_apic].gsib) * 2 + 16;

            IoApic::write(ioredtbl + 0, io_apic, redirection as u32);
            IoApic::write(ioredtbl + 1, io_apic, redirection as u32 >> 32);
        }
    }

    pub fn read(reg: u32, io_apic_num: usize) -> u32 {
        let mut base = IO_APICS.lock()[io_apic_num].address;
        unsafe {
            // Tell IOSEGSEL what register we want to use.
            let val = reg;
            let io_seg_sel = &mut base as *mut u32;
            ptr::write_volatile(io_seg_sel, val);
            
            // Read back from IOREGWIN.
            let io_seg_win = &mut (base + 4) as *mut u32;
            ptr::read_volatile(io_seg_win)
        }
    }

    pub fn write(reg: u32, io_apic_num: usize, data: u32) {
        let mut base = IO_APICS.lock()[io_apic_num].address;
        unsafe {
            let val = reg;
            let io_seg_sel = &mut base as *mut u32;
            ptr::write_volatile(io_seg_sel, val);

            let io_seg_win = &mut (base + 4) as *mut u32;
            ptr::write_volatile(io_seg_win, data);
        }
    }

    pub fn get_max_redirect(io_apic_num: usize) -> u32 {
        (IoApic::read(1, io_apic_num) & 0xff0000) >> 16
    }
}
