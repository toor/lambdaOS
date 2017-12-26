#![feature(lang_items)]
#![feature(const_fn, unique)]
#![feature(alloc)]
#![feature(allocator_api)]
#![feature(asm)]
#![feature(naked_functions)]
#![feature(abi_x86_interrupt)]
#![feature(const_unique_new)]
#![feature(const_max_value)]
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
pub mod io;
pub mod interrupts;
pub mod klib;
pub mod task;
pub mod syscall;
mod utils;
mod runtime_glue;

use io::pic::PICS;
use utils::*;
pub use runtime_glue::*;

#[no_mangle]
pub extern "C" fn kmain(multiboot_information_address: usize) {
    io::vga::buffer::clear_screen();
    println!("[ INFO ] lambdaOS: Begin init.");
    
    //Load a multiboot BootInfo structure using the address passed in ebx.
    let boot_info = unsafe { multiboot2::load(multiboot_information_address) };
    
    //Safety stuff.
    enable_nxe_bit();
    enable_write_protect_bit();

    // set up guard page and map the heap pages
    let mut memory_controller = memory::init(boot_info);

    // Interrupts.
    unsafe {
        // Clear all current hardware interrupts.
        disable_interrupts();
        // Load an IDT.
        interrupts::init(&mut memory_controller);
        // Initialise the 8259 PIC.
        PICS.lock().init();
        //Initalise all other hardware devices.
        io::init_devices();
        // Turn on real interrupts.
        enable_interrupts();
    }
    
    let proc_closure = || {
        let max_procs = 50;

        for i in 0..max_procs {
            syscall::create(process_test, format!("test_process_{}", i));
        }
    };

    disable_interrupts_and_then(proc_closure);
    
    loop {}
}

#[no_mangle]
pub extern "C" fn real_main() {
    println!("In real main");
}

pub extern "C" fn process_test() {
    println!("Inside test process."); 
}
