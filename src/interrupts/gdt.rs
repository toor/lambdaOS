/* This file contains GDT specific code. It uses stuff from the x86_64 crate, so I recommend
 * reading through the documentation for that to properly understand what is going on.*/

use x86_64::structures::tss::TaskStateSegment;
use x86_64::structures::gdt::SegmentSelector;
use x86_64::PrivilegeLevel;

//8 entries should be more than enough to store all our segments.
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
        let index = match entry {
            Descriptor::UserSegment(value) => self.push(value),
            Descriptor::SystemSegment(value_low, value_high) => {
                let index  = self.push(value_low);
                self.push(value_high);
                index
            }
        };
        SegmentSelector::new(index as u16, PrivilegeLevel::Ring0)
    }

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

    pub fn load(&'static self) {
        use x86_64::instructions::tables::{DescriptorTablePointer, lgdt};
        use core::mem::size_of;

        let ptr = DescriptorTablePointer {
            base: self.table.as_ptr() as u64,
            limit: (self.table.len() * size_of::<u64>() - 1) as u16,
        };

        unsafe { lgdt(&ptr) }; 
    }
}

pub enum Descriptor {
    UserSegment(u64),
    SystemSegment(u64, u64),
}

impl Descriptor {
    //Using flags, we can identify the kernel segment.
    pub fn kernel_code_segment() -> Descriptor {
        //Kernel is in long mode, is a user segment and is and executable code segment
        let flags = DescriptorFlags::USER_SEGMENT | DescriptorFlags::PRESENT | DescriptorFlags::EXECUTABLE | DescriptorFlags::LONG_MODE;
        Descriptor::UserSegment(flags.bits())
    }
    
    //Only system segment we need worry about
    pub fn tss_segment(tss: &'static TaskStateSegment) -> Descriptor {
        use core::mem::size_of;
        use bit_field::BitField;


        let ptr = tss as *const _ as u64;

        let mut low = DescriptorFlags::PRESENT.bits();

        //Base
        low.set_bits(16..40, ptr.get_bits(0..24));
        low.set_bits(56..64, ptr.get_bits(24..32));
        //Limit
        low.set_bits(0..16, (size_of::<TaskStateSegment>() -1) as u64);
        //Type = 0b1001 => available 64-bit tss.
        low.set_bits(0..44, 0b1001);

        let mut high = 0;

        high.set_bits(0..32, ptr.get_bits(32..64));

        Descriptor::SystemSegment(low, high)
    }
}

bitflags! {
    struct DescriptorFlags: u64 {
        const CONFORMING        = 1 << 42;
        const EXECUTABLE        = 1 << 43;
        const USER_SEGMENT      = 1 << 44;
        const PRESENT           = 1 << 45;
        const LONG_MODE         = 1 << 53;
    }
}
