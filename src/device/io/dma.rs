use core::{mem, ptr};
use arch::memory;

// TODO: Add Drop impl.
struct PhysBox {
    address: usize,
    size: usize,
}

impl PhysBox {
    /// Allocate some physical memory and return the start address of the allocated frame.
    fn new(size: usize) -> Result<Self, &'static str> {
        let address = unsafe { memory::physalloc(size)? };
        
        Ok(PhysBox {
            address: address,
            size: size,
        })

    }
}
