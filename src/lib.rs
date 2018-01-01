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
#![no_std]

extern crate rlibc;
extern crate volatile;
extern crate spin;
extern crate multiboot2;
#[macro_use]
extern crate bitflags;
extern crate x86_64;
#[macro_use]
extern crate once;
extern crate bit_field;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate alloc;
extern crate linked_list_allocator;

#[macro_use]
mod macros;
pub mod device;
pub mod task;
pub mod syscall;
pub mod arch;
mod runtime_glue;

use device::pic::PICS;
pub use runtime_glue::*;

#[no_mangle]
pub extern "C" fn kmain(multiboot_information_address: usize) {
    unsafe { arch::kinit(multiboot_information_address) };

    let proc_closure = || {
        let max_procs = 50;

        for i in 0..max_procs {
            syscall::create(process_test, format!("test_process_{}", i));
        }
    };

    proc_closure();
    
    use alloc::String;

    syscall::create(real_main, String::from("real_main"));
    
    loop {}
}

#[no_mangle]
pub extern "C" fn real_main() {
    println!("In real main");
}

pub extern "C" fn process_test() {
    println!("Inside test process."); 
}

use arch::memory::heap_allocator::HeapAllocator;

// Attribute tells Rust to use this as the default heap allocator.
#[global_allocator]
static HEAP_ALLOCATOR: HeapAllocator = HeapAllocator::new();
