pub use self::area_frame_allocator::AreaFrameAllocator;
pub use self::paging::ActivePageTable;
pub use self::stack_allocator::Stack;
use self::paging::{PhysicalAddress, VirtualAddress};
use multiboot2::BootInformation;
use spin::Mutex;

pub mod area_frame_allocator;
pub mod heap_allocator;
pub mod paging;
pub mod stack_allocator;

pub const PAGE_SIZE: usize = 4096;

pub static ALLOCATOR: Mutex<Option<AreaFrameAllocator>> = Mutex::new(None);

pub fn init(boot_info: &BootInformation) {
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
    /* MemoryController {
        active_table: active_table,
        frame_allocator: frame_allocator,
        stack_allocator: stack_allocator,
    }*/
}

// TODO: Move this back inside the main memory::init function.
pub fn init_noncore(
    stack_allocator: stack_allocator::StackAllocator,
    active_table: &'static mut paging::ActivePageTable,
) -> MemoryController {
    MemoryController {
        active_table,
        stack_allocator,
    }
}

pub struct MemoryController {
    active_table: &'static mut paging::ActivePageTable,
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
