#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique)]
#![no_std]
#![feature(alloc)]
#![feature(custom_attributes)]
#![feature(allocator_internals)]
#![default_lib_allocator]

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
extern crate hole_list_allocator as allocator;

#[macro_use]
mod vga;
mod memory;

#[no_mangle]
pub extern "C" fn rust_main(multiboot_information_address: usize) {
    vga::clear_screen();
    println!("Hello World{}", "!");

    let boot_info = unsafe{ multiboot2::load(multiboot_information_address) };

    enable_nxe_bit();
    enable_write_protect_bit();
    
    memory::init(boot_info);

    use alloc::boxed::Box;
    let mut heap_test = Box::new(42);
    *heap_test -= 15;
    let heap_test_2 = Box::new("hello");
    println!("{:?} {:?}", heap_test, heap_test_2);

    let mut vec_test = vec![1,2,3,4,5,6,7];
    vec_test[3] = 42;

    for i in &vec_test {
        println!("{}", i);
    }

    println!("It did not crash!");

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

#[lang = "eh_personality"] extern fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    println!("\n\nPANIC in {} at line {}:", file, line);
    println!("    {}", fmt);
    loop{}
}
