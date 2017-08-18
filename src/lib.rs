#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique)]
#![no_std]
#![feature(alloc)]
#![feature(custom_attributes)]
#![feature(allocator_internals)]
#![feature(global_allocator)]
#![default_lib_allocator]
#![feature(abi_x86_interrupt)]
#![feature(asm)]
#![feature(box_syntax)]
#![feature(drop_types_in_const)]

extern crate rlibc;
extern crate volatile;
extern crate spin;
extern crate multiboot2;
#[macro_use]
extern crate bitflags;
extern crate x86_64;
#[macro_use]
extern crate once;
#[macro_use]
extern crate alloc;
extern crate linked_list_allocator;
extern crate hole_list_allocator as allocator;
#[macro_use]
extern crate lazy_static;
extern crate bit_field;
extern crate x86;
extern crate libtoorix;

#[macro_use]
mod macros;

#[macro_use]
mod vga;
mod memory;
mod interrupts;
mod constants;
mod io;
mod state;
mod debug;
mod util;
mod event;
mod task;

static mut MEMORY_SAFE: bool = false;

#[no_mangle]
pub extern "C" fn rust_main(multiboot_information_address: usize) {
    vga::clear_screen();
    println!("Hello World{}", "!");

    let boot_info = unsafe{ multiboot2::load(multiboot_information_address) };

    //Set NXE bit so we can use the NO_EXECUTE flag.
    enable_nxe_bit();
    //Enable write protect so we are no longer able to access the .rodata and etc. sections.
    enable_write_protect_bit();
    
    //Remap kernel and set up a guard page
    let mut memory_controller = memory::init(boot_info);

    state();
    unsafe { MEMORY_SAFE = true; }

    //Heap is working so we can use kprint
    io::kprint::init();

    //Set up the Interrupt Descriptor table
    interrupts::init(&mut memory_controller);

    io::init();


    kprint!("Time is {}", io::timer::real_time());

    kprint!("Hello there.");
    //if you see this message it's all good

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

pub fn state() -> &'static mut state::State {
    state::state()
}

pub fn memory_safe() -> bool {
    unsafe { MEMORY_SAFE } //Static mut so requires unsafe block
}

#[lang = "eh_personality"] extern fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    println!("\n\nPANIC in {} at line {}:", file, line);
    println!("    {}", fmt);
    loop{}
}
