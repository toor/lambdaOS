mod gdt;
use x86::bits64::irq::IdtEntry;
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtualAddress;
use memory::MemoryController;

pub struct Idt {
    table: [IdtEntry; 256],
}

impl Idt {
    pub fn load(&mut self) {
        use x86_64::instructions::tables::{DescriptorTablePointer, lidt};

        let ptr = DescriptorTablePointer {
            base: self.table.as_ptr() as u64,
            limit: (self.table.len() * size_of::<u64>() -1) as u16,
        };
        
        unsafe { lidt(&ptr) };
    }
}
