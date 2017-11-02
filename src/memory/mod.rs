pub use self::area_frame_allocator::AreaFrameAllocator;
pub use self::paging::remap_the_kernel;
use self::paging::{ActivePageTable, Page, PhysicalAddress, VirtualAddress};
use super::allocator;
pub use self::stack_allocator::Stack;
use multiboot2::BootInformation;
use alloc::boxed::Box;

mod area_frame_allocator;
pub mod paging;
mod stack_allocator;

pub const PAGE_SIZE: usize = 4096;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    number: usize,
}

impl Frame {
    fn containing_address(address: usize) -> Frame {
        Frame {
            number: address / PAGE_SIZE,
        }
    }

    fn start_address(&self) -> PhysicalAddress {
        self.number * PAGE_SIZE
    }

    fn clone(&self) -> Frame {
        Frame {
            number: self.number,
        }
    }

    fn range_inclusive(start: Frame, end: Frame) -> FrameIter {
        FrameIter {
            start: start,
            end: end,
        }
    }
}

struct FrameIter {
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
    fn allocate_frame(&mut self) -> Option<Frame>;
    fn deallocate_frame(&mut self, frame: Frame);
}

pub struct MemoryController {
    stack_allocator: stack_allocator::StackAllocator,
}

impl MemoryController {
    pub fn alloc_stack(&mut self, size_in_pages: usize) -> Option<Stack> {
        let &mut MemoryController {
            ref mut stack_allocator,
        } = self;
        stack_allocator.alloc_stack(size_in_pages)
    }
}

static mut MEMORY_CONTROLLER: Option<&'static mut MemoryController> = None;
static mut ACTIVE_TABLE_PTR: Option<&'static mut ActivePageTable> = None;
static mut AREA_FRAME_ALLOCATOR_PTR: Option<&'static mut AreaFrameAllocator> = None;

pub fn area_frame_allocator() -> &'static mut AreaFrameAllocator {
    unsafe {
        match AREA_FRAME_ALLOCATOR_PTR {
            Some(ref mut a) => a,
            None => {
                panic!("frame_allocator called before init");
            }
        }
    }
}

pub fn memory_controller() -> &'static mut MemoryController {
    unsafe {
        match MEMORY_CONTROLLER {
            Some(ref mut a) => a,
            None => {
                panic!("stack allocator called before initializing");
            }
        }
    }
}

pub fn page_table() -> &'static mut ActivePageTable {
    unsafe {
        match ACTIVE_TABLE_PTR {
            Some(ref mut a) => a,
            None => {
                panic!("active page table called before init");
            }
        }
    }
}

pub fn init(boot_info: &BootInformation) {
    assert_has_not_been_called!("memory::init must only be called once");

    let memory_map_tag = boot_info.memory_map_tag().expect("Memory map tag required");
    let elf_sections_tag = boot_info
        .elf_sections_tag()
        .expect("Elf sections tag required");

    let kernel_start = elf_sections_tag
        .sections()
        .filter(|s| s.is_allocated())
        .map(|s| s.addr)
        .min()
        .unwrap();
    let kernel_end = elf_sections_tag
        .sections()
        .filter(|s| s.is_allocated())
        .map(|s| s.addr + s.size)
        .max()
        .unwrap();


    println!(
        "kernel start: 0x{:x}, kernel end: 0x{:x}",
        kernel_start,
        kernel_end
    );
    println!(
        "multiboot start: 0x{:x}, multiboot end: 0x{:x}",
        boot_info.start_address(),
        boot_info.end_address()
    );

    let mut allocator = AreaFrameAllocator::new(
        kernel_start as usize,
        kernel_end as usize,
        boot_info.start_address(),
        boot_info.end_address(),
        memory_map_tag.memory_areas(),
    );

    let mut active_table = paging::remap_the_kernel(&mut allocator, boot_info);

    use self::paging::Page;

    use allocator::{HEAP_SIZE, HEAP_START};

    let heap_start_page = Page::containing_address(HEAP_START);
    let heap_end_page = Page::containing_address(HEAP_START + HEAP_SIZE - 1);
    
    use self::paging::entry::EntryFlags;

    for page in Page::range_inclusive(heap_start_page, heap_end_page) {
        active_table.map(page, EntryFlags::WRITABLE, &mut allocator);
    }

    unsafe {
        allocator::init(HEAP_START, HEAP_SIZE);
    }

    unsafe {
        AREA_FRAME_ALLOCATOR_PTR = Some(&mut *Box::into_raw(Box::new(allocator)));
    }

    unsafe { ACTIVE_TABLE_PTR = Some(&mut *Box::into_raw(Box::new(active_table))) }

    let stack_allocator = {
        let stack_alloc_start = heap_end_page + 1;
        let stack_alloc_end = stack_alloc_start + 10000;
        let stack_alloc_range = Page::range_inclusive(stack_alloc_start, stack_alloc_end);
        stack_allocator::StackAllocator::new(stack_alloc_range)
    };

    let mc = MemoryController {
        stack_allocator: stack_allocator,
    };

    unsafe { MEMORY_CONTROLLER = Some(&mut *Box::into_raw(Box::new(mc))) }
}
