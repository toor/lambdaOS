#![feature(lang_items)]
#![feature(const_fn, unique)]
#![feature(alloc)]
#![feature(allocator_api)]
#![feature(asm)]
#![feature(naked_functions)]
#![feature(abi_x86_interrupt)]
#![feature(const_unique_new)]
#![feature(const_max_value)]
#![feature(core_intrinsics)]
#![feature(global_allocator)]
#![feature(ptr_internals)]
#![feature(integer_atomics)]
#![no_std]

#[macro_use]
extern crate alloc;
extern crate bit_field;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate lazy_static;
extern crate linked_list_allocator;
extern crate multiboot2;
#[macro_use]
extern crate once;
extern crate rlibc;
extern crate spin;
extern crate volatile;
extern crate x86_64;
extern crate raw_cpuid;

#[macro_use]
mod macros;
pub mod device;
pub mod task;
pub mod syscall;
pub mod arch;
pub mod acpi;
mod runtime_glue;

pub use runtime_glue::*;

#[no_mangle]
pub extern "C" fn kmain(multiboot_information_address: usize) {
    unsafe { arch::init(multiboot_information_address) };

    loop {}
}

// TODO: Move this to the memory module once some bugs with Rust get figured out.
use arch::memory::heap_allocator::HeapAllocator;

#[global_allocator]
static HEAP_ALLOCATOR: HeapAllocator = HeapAllocator::new();
