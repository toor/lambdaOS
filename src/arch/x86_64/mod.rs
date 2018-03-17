pub mod interrupts;
pub mod memory;
pub mod init;

use self::memory::MemoryController;
pub use self::init::init;

pub static mut MEMORY_CONTROLLER: Option<MemoryController> = None;

pub unsafe fn memory_controller() -> &'static mut MemoryController {
    match MEMORY_CONTROLLER {
        Some(ref mut m) => m,
        None => panic!("Memory controller called before init."),
    }
}
