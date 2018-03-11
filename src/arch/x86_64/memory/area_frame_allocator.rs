use arch::memory::{Frame, FrameAllocator};
use multiboot2::{MemoryArea, MemoryAreaIter};
use arch::memory::paging::PhysicalAddress;

/// A frame allocator that uses the memory areas from the multiboot information structure as
/// source. The {kernel, multiboot}_{start, end} fields are used to avoid returning memory that is
/// already in use.
///
/// `kernel_end` and `multiboot_end` are _inclusive_ bounds.
pub struct AreaFrameAllocator {
    /// The next available physical frame.
    next_free_frame: Frame,
    /// The current memory area, detected by multiboot using the e820.
    current_area: Option<&'static MemoryArea>,
    /// An iterator over all memory areas.
    areas: MemoryAreaIter,
    /// The starting frame of the kernel in physical memory.
    /// frame.start_address().get() == 1MiB.
    kernel_start: Frame,
    /// The end frame of the kernel in physical memory.
    kernel_end: Frame,
    /// The starting frame of the multiboot structure in physical memory,
    multiboot_start: Frame,
    /// The end frame of the multiboot data structure in physical memory.
    multiboot_end: Frame,
}

impl AreaFrameAllocator {
    pub fn new(
        kernel_start: usize,
        kernel_end: usize,
        multiboot_start: usize,
        multiboot_end: usize,
        memory_areas: MemoryAreaIter,
    ) -> AreaFrameAllocator {
        let mut allocator = AreaFrameAllocator {
            next_free_frame: Frame::containing_address(PhysicalAddress::new(0)),
            current_area: None,
            areas: memory_areas,
            kernel_start: Frame::containing_address(PhysicalAddress::new(kernel_start)),
            kernel_end: Frame::containing_address(PhysicalAddress::new(kernel_end)),
            multiboot_start: Frame::containing_address(PhysicalAddress::new(multiboot_start)),
            multiboot_end: Frame::containing_address(PhysicalAddress::new(multiboot_end)),
        };
        allocator.choose_next_area();
        allocator
    }

    /// Choose the next available memory area.
    fn choose_next_area(&mut self) {
        self.current_area = self.areas
            .clone()
            .filter(|area| {
                let address = area.start_address() + area.size() - 1;
                Frame::containing_address(PhysicalAddress::new(address as usize))
                    >= self.next_free_frame
            })
            .min_by_key(|area| area.start_address());

        if let Some(area) = self.current_area {
            let start_frame = Frame::containing_address(PhysicalAddress::new(area.start_address()));
            println!(
                "First area starts at address: {:#x}",
                start_frame.start_address().get()
            );
            if self.next_free_frame < start_frame {
                self.next_free_frame = start_frame;
            }
        }
    }
}

impl FrameAllocator for AreaFrameAllocator {
    /// Allocate a single frame. Return `None` if we are out of memory.
    fn allocate_frame(&mut self, count: usize) -> Option<Frame> {
        if count == 0 {
            return None;
        } else if let Some(area) = self.current_area {
            // "clone" the frame to return it if it's free. Frame doesn't
            // implement Clone, but we can construct an identical frame.
            let start_frame = Frame {
                number: self.next_free_frame.number,
            };

            let end_frame = Frame {
                number: self.next_free_frame.number + (count - 1),
            };

            // the last frame of the current area
            let current_area_last_frame = {
                let address = area.start_address() + area.size() - 1;
                Frame::containing_address(PhysicalAddress::new(address))
            };

            if end_frame > current_area_last_frame {
                // all frames of current area are used, switch to next area
                self.choose_next_area();
            } else if (start_frame >= self.kernel_start && start_frame <= self.kernel_end)
                || (end_frame >= self.kernel_start && start_frame <= self.kernel_end)
            {
                // frame range is used by the kernel.
                self.next_free_frame = Frame {
                    number: self.kernel_end.number + 1,
                };
            } else if (start_frame >= self.multiboot_start && start_frame <= self.multiboot_end)
                || (end_frame >= self.multiboot_start && end_frame <= self.multiboot_end)
            {
                // `frame` is used by the multiboot information structure
                self.next_free_frame = Frame {
                    number: self.multiboot_end.number + 1,
                };
            } else {
                // frame is unused, increment `next_free_frame` and return it
                self.next_free_frame.number += 1;
                return Some(start_frame);
            }
            // `frame` was not valid, try it again with the updated `next_free_frame`
            self.allocate_frame(count)
        } else {
            None // no free frames left
        }
    }

    fn deallocate_frame(&mut self, _frame: Frame) {
        unimplemented!()
    }
    
    /// Get a count of available free frames.
    fn free_frames(&mut self) -> usize {
        let mut count = 0;

        for area in self.areas.clone() {
            let start_frame = Frame::containing_address(PhysicalAddress::new(area.start_address()));
            let end_frame = Frame::containing_address(PhysicalAddress::new(
                (area.start_address() + area.size() - 1),
            ));

            for frame in Frame::range_inclusive(start_frame, end_frame) {
                if frame >= self.kernel_start && frame <= self.kernel_end {
                    // Frame is used by the kernel.
                } else if frame >= self.next_free_frame {
                    count += 1;
                } else {
                    // Inside of used range.
                }
            }
        }

        count
    }
}
