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
use task::Scheduling;
use utils::{enable_nxe_bit, enable_write_protect_bit};
pub use runtime_glue::*;
use alloc::String;

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

    //Clear interrupts
    unsafe { asm!("cli") };

    interrupts::init(&mut memory_controller);

    //Init PICS.
    unsafe { PICS.lock().init() };
    
    unsafe { io::init_devices() };

    //Start real interrupts.
    unsafe { asm!("sti") };

    println!("[ OK ] Initialized lambdaOS");
    
    syscall::create(real_main, String::from("real_main"));
    
    let mut i = 0;

    loop {
        syscall::create(process_test, format!("test_process_{}", i));

        unsafe {
            println!("Running a test process.");
            task::SCHEDULER.resched();
            i += 1;
        }
    }
}

#[no_mangle]
pub extern "C" fn real_main() {
    println!("In real main");
}

pub extern "C" fn process_test() {
    println!("Inside test process."); 
}
