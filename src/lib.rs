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
mod debug;
mod libkernel;
mod task;

use io::pic::ChainedPics;
use spin::Mutex;
use multiboot2::BootInformation;
use libkernel::*;
use core::cell::RefCell;
use alloc::rc::Rc;

//Constants and statics.
pub const HEAP_START: usize = 0o_000_001_000_000_0000;
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB
pub static PICS: Mutex<ChainedPics> = Mutex::new(unsafe { ChainedPics::new(0x20, 0x28) });
pub static mut BOOT_INFO: Option<&BootInformation> = None;

#[no_mangle]
pub extern "C" fn kmain(multiboot_information_address: usize) {
    // ATTENTION: we have a very small stack and no guard page
    io::vga::buffer::clear_screen();
    println!("Hello World{}", "!");
    
    //Load a multiboot BootInfo structure using the address passed in ebx.
    let boot_info = unsafe { multiboot2::load(multiboot_information_address) };
    
    //Set up BOOT_INFO static so it can be used for printing debug info.
    unsafe { BOOT_INFO = Some(boot_info) };
    
    //Safety stuff.
    enable_nxe_bit();
    enable_write_protect_bit();

    // set up guard page and map the heap pages
    let mut memory_controller = memory::init(boot_info);
    let mut memc_lock: Rc<RefCell<memory::MemoryController>> = Rc::new(RefCell::new(memory_controller));

    //Clear interrupts
    unsafe { asm!("cli") };
    // initialize our IDT
    println!("Loading IDT.");
    interrupts::init(&mut memc_lock.borrow_mut());

    //Init PICS.
    unsafe { PICS.lock().init() };
    
    //Start real interrupts.
    unsafe { asm!("sti") };

    println!("It did not crash!");
    
    //Test kalloc
    io::vga::buffer::clear_screen();
    loop {}
}

fn enable_nxe_bit() {
    use x86_64::registers::msr::{IA32_EFER, rdmsr, wrmsr};

    let nxe_bit = 1 << 11;
    unsafe {
        let efer = rdmsr(IA32_EFER);
        wrmsr(IA32_EFER, efer | nxe_bit);
    }
}

fn enable_write_protect_bit() {
    use x86_64::registers::control_regs::{cr0, cr0_write, Cr0};

    unsafe { cr0_write(cr0() | Cr0::WRITE_PROTECT) };
}

#[cfg(not(test))]
#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

#[cfg(not(test))]
#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    println!("\n\nPANIC in {} at line {}:", file, line);
    println!("    {}", fmt);
    loop {}
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    loop {}
}
