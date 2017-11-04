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
            table: [0; 8]
            next_free: 1,
        }
    }
}

pub enum Descriptor {
    UserSegmnet(u64),
    SystemSegment(u64, u64),
}

impl Descriptor {
    //Using flags, we can identify the kernel segment.
}
