pub use self::area_frame_allocator::AreaFrameAllocator;
pub use self::paging::ActivePageTable;
pub use self::stack_allocator::Stack;
use self::paging::{PhysicalAddress, VirtualAddress};
use self::paging::entry::EntryFlags;

use multiboot2::BootInformation;
use spin::Mutex;

pub mod area_frame_allocator;
pub mod heap_allocator;
pub mod paging;
pub mod stack_allocator;

/// The size of a physical page on x86.
pub const PAGE_SIZE: usize = 4096;

/// The global physical page frame allocator.
pub static ALLOCATOR: Mutex<Option<AreaFrameAllocator>> = Mutex::new(None);

pub fn init(boot_info: &BootInformation) -> MemoryController {
    assert_has_not_been_called!("memory::init must be called only once");

    let memory_map_tag = boot_info.memory_map_tag().expect("Memory map tag required");
    let elf_sections_tag = boot_info
        .elf_sections_tag()
        .expect("Elf sections tag required");

    let kernel_start = elf_sections_tag
        .sections()
        .filter(|s| s.is_allocated())
        .map(|s| s.start_address())
        .min()
        .unwrap();
    let kernel_end = elf_sections_tag
        .sections()
        .filter(|s| s.is_allocated())
        .map(|s| s.start_address() + s.size())
        .max()
        .unwrap();

    println!(
        "[ pmm ] Kernel start: {:#x}, kernel end: {:#x}",
        kernel_start, kernel_end
    );
    println!(
        "[ pmm ] Multiboot data structure start: {:#x}, end: {:#x}",
        boot_info.start_address(),
        boot_info.end_address()
    );

    let mut frame_allocator = AreaFrameAllocator::new(
        kernel_start as usize,
        kernel_end as usize,
        boot_info.start_address(),
        boot_info.end_address(),
        memory_map_tag.memory_areas(),
    );

    *ALLOCATOR.lock() = Some(frame_allocator);
    
    let mut active_table = paging::init(&boot_info);

    use self::paging::Page;
    use self::heap_allocator::{HEAP_START, HEAP_SIZE};

    let heap_start_page = Page::containing_address(VirtualAddress::new(HEAP_START));
    let heap_end_page = Page::containing_address(VirtualAddress::new(HEAP_START + HEAP_SIZE - 1));

    println!("[ vmm ] Mapping heap pages ...");

    for page in Page::range_inclusive(heap_start_page, heap_end_page) {
        active_table.map(page, EntryFlags::PRESENT | EntryFlags::WRITABLE);
    }

    unsafe {
        ::HEAP_ALLOCATOR.init(HEAP_START, HEAP_SIZE)
    };

    let stack_allocator = {
        let stack_start_page = heap_end_page + 1;
        let stack_end_page = stack_start_page + 100;
        let stack_alloc_range = Page::range_inclusive(stack_start_page, stack_end_page);
        stack_allocator::StackAllocator::new(stack_alloc_range)
    };

    MemoryController {
        active_table: active_table,
        stack_allocator: stack_allocator,
    }
}

pub struct MemoryController {
    active_table: paging::ActivePageTable,
    stack_allocator: stack_allocator::StackAllocator,
}

impl MemoryController {
    pub fn alloc_stack(&mut self, size_in_pages: usize) -> Option<Stack> {
        let &mut MemoryController {
            ref mut active_table,
            ref mut stack_allocator,
        } = self;
        stack_allocator.alloc_stack(active_table, size_in_pages)
    }

    /* pub fn allocate_frame(&mut self, count: usize) -> Option<Frame> {
        let &mut MemoryController {
            ref mut active_table,
            ref mut frame_allocator,
            ref mut stack_allocator,
        } = self;

        frame_allocator.allocate_frame(count)
    } */
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    number: usize,
}

impl Frame {
    /// Return the frame that contains the given physical address.
    pub fn containing_address(address: PhysicalAddress) -> Frame {
        Frame {
            number: address.get() / PAGE_SIZE,
        }
    }
    
    /// Return the starting address of this frame.
    pub fn start_address(&self) -> PhysicalAddress {
        PhysicalAddress::new(self.number * PAGE_SIZE)
    }

    fn clone(&self) -> Frame {
        Frame {
            number: self.number,
        }
    }

    pub fn range_inclusive(start: Frame, end: Frame) -> FrameIter {
        FrameIter {
            start: start,
            end: end,
        }
    }
}

pub struct FrameIter {
    start: Frame,
    end: Frame,
}

impl Iterator for FrameIter {
    type Item = Frame;

    fn next(&mut self) -> Option<Frame> {
        if self.start <= self.end {
            let frame = self.start.clone();
            self.start.number += 1;
            Some(frame)
        } else {
            None
        }
    }
}

pub trait FrameAllocator {
    fn allocate_frame(&mut self, count: usize) -> Option<Frame>;
    fn deallocate_frame(&mut self, frame: Frame);
    fn free_frames(&mut self) -> usize;
}

/// Allocate a frame.
pub fn allocate_frames(count: usize) -> Option<Frame> {
    if let Some(ref mut frame_allocator) = *ALLOCATOR.lock() {
        return frame_allocator.allocate_frame(count);
    } else {
        panic!("Frame allocator called before init.");
    }
}
