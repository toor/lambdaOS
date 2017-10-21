use x86_64::structures::tss::TaskStateSegment;

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
}
