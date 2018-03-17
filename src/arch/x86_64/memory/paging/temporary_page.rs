use super::{ActivePageTable, Page, VirtualAddress};
use super::table::{Level1, Table};
use arch::memory::Frame;

pub struct TemporaryPage {
    page: Page,
}

impl TemporaryPage {
    pub fn new(page: Page) -> TemporaryPage {
        TemporaryPage { page: page }
    }

    /// Maps the temporary page to the given frame in the active table.
    /// Returns the start address of the temporary page.
    pub fn map(&mut self, frame: Frame, active_table: &mut ActivePageTable) -> VirtualAddress {
        use super::entry::EntryFlags;

        assert!(
            active_table.translate_page(self.page).is_none(),
            "temporary page is already mapped"
        );
        let result = active_table.map_to(self.page, frame, EntryFlags::WRITABLE);
        result.flush(active_table);
        self.page.start_address()
    }

    /// Maps the temporary page to the given page table frame in the active table.
    /// Returns a reference to the now mapped table.
    pub fn map_table_frame(
        &mut self,
        frame: Frame,
        active_table: &mut ActivePageTable,
    ) -> &mut Table<Level1> {
        unsafe { &mut *(self.map(frame, active_table).get() as *mut Table<Level1>) }
    }

    /// Unmaps the temporary page in the active table.
    pub fn unmap(&mut self, active_table: &mut ActivePageTable) {
        let result = active_table.unmap(self.page);
        result.flush(active_table);
    }
}

/* struct TinyAllocator([Option<Frame>; 3]);

impl TinyAllocator {
    fn new<A>(allocator: &mut A) -> TinyAllocator
    where
        A: FrameAllocator,
    {
        let mut f = || allocator.allocate_frame(1);
        let frames = [f(), f(), f()];
        TinyAllocator(frames)
    }
}

impl FrameAllocator for TinyAllocator {
    /// Allocate the frames that have been borrowed from the main allocator.
    fn allocate_frame(&mut self, _count: usize) -> Option<Frame> {
        for frame_option in &mut self.0 {
            if frame_option.is_some() {
                return frame_option.take();
            }
        }
        None
    }

    /// Mark any `None` frames as `Some`, and return.
    fn deallocate_frame(&mut self, frame: Frame) {
        for frame_option in &mut self.0 {
            if frame_option.is_none() {
                *frame_option = Some(frame);
                return;
            }
        }
        panic!("Tiny allocator can hold only 3 frames.");
    }

    fn free_frames(&mut self) -> usize {
        let mut count: usize = 0;
        
        for frame_option in &mut self.0 {
            if frame_option.is_some() {
                count += 1;
            }
        }

        count
    }
} */
