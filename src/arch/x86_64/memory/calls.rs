use super::paging::{PhysicalAddress, VirtualAddress};
use super::paging::entry::EntryFlags;
use super::paging::ActivePageTable;
use super::{Frame, FrameAllocator};
use super::paging::Page;

/// Allocate some physical memory.
pub fn physalloc(size: usize) -> Result<usize, &'static str> {
    let allocator = unsafe { super::allocator() };

    allocator
        .allocate_frame((size + 4095) / 4096)
        .ok_or("OOM")
        .map(|frame| frame.start_address())
}

/// Map the physical frames to virtual memory in the range `to + size`.
pub unsafe fn physmap(from: PhysicalAddress, to: VirtualAddress, flags: EntryFlags, size: usize) {
    let mut active_table = ActivePageTable::new();
    let allocator = super::allocator();

    let start_page = Page::containing_address(to);
    let end_page = Page::containing_address(to + size - 1);

    // Map all pages in the calculated range.
    for page in Page::range_inclusive(start_page, end_page) {
        // Knowing that this frame already exists, we can calulate it.
        let frame =
            Frame::containing_address((page.start_address() - to + from) as PhysicalAddress);
        active_table.map_to(page, frame, flags, allocator);
    }

    // TODO: Zero the mapped memory.
}
