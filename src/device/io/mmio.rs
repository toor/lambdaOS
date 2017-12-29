use core::intrinsics::{volatile_load, volatile_store};
use core::mem::uninitialized;
use core::ops::{BitAnd, BitOr, Not};

#[repr(packed)]
pub struct Mmio<T> {
    value: T,
}

impl<T> Mmio<T>
where T: Copy + PartialEq + BitAnd<Output=T> + BitOr<Output=T> + Not<Output=T>
{
    pub fn new() -> Self {
        Mmio {
            value: unsafe { uninitialized() },
        }
    }

    pub fn read(&self) -> T {
        unsafe { volatile_load(&self.value) } 
    }

    pub fn write(&mut self, value: T) {
        unsafe { volatile_store(&mut self.value, value) }
    }

    pub fn readf(&self, flags: T) -> bool {
        (self.value & flags) as T == flags
    }

    pub fn writef(&mut self, flags: T, value: bool) {
        let tmp: T = match value {
            true => self.read() | flags,
            false => self.read() & !flags,
        };

        self.write(tmp);
    }
}
