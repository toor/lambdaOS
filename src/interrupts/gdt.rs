use x86_64::structures::tss::TaskStateSegment;
use x86_64::structures::gdt::SegmentSelector;
use x86_64::PrivilegeLevel;

/* User segments span the complete address space, and contain only a few flags. They fit into a
 * single GDT Entry.
 * System segments are slightly different. The TSS descriptor for example contains a base address
 * and a limit and thus need more than 64-bits.
 * We store the system segments as two consecutive entries in the GDT.
*/
pub enum Descriptor { 
    UserSegment(u64),
    SystemSegments(u64, u64),
}

impl Descriptor {
    //Create a kernel code segment.
    pub fn kernel_code_segment() {
        let flags = USER_SEGMENT | PRESENT | EXECUTABLE | LONG_MODE;
        Descriptor::UserSegment(flags.bits());
    }

    //Boring old TSS.
    pub fn tss_segment(tss: &'static TaskStateSegment) -> Descriptor {
        use core::mem::size_of;
        use bit_field::BitField;

        let ptr = tss as *const _ as u64;
        
        //Existing TSS.
        let mut low = PRESENT.bits();

        //Set the base of the TSS.
        low.set_bits(16..40, ptr.get_bits(0..24));
        low.set_bits(56..64, ptr.get_bits(24..32));
        //Set limit.
        low.set_bits(0..16, (size_of::<TaskStateSegment>() -1) as u64);
        //Set type 0b1001 (Available and 64-bit TSS.
        low.set_bits(40..44, 0b1001);

        let mut high = 0;
        high.set_bits(0..32, ptr.get_bits(32..64));

        Descriptor::SystemSegment(low, high)
    }
}

//Create a general DescriptorFlags type.
bitflags! {
    struct DescriptorFlags: u64 {
        const CONFORMING        = 1 << 42;
        const EXECUTABLE        = 1 << 43;
        const USER_SEGMENT      = 1 << 44;
        const PRESENT           = 1 << 47;
        const LONG_MODE         = 1 << 53;
    }
}

pub struct Gdt {
    table: [u64; 8],
    next_free: usize,
}

impl Gdt {
    pub fn new() -> Gdt {
        Gdt {
            table: [0; 8],
            next_free: 1,
        }
    }

    pub fn add_entry(&mut self, entry: Descriptor) -> SegmentSelector {
        //Match against the Descriptor type to check how we should populate the GDT.
        let index = match entry {
            Descriptor::UserSegment(value) => self.push(value),
            Descriptor::SystemSegment(value_low, value_high) => {
                let index = self.push(value_low);
                self.push(value_high);
                index
            }
        };

        SegmentSelector::new(index as u16, PrivilegeLevel::Ring0)
    }
    
    //Push entry to the table array.
    fn push(&mut self, value: u64) -> usize {
        if self.next_free < self.table.len() {
            let index = self.next_free;
            self.table[index] = value;
            self.next_free += 1;
            index
        } else {
            panic!("GDT full");
        }
    }
    
    //Present our newly created GDT to the CPU.
    pub fn load(&'static self) {
        use x86_64::instructions::tables::{DescriptorTablePointer, lgdt};
        use core::mem::size_of;

        let ptr = DescriptorTablePointer {
            base: self.table.as_ptr() as u64,
            limit: (self.table.len() * size_of::<u64>() -1) as u16,
        };

        unsafe { lgdt(&ptr) };
    }
}
