use arch::memory::Frame;
use multiboot2::ElfSection;
use arch::memory::paging::PhysicalAddress;

/// A page table entry.
pub struct Entry(u64);

impl Entry {
    /// Check if the entry is used or not.
    pub fn is_unused(&self) -> bool {
        self.0 == 0
    }

    /// Set this entry as unused.
    pub fn set_unused(&mut self) {
        self.0 = 0;
    }

    /// Return the current flags on the page.
    pub fn flags(&self) -> EntryFlags {
        EntryFlags::from_bits_truncate(self.0)
    }

    /// Return the physical frame that this page points to.
    pub fn pointed_frame(&self) -> Option<Frame> {
        if self.flags().contains(EntryFlags::PRESENT) {
            Some(Frame::containing_address(
                PhysicalAddress::new(
                    self.0 as usize & 0x000fffff_fffff000),
            ))
        } else {
            None
        }
    }

    /// Set some flags on an entry.
    pub fn set(&mut self, frame: Frame, flags: EntryFlags) {
        assert!(frame.start_address().get() & !0x000fffff_fffff000 == 0);
        self.0 = (frame.start_address().get() as u64) | flags.bits();
    }
}

bitflags! {
    pub struct EntryFlags: u64 {
        /// Page is present.
        const PRESENT =         1 << 0;
        /// Page is writable-to.
        const WRITABLE =        1 << 1;
        /// Page is accesible from ring-3
        const USER_ACCESSIBLE = 1 << 2;
        /// Write through caching is performed
        /// on this page.
        const WRITE_THROUGH =   1 << 3;
        /// This page should not be cached.
        const NO_CACHE =        1 << 4;
        /// This page has been accessed.
        const ACCESSED =        1 << 5;
        /// This page has been written to.
        const DIRTY =           1 << 6;
        /// Page is a hugepage.
        const HUGE_PAGE =       1 << 7;
        /// This page's address will not be updated in the TLB,
        /// if CR3 is reset.
        const GLOBAL =          1 << 8;
        /// Non-executable page.
        const NO_EXECUTE =      1 << 63;
    }
}

impl EntryFlags {
    /// Parse the flags on an ELF section to our `EntryFlags` struct.
    pub fn from_elf_section_flags(section: &ElfSection) -> EntryFlags {
        use multiboot2::{ELF_SECTION_ALLOCATED, ELF_SECTION_EXECUTABLE, ELF_SECTION_WRITABLE};

        let mut flags = EntryFlags::empty();

        if section.flags().contains(ELF_SECTION_ALLOCATED) {
            // section is loaded to memory
            flags = flags | EntryFlags::PRESENT;
        }
        if section.flags().contains(ELF_SECTION_WRITABLE) {
            flags = flags | EntryFlags::WRITABLE;
        }
        if !section.flags().contains(ELF_SECTION_EXECUTABLE) {
            flags = flags | EntryFlags::NO_EXECUTE;
        }

        flags
    }
}
