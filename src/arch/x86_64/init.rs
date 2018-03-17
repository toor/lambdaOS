use super::interrupts;
use super::memory;
use super::MEMORY_CONTROLLER;
use device;

/// Main kernel init function. This sets everything up for us.
pub unsafe fn init(multiboot_info: usize) {
    interrupts::disable_interrupts();
    {
        device::vga::buffer::clear_screen();
        println!("[ INFO ] lambdaOS: Begin init.");

        let boot_info = ::multiboot2::load(multiboot_info);

        // Set safety bits in certain registers.
        enable_nxe_bit();
        enable_write_protect_bit();

        // Setup memory management.
        let mut memory_controller = memory::init(&boot_info);
        interrupts::init(&mut memory_controller);

        MEMORY_CONTROLLER = Some(memory_controller);

        device::init();
    }

    // set interrupt flag using sti instruction.
    interrupts::enable_interrupts();

    println!("[ OK ] Init successful, you may now type.")
}

pub fn enable_nxe_bit() {
    use x86_64::registers::msr::{rdmsr, wrmsr, IA32_EFER};

    let nxe_bit = 1 << 11;
    unsafe {
        let efer = rdmsr(IA32_EFER);
        wrmsr(IA32_EFER, efer | nxe_bit);
    }
}

pub fn enable_write_protect_bit() {
    use x86_64::registers::control_regs::{Cr0, cr0, cr0_write};

    unsafe { cr0_write(cr0() | Cr0::WRITE_PROTECT) };
}
