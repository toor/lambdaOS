pub mod interrupts;
pub mod memory;

use device;

pub unsafe fn kinit(multiboot_info: usize) {
    interrupts::disable_interrupts();
    {
        device::vga::buffer::clear_screen();

        let boot_info = ::multiboot2::load(multiboot_info);

        enable_nxe_bit();
        enable_write_protect_bit();

        let mut memory_controller = memory::init(&boot_info);

        interrupts::init(&mut memory_controller);

        device::pic::PICS.lock().init();

        device::init();
    }
    interrupts::enable_interrupts();
}

pub fn enable_nxe_bit() {
    use x86_64::registers::msr::{IA32_EFER, rdmsr, wrmsr};

    let nxe_bit = 1 << 11;
    unsafe {
        let efer = rdmsr(IA32_EFER);
        wrmsr(IA32_EFER, efer | nxe_bit);
    }
}

pub fn enable_write_protect_bit() {
    use x86_64::registers::control_regs::{cr0, cr0_write, Cr0};

    unsafe { cr0_write(cr0() | Cr0::WRITE_PROTECT) };
}


