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
extern crate hole_list_allocator as allocator;
#[macro_use]
extern crate alloc;

#[macro_use]
mod macros;
pub mod memory;
pub mod device;
pub mod interrupts;
pub mod task;
pub mod syscall;
mod utils;
mod runtime_glue;

use device::pic::PICS;
use utils::*;
pub use runtime_glue::*;

#[no_mangle]
pub extern "C" fn kmain(multiboot_information_address: usize) {
    device::vga::buffer::clear_screen();
    println!("[ INFO ] lambdaOS: Begin init.");
    
    //Load a multiboot BootInfo structure using the address passed in ebx.
    let boot_info = unsafe { multiboot2::load(multiboot_information_address) };
    
    //Safety stuff.
    enable_nxe_bit();
    enable_write_protect_bit();

    // set up guard page and map the heap pages
    let mut memory_controller = memory::init(boot_info);

    // Interrupts.
    disable_interrupts_and_then(|| {
        unsafe {
            // Load IDT.
            interrupts::init(&mut memory_controller);
            // Initialise 8259 PIC.
            PICS.lock().init();
            // Initalise all other hardware devices.
            device::init();
        }
    });
    
    /*let proc_closure = || {
        let max_procs = 50;

        for i in 0..max_procs {
            syscall::create(process_test, format!("test_process_{}", i));
        }
    };

    disable_interrupts_and_then(proc_closure);*/
    
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
