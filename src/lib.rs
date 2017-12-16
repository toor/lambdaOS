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
mod libkernel;
mod task;
mod syscall;

use io::pic::PICS;
use spin::Mutex;
use multiboot2::BootInformation;

#[no_mangle]
pub extern "C" fn kmain(multiboot_information_address: usize) {
    io::vga::buffer::clear_screen();
    println!("Hello World{}", "!");
    
    //Load a multiboot BootInfo structure using the address passed in ebx.
    let boot_info = unsafe { multiboot2::load(multiboot_information_address) };
    
    //Safety stuff.
    enable_nxe_bit();
    enable_write_protect_bit();

    // set up guard page and map the heap pages
    let mut memory_controller = memory::init(boot_info);

    //Clear interrupts
    unsafe { asm!("cli") };
    // initialize our IDT
    println!("Loading IDT.");
    interrupts::init(&mut memory_controller);

    //Init PICS.
    unsafe { PICS.lock().init() };
    
    //Start real interrupts.
    unsafe { asm!("sti") };

    println!("It did not crash!");
    
    use alloc::String;
        
    syscall::create(real_main, String::from("real_main"));
    
    let mut i = 0;

    loop {
        use task::Scheduling;

        syscall::create(process_test, format!("test_process_{}", i));
        println!("Running test process {}", i);

        unsafe {
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
