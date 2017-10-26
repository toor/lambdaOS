#![feature(lang_items, const_fn, const_unsafe_cell_new, alloc, custom_attributes, global_allocator,
          box_syntax, drop_types_in_const, unique, const_unique_new, allocator_internals,
          abi_x86_interrupts, asm, exclusive_range_pattern)]
#![no_std]
#![default_lib_allocator]
#![allow(safe_extern_statics)]
#![allow(const_err)]

#[macro_use]
extern crate alloc;
extern crate bit_field;
#[macro_use]
extern crate bitflags;
extern crate hole_list_allocator as allocator;
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
extern crate x86;

#[macro_use]
mod macros;

#[macro_use]
mod memory;
mod io;
mod interrupts;
mod vga;

#[allow(non_snake_case)]
#[no_mangle]
pub fn _UnwindResume() {
    loop {}
}

#[no_mangle]
pub extern "C" fn kmain(multiboot_information_address: usize) {
    use vga::{ColorCode, SCREEN};
    use vga::Color::*;
    vga::clear_screen();
    println!("Hello world!");

    let boot_info = unsafe { multiboot2::load(multiboot_information_address) };

    //Set NXE bit so we can use the NO_EXECUTE flag.
    enable_nxe_bit();
    //Enable write protect so we are no longer able to access the .rodata and etc. sections.
    enable_write_protect_bit();

    //Remap kernel and set up a guard page
    let mut memory_controller = memory::init(boot_info);

    //Interrupts.
    interrupts::initialize();
}

fn enable_nxe_bit() {
    use x86_64::registers::msr::{rdmsr, wrmsr, IA32_EFER};

    let nxe_bit = 1 << 11;
    unsafe {
        let efer = rdmsr(IA32_EFER);
        wrmsr(IA32_EFER, efer | nxe_bit);
    }
}

fn enable_write_protect_bit() {
    use x86_64::registers::control_regs::{Cr0, cr0, cr0_write};

    unsafe { cr0_write(cr0() | Cr0::WRITE_PROTECT) };
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    println!("\n\nPANIC in {} at line {}:", file, line);
    println!("    {}", fmt);
    loop {}
}
