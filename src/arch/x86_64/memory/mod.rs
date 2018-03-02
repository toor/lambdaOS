pub use self::area_frame_allocator::AreaFrameAllocator;
pub use self::paging::ActivePageTable;
pub use self::stack_allocator::Stack;
use self::paging::{PhysicalAddress, VirtualAddress};
use multiboot2::BootInformation;

pub mod area_frame_allocator;
pub mod heap_allocator;
pub mod paging;
pub mod stack_allocator;

pub const PAGE_SIZE: usize = 4096;

pub fn init(boot_info: &BootInformation) -> MemoryController {
    assert_has_not_been_called!("memory::init must be called only once");

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

    println!("[ vmm ] Initialising paging...");
    let mut active_table = paging::init(&mut frame_allocator, boot_info);

    use self::paging::Page;
    use self::heap_allocator::{HEAP_SIZE, HEAP_START};

    let heap_start_page = Page::containing_address(VirtualAddress::new(HEAP_START));
    let heap_end_page = Page::containing_address(VirtualAddress::new(HEAP_START + HEAP_SIZE - 1));
    
    println!("[ vmm ] Mapping heap pages.");
    for page in Page::range_inclusive(heap_start_page, heap_end_page) {
        active_table.map(page, paging::EntryFlags::WRITABLE, &mut frame_allocator);
    }

    println!("[ vmm ] Heap start: {:#x}", heap_start_page.start_address().get());
    println!("[ vmm ] Heap end: {:#x}", heap_end_page.start_address().get());

    // Init the heap
    unsafe {
        ::HEAP_ALLOCATOR.init(HEAP_START, HEAP_SIZE);
    }

    let stack_allocator = {
        let stack_alloc_start = heap_end_page + 1;
        let stack_alloc_end = stack_alloc_start + 100;
        let stack_alloc_range = Page::range_inclusive(stack_alloc_start, stack_alloc_end);
        stack_allocator::StackAllocator::new(stack_alloc_range)
    };

    MemoryController {
        active_table: active_table,
        frame_allocator: frame_allocator,
        stack_allocator: stack_allocator,
    }
}

pub struct MemoryController {
    active_table: paging::ActivePageTable,
    frame_allocator: AreaFrameAllocator,
    stack_allocator: stack_allocator::StackAllocator,
}

impl MemoryController {
    pub fn alloc_stack(&mut self, size_in_pages: usize) -> Option<Stack> {
        let &mut MemoryController {
            ref mut active_table,
            ref mut frame_allocator,
            ref mut stack_allocator,
        } = self;
        stack_allocator.alloc_stack(active_table, frame_allocator, size_in_pages)
    }

    /// Get reference to the global frame allocator.
    pub fn frame_allocator(&mut self) -> &mut AreaFrameAllocator {
        &mut self.frame_allocator
    }

    pub fn active_table(&mut self) -> &mut ActivePageTable {
        &mut self.active_table
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    number: usize,
}

impl Frame {
    pub fn containing_address(address: PhysicalAddress) -> Frame {
        Frame {
            number: address.get() / PAGE_SIZE,
        }
    }

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

pub unsafe fn allocator<'a>() -> &'a mut AreaFrameAllocator {
    if let Some(ref mut memory_controller) = ::arch::MEMORY_CONTROLLER {
        return memory_controller.frame_allocator();
    } else {
        panic!("memory controller called before init.");
    }
}

pub unsafe fn active_table<'a>() -> &'a mut ActivePageTable {
    if let Some(ref mut memory_controller) = ::arch::MEMORY_CONTROLLER {
        memory_controller.active_table()
    } else {
        panic!("memory controller called before init.");
    }
}
