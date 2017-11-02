#![feature(lang_items, const_fn, const_unsafe_cell_new, alloc, custom_attributes, global_allocator,
          box_syntax, drop_types_in_const, unique, const_unique_new, allocator_internals,
          abi_x86_interrupts, asm, exclusive_range_pattern, naked_functions, core_intrinsics)]
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
mod vga;
#[macro_use]
mod interrupts;

#[allow(non_snake_case)]
#[no_mangle]
pub fn _UnwindResume() {
    loop {}
}

use spin::Mutex;
static IDT: Mutex<interrupts::Idt> = Mutex::new(interrupts::Idt::new());

#[no_mangle]
pub extern "C" fn kmain(multiboot_information_address: usize) {
    println!("Hello world!");

    let boot_info = unsafe { multiboot2::load(multiboot_information_address) };

    enable_nxe_bit();
    enable_write_protect_bit();

    //Remap kernel and set up a guard page
    let mut memory_controller = memory::init(boot_info);

    use interrupts::PICS;
    
    //Remap the Programmable Interrupt Controllers. (src/io/mod.rs).
    PICS.lock().init();
    
    use x86::bits64::irq::IdtEntry;

    //Interrupt Service 13, 0xD. General Protection Fault. We can't handle this at the moment, so
    //just panic.
    let gpf = make_idt_entry!(isr13, {
        panic!("General Protection Fault!");
    });
    
    //Timer is IRQ 0. Remapped IRQs start at 0x20 = 32. 32+0 = 32.
    let timer = make_idt_entry!(isr32, {
        PICS.lock().notify_end_of_interrupt(0x20);
    });
    
    //32+1 = 33
    let keyboard = make_idt_entry!(isr33, {
        let port = unsafe { io::cpuio::Port::new(0x60 as u16) };
        
        //Read a single code off the port.
        let scancode = port.read();

        if let Some(c) = io::keyboard::scancode_to_ascii(scancode as usize) {
            println!("{}", c);
        }
        
        //outb(0x20, 0x20), outb(0xA0, 0x20) - notify master and slave of EOI.
        PICS.lock().notify_end_of_interrupt(0x21);
    });
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
